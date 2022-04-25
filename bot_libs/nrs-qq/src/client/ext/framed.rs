use crate::{AsyncRead, AsyncWrite};

use super::{
    decoder::Decoder,
    encoder::Encoder,
    framed_impl::{FramedImpl, RWFrames},
    fuse::FusedAlways,
};
use bytes::BytesMut;
use core::pin::Pin;
use core::task::{Context, Poll};
use futures::{stream::FusedStream, Sink, Stream};
use pin_project_lite::pin_project;
pin_project! {
    pub struct Framed<T,U> {
        #[pin]
        inner:FramedImpl<T,U,RWFrames>
    }
}

impl<T, U> FusedAlways for Framed<T, U> {}
impl<T, U> FusedStream for Framed<T, U>
where
    T: AsyncRead + AsyncWrite,
    U: Decoder,
{
    fn is_terminated(&self) -> bool {
        self.inner.is_terminated()
    }
}
impl<T, U> Framed<T, U>
where
    T: AsyncRead + AsyncWrite,
{
    pub fn new(inner: T, codec: U) -> Self {
        // Self {
        //     inner:FramedImpl { inner: (), state: (), codec: () }
        // }
        Framed {
            inner: FramedImpl {
                inner,
                codec,
                state: Default::default(),
            },
        }
    }
}

impl<T, U> Framed<T, U> {
    pub fn from_parts(parts: FramedParts<T, U>) -> Framed<T, U> {
        Framed {
            inner: FramedImpl {
                inner: parts.io,
                codec: parts.codec,
                state: RWFrames {
                    read: parts.read_buf.into(),
                    write: parts.write_buf.into(),
                },
            },
        }
    }

    pub fn get_ref(&self) -> &T {
        &self.inner.inner
    }
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner.inner
    }

    pub fn get_pin_mut(self: Pin<&mut Self>) -> Pin<&mut T> {
        self.project().inner.project().inner
    }

    pub fn codec(&self) -> &U {
        &self.inner.codec
    }

    pub fn codec_mut(&mut self) -> &mut U {
        &mut self.inner.codec
    }

    pub fn map_codec<C, F>(self, map: F) -> Framed<T, C>
    where
        F: FnOnce(U) -> C,
    {
        // This could be potentially simplified once rust-lang/rust#86555 hits stable
        let parts = self.into_parts();
        Framed::from_parts(FramedParts {
            io: parts.io,
            codec: map(parts.codec),
            read_buf: parts.read_buf,
            write_buf: parts.write_buf,
            _priv: (),
        })
    }
    /// Returns a mutable reference to the underlying codec wrapped by
    /// `Framed`.
    ///
    /// Note that care should be taken to not tamper with the underlying codec
    /// as it may corrupt the stream of frames otherwise being worked with.
    pub fn codec_pin_mut(self: Pin<&mut Self>) -> &mut U {
        self.project().inner.project().codec
    }

    /// Returns a reference to the read buffer.
    pub fn read_buffer(&self) -> &BytesMut {
        &self.inner.state.read.buffer
    }

    /// Returns a mutable reference to the read buffer.
    pub fn read_buffer_mut(&mut self) -> &mut BytesMut {
        &mut self.inner.state.read.buffer
    }

    /// Returns a reference to the write buffer.
    pub fn write_buffer(&self) -> &BytesMut {
        &self.inner.state.write.buffer
    }

    /// Returns a mutable reference to the write buffer.
    pub fn write_buffer_mut(&mut self) -> &mut BytesMut {
        &mut self.inner.state.write.buffer
    }
    pub fn into_inner(self) -> T {
        self.inner.inner
    }
    pub fn into_parts(self) -> FramedParts<T, U> {
        FramedParts {
            io: self.inner.inner,
            codec: self.inner.codec,
            read_buf: self.inner.state.read.buffer,
            write_buf: self.inner.state.write.buffer,
            _priv: (),
        }
    }
}

// This impl just defers to the underlying FramedImpl
impl<T, U> Stream for Framed<T, U>
where
    T: AsyncRead,
    U: Decoder,
{
    type Item = Result<U::Item, U::Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.project().inner.poll_next(cx)
    }
}

// This impl just defers to the underlying FramedImpl
impl<T, I, U> Sink<I> for Framed<T, U>
where
    T: AsyncWrite,
    U: Encoder<I>,
{
    type Error = U::Error;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().inner.poll_ready(cx)
    }

    fn start_send(self: Pin<&mut Self>, item: I) -> Result<(), Self::Error> {
        self.project().inner.start_send(item)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().inner.poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.project().inner.poll_close(cx)
    }
}

impl<T, U> core::fmt::Debug for Framed<T, U>
where
    T: core::fmt::Debug,
    U: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Framed")
            .field("io", self.get_ref())
            .field("codec", self.codec())
            .finish()
    }
}

#[derive(Debug)]
#[allow(clippy::manual_non_exhaustive)]
pub struct FramedParts<T, U> {
    /// The inner transport used to read bytes to and write bytes to
    pub io: T,

    /// The codec
    pub codec: U,

    /// The buffer with read but unprocessed data.
    pub read_buf: BytesMut,

    /// A buffer with unprocessed data which are not written yet.
    pub write_buf: BytesMut,

    /// This private field allows us to add additional fields in the future in a
    /// backwards compatible way.
    _priv: (),
}

impl<T, U> FramedParts<T, U> {
    /// Create a new, default, `FramedParts`
    pub fn new<I>(io: T, codec: U) -> FramedParts<T, U>
    where
        U: Encoder<I>,
    {
        FramedParts {
            io,
            codec,
            read_buf: BytesMut::new(),
            write_buf: BytesMut::new(),
            _priv: (),
        }
    }
}
