use crate::client::ReadBuf;
use crate::{AsyncRead, AsyncWrite};
use bytes::{Buf, BufMut, BytesMut};
use core::borrow::{Borrow, BorrowMut};
use core::mem::MaybeUninit;
use core::pin::Pin;
use core::task::{Context, Poll};
use futures::stream::FusedStream;
use futures::{Sink, Stream};
use log::debug;
use nrq_engine::{RQError, RQResult};
use tracing::trace;

use super::decoder::Decoder;
use super::encoder::Encoder;
use futures::ready;
use pin_project_lite::pin_project;

pin_project! {
    #[derive(Debug)]
    pub struct FramedImpl<T,U,State> {
        #[pin]
        pub inner:T,
        pub state:State,
        pub codec:U,
    }
}

const INITIAL_CAPACITY: usize = 8 * 1024;
const BACKPRESSURE_BOUNDARY: usize = INITIAL_CAPACITY;

#[derive(Debug)]
pub struct ReadFrame {
    pub eof: bool,
    pub is_readable: bool,
    pub buffer: BytesMut,
    pub has_errored: bool,
}

pub struct WriteFrame {
    pub buffer: BytesMut,
}

#[derive(Default)]
pub struct RWFrames {
    pub read: ReadFrame,
    pub write: WriteFrame,
}

impl Default for ReadFrame {
    fn default() -> Self {
        Self {
            eof: false,
            is_readable: false,
            buffer: BytesMut::with_capacity(INITIAL_CAPACITY),
            has_errored: false,
        }
    }
}

impl Default for WriteFrame {
    fn default() -> Self {
        Self {
            buffer: BytesMut::with_capacity(INITIAL_CAPACITY),
        }
    }
}

impl From<BytesMut> for ReadFrame {
    fn from(mut buffer: BytesMut) -> Self {
        let size = buffer.capacity();
        if size < INITIAL_CAPACITY {
            buffer.reserve(INITIAL_CAPACITY - size);
        }
        Self {
            buffer,
            is_readable: size > 0,
            eof: false,
            has_errored: false,
        }
    }
}

impl From<BytesMut> for WriteFrame {
    fn from(mut buffer: BytesMut) -> Self {
        let size = buffer.capacity();
        if size < INITIAL_CAPACITY {
            buffer.reserve(INITIAL_CAPACITY - size);
        }

        Self { buffer }
    }
}

impl Borrow<ReadFrame> for RWFrames {
    fn borrow(&self) -> &ReadFrame {
        &self.read
    }
}
impl BorrowMut<ReadFrame> for RWFrames {
    fn borrow_mut(&mut self) -> &mut ReadFrame {
        &mut self.read
    }
}
impl Borrow<WriteFrame> for RWFrames {
    fn borrow(&self) -> &WriteFrame {
        &self.write
    }
}
impl BorrowMut<WriteFrame> for RWFrames {
    fn borrow_mut(&mut self) -> &mut WriteFrame {
        &mut self.write
    }
}

impl<T, U, R> Stream for FramedImpl<T, U, R>
where
    T: AsyncRead,
    U: Decoder,
    R: BorrowMut<ReadFrame>,
{
    type Item = Result<U::Item, U::Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut pinned = self.project();
        let state: &mut ReadFrame = pinned.state.borrow_mut();
        loop {
            if state.has_errored {
                // preparing has_errored -> paused
                // tracing::trace!("Returning None and setting paused");
                log::debug!("Returning None and setting paused");
                state.is_readable = false;
                state.has_errored = false;
                return Poll::Ready(None);
            }
            if state.is_readable {
                // pausing or framing
                if state.eof {
                    // pausing
                    let frame = pinned.codec.decode_eof(&mut state.buffer).map_err(|err| {
                        debug!("Got an error, going to errored state");
                        state.has_errored = true;
                        err
                    })?;
                    if frame.is_none() {
                        state.is_readable = false; // prepare pausing -> paused
                    }
                    // implicit pausing -> pausing or pausing -> paused
                    return Poll::Ready(frame.map(Ok));
                }

                // framing
                debug!("attempting to decode a frame");
                if let Some(frame) = pinned.codec.decode(&mut state.buffer).map_err(|op| {
                    debug!("Got an error, going to errored state");
                    state.has_errored = true;
                    op
                })? {
                    debug!("frame decoded from buffer");
                    // implicit framing -> framing
                    return Poll::Ready(Some(Ok(frame)));
                }

                // framing -> reading
                state.is_readable = false;
            }
            // reading or paused
            // If we can't build a frame yet, try to read more data and try again.
            // Make sure we've got room for at least one byte to read to ensure
            // that we don't get a spurious 0 that looks like EOF.
            state.buffer.reserve(1);
            let bytect = match poll_read_buf(pinned.inner.as_mut(), cx, &mut state.buffer).map_err(
                |err| {
                    debug!("Got an error, going to errored state");
                    state.has_errored = true;
                    err
                },
            )? {
                Poll::Ready(ct) => ct,
                // implicit reading -> reading or implicit paused -> paused
                Poll::Pending => return Poll::Pending,
            };
            if bytect == 0 {
                if state.eof {
                    // We're already at an EOF, and since we've reached this path
                    // we're also not readable. This implies that we've already finished
                    // our `decode_eof` handling, so we can simply return `None`.
                    // implicit paused -> paused
                    return Poll::Ready(None);
                }
                // prepare reading -> paused
                state.eof = true;
            } else {
                // prepare paused -> framing or noop reading -> framing
                state.eof = false;
            }

            // paused -> framing or reading -> framing or reading -> pausing
            state.is_readable = true;
        }
    }
}

impl<T, U, R> FusedStream for FramedImpl<T, U, R>
where
    T: AsyncRead,
    U: Decoder,
    R: BorrowMut<ReadFrame>,
{
    fn is_terminated(&self) -> bool {
        // let state: &ReadFrame = self.state.borrow();
        // state.eof || !state.is_readable
        false
    }
}

impl<T, I, U, W> Sink<I> for FramedImpl<T, U, W>
where
    T: AsyncWrite,
    U: Encoder<I>,
    W: BorrowMut<WriteFrame>,
{
    type Error = U::Error;
    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        if self.state.borrow().buffer.len() >= BACKPRESSURE_BOUNDARY {
            self.as_mut().poll_flush(cx)
        } else {
            Poll::Ready(Ok(()))
        }
    }
    fn start_send(self: Pin<&mut Self>, item: I) -> Result<(), Self::Error> {
        let pinned = self.project();
        pinned
            .codec
            .encode(item, &mut pinned.state.borrow_mut().buffer)?;
        Ok(())
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // use crate::util::poll_write_buf;
        // tracing::trace!("flushing framed transport");
        log::debug!("flushing framed transport");
        let mut pinned = self.project();

        while !pinned.state.borrow_mut().buffer.is_empty() {
            let WriteFrame { buffer } = pinned.state.borrow_mut();
            // tracing::trace!("writing; remaining={}", buffer.len());
            log::debug!("writing; remaining={}", buffer.len());
            let n = ready!(poll_write_buf(pinned.inner.as_mut(), cx, buffer))?;

            if n == 0 {
                // return Poll::Ready(Err(io::Error::new(
                //     io::ErrorKind::WriteZero,
                //     "failed to \
                //      write frame to transport",
                // )
                // .into()));
                return Poll::Ready(Err(RQError::IO(
                    "failed to \
                    write frame to transport"
                        .into(),
                )
                .into()));
            }
        }

        // // Try flushing the underlying IO
        ready!(pinned.inner.poll_flush(cx)).map_err(|e| RQError::IO(alloc::format!("{:?}", e)))?;
        // tracing::trace!("framed transport flushed");
        log::debug!("framed transport flushed");
        Poll::Ready(Ok(()))
    }
    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        ready!(self.as_mut().poll_flush(cx))?;
        // TODO: 暂时写不出来bounds，先直接转换
        // ready!(self.project().inner.poll_shutdown(cx))?;
        ready!(self.project().inner.poll_shutdown(cx))
            .map_err(|e| RQError::IO(alloc::format!("{:?}", e).into()))?;
        Poll::Ready(Ok(()))
    }
}

pub fn poll_write_buf<T: AsyncWrite, B: Buf>(
    io: Pin<&mut T>,
    cx: &mut Context<'_>,
    buf: &mut B,
) -> Poll<RQResult<usize>> {
    if !buf.has_remaining() {
        return Poll::Ready(Ok(0));
    }

    let n = ready!(io.poll_write(cx, buf.chunk()))
        .map_err(|e| RQError::IO(alloc::format!("{:?}", e)))?;

    buf.advance(n);

    Poll::Ready(Ok(n))
}

pub fn poll_read_buf<T: AsyncRead, B: BufMut>(
    io: Pin<&mut T>,
    cx: &mut Context<'_>,
    buf: &mut B,
) -> Poll<RQResult<usize>> {
    if !buf.has_remaining_mut() {
        return Poll::Ready(Ok(0));
    }

    let n = {
        let dst = buf.chunk_mut();
        let dst = unsafe { &mut *(dst as *mut _ as *mut [MaybeUninit<u8>]) };
        let mut buf = ReadBuf::uninit(dst);
        let ptr = buf.filled().as_ptr();
        log::debug!("call poll_read");
        ready!(io.poll_read(cx, &mut buf)).map_err(|e| RQError::IO(alloc::format!("{:?}", e)))?;
        log::debug!("poll_read success");
        // Ensure the pointer does not change from under us
        assert_eq!(ptr, buf.filled().as_ptr());
        log::debug!("assert success");
        buf.filled().len()
    };

    // Safety: This is guaranteed to be the number of initialized (and read)
    // bytes due to the invariants provided by `ReadBuf::filled`.
    log::debug!("advance_mut");
    unsafe {
        buf.advance_mut(n);
    }
    log::debug!("poll_read_buf() done");
    Poll::Ready(Ok(n))
}
