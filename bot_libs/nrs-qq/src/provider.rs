use core::fmt::Debug;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

use nrq_engine::RQError;

use crate::client::ReadBuf;

pub trait TSender<T: Clone + Send>: Clone + Send + Sync {
    type Error: Debug + Send = ();
    type Receiver: TReceiver<T>;
    type SenderFuture<'a>: Future<Output = Result<usize, Self::Error>> + Send + 'a
    where
        Self: 'a;
    fn send(&self, value: T) -> Self::SenderFuture<'_>;
    fn is_closed(&self) -> bool;
    fn close_channel(&self) -> ();
    fn subscribe(&self) -> Self::Receiver;
}
pub trait TReceiver<T>: Send + Sync
where
    T: Send + Clone,
{
    type Error: Debug + Send = ();
    type RecvFuture<'a>: Future<Output = Result<T, Self::Error>>
        + futures::future::FusedFuture
        + Send
        + 'a
    where
        Self: 'a;
    // fn close(&self) -> ();
    fn recv(&mut self) -> Self::RecvFuture<'_>;
}
pub trait TOneShotSender<T>: Send + Sync {
    type Error: Debug + Send = ();
    fn send(self, t: T) -> Result<(), Self::Error>;
}
pub trait TOneShotReceiver<T>: Send + Future<Output = Result<T, Self::Error>> {
    type Error: Debug + Send = ();
}
pub trait ChannelProvider: Sync {
    type Sender<T: 'static + Clone + Send + Sync>: TSender<T>;
    type Receiver<T: 'static + Clone + Send + Sync>: TReceiver<T>;
    fn channel<T: 'static + Clone + Send + Sync>(
        buff: usize,
    ) -> (Self::Sender<T>, Self::Receiver<T>);
}
pub trait OneShotChannelProvider: Sync {
    type Sender<T: Send>: TOneShotSender<T>;
    type Receiver<T: Send>: TOneShotReceiver<T>;
    fn channel<T: Send>() -> (Self::Sender<T>, Self::Receiver<T>);
}
pub trait TRwLockReadGuard<'a, T: ?Sized>: core::ops::Deref<Target = T> {}
pub trait TRwLockWriteGuard<'a, T: ?Sized>:
    core::ops::Deref<Target = T> + core::ops::DerefMut
{
}
pub trait TRwLock<T: ?Sized>: Send + Sync {
    type ReadGuard<'a>: TRwLockReadGuard<'a, T> + Send
    where
        Self: 'a;
    type WriteGuard<'a>: TRwLockWriteGuard<'a, T> + Send
    where
        Self: 'a;
    type ReadFuture<'a>: Future<Output = Self::ReadGuard<'a>> + Send
    where
        Self: 'a;
    type WriteFuture<'a>: Future<Output = Self::WriteGuard<'a>> + Send
    where
        Self: 'a;
    fn new(value: T) -> Self;
    fn read(&self) -> Self::ReadFuture<'_>;
    fn write(&self) -> Self::WriteFuture<'_>;
}
pub trait RwLockProvider: Sync {
    type RwLock<T>: TRwLock<T>
    where
        T: Send + Sync;
}

pub trait TJoinHandle<T>: Send + Sync + Unpin + Future<Output = Result<T, Self::Error>> {
    type Error = ();
    fn detach(self) -> ();
}

pub trait TaskProvider: Sync {
    type YieldFuture: Future<Output = ()> + Send;
    type SleepFuture: Future<Output = ()> + Send;
    type TimeoutError: Debug + Send = ();
    type TimeoutFuture<T: Future + Send>: Future<Output = Result<T::Output, Self::TimeoutError>>
        + Send;
    type SpawnJoinHandle<T: Send>: TJoinHandle<T>;
    fn yield_now() -> Self::YieldFuture;
    fn sleep(duration: core::time::Duration) -> Self::SleepFuture;
    fn timeout<T: Future + Send>(
        duration: core::time::Duration,
        future: T,
    ) -> Self::TimeoutFuture<T>;
    fn spawn<T>(future: T) -> Self::SpawnJoinHandle<T::Output>
    where
        T: Future + Send + 'static,
        T::Output: Send + 'static;
}
pub trait AsyncRead {
    type IOError: Debug = ();
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<Result<(), Self::IOError>>;
}
pub trait AsyncWrite {
    type IOError: Debug + From<RQError> = ();
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, Self::IOError>>;
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::IOError>>;
    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>)
        -> Poll<Result<(), Self::IOError>>;
}

macro_rules! deref_async_write {
    () => {
        fn poll_write(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &[u8],
        ) -> Poll<Result<usize, Self::IOError>> {
            Pin::new(&mut **self).poll_write(cx, buf)
        }

        fn poll_flush(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::IOError>> {
            Pin::new(&mut **self).poll_flush(cx)
        }

        fn poll_shutdown(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::IOError>> {
            Pin::new(&mut **self).poll_shutdown(cx)
        }
    };
}

impl<T: ?Sized + AsyncWrite + Unpin> AsyncWrite for alloc::boxed::Box<T> {
    type IOError = T::IOError;
    deref_async_write!();
}

impl<T: ?Sized + AsyncWrite + Unpin> AsyncWrite for &mut T {
    type IOError = T::IOError;
    deref_async_write!();
}

impl AsyncWrite for alloc::vec::Vec<u8> {
    type IOError = ();
    fn poll_write(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, Self::IOError>> {
        self.get_mut().extend_from_slice(buf);
        Poll::Ready(Ok(buf.len()))
    }
    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::IOError>> {
        Poll::Ready(Ok(()))
    }
    fn poll_shutdown(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::IOError>> {
        Poll::Ready(Ok(()))
    }
}

impl<P> AsyncWrite for Pin<P>
where
    P: core::ops::DerefMut + Unpin,
    P::Target: AsyncWrite,
{
    type IOError = <<P as core::ops::Deref>::Target as AsyncWrite>::IOError;
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, Self::IOError>> {
        self.get_mut().as_mut().poll_write(cx, buf)
    }
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::IOError>> {
        self.get_mut().as_mut().poll_flush(cx)
    }
    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::IOError>> {
        self.get_mut().as_mut().poll_shutdown(cx)
    }
}

pub trait TcpStreamProvider: Sized + AsyncRead + AsyncWrite + Sync + Send {
    type IOError: Debug + Send = ();
    type ConnectFuture: Future<Output = Result<Self, <Self as TcpStreamProvider>::IOError>> + Send;
    fn connect<A: no_std_net::ToSocketAddrs>(addr: A) -> Self::ConnectFuture;
}

pub trait TMutexGuard<'a, T: ?Sized>: core::ops::Deref<Target = T> + core::ops::DerefMut {}
pub trait TMutex<T>: Send + Sync {
    type Guard<'a>: TMutexGuard<'a, T> + Send
    where
        Self: 'a;
    type Future<'a>: Future<Output = Self::Guard<'a>> + Send
    where
        Self: 'a;
    fn new(value: T) -> Self;
    fn lock(&self) -> Self::Future<'_>;
}

pub trait MutexProvider: Sync {
    type Mutex<T: Send + Sync>: TMutex<T>;
}

// 这里用来测试实现是否好用
// struct MyGuard<T:?Sized>(T);
// impl<T:?Sized> core::ops::Deref for MyGuard<T> {
//     type Target = T;
//     fn deref(&self) -> &Self::Target {
//         todo!()
//     }
// }
// impl<T:?Sized> core::ops::DerefMut for MyGuard<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         todo!()
//     }
// }
// impl<'a,T:?Sized> TMutexGuard<'a,T> for MyGuard<T> {

// }

// struct MyMutex;
// impl<T> TMutex<T> for MyMutex {

//     type Guard<'a> where Self:'a=MyGuard<T> ;
//     type Future<'a> where Self:'a = futures::future::Ready<Self::Guard<'a>>;
//     fn new(value:T) -> Self {
//         todo!()
//     }
//     fn lock(&self) -> Self::Future<'_> {
//         todo!()
//     }
// }

// struct Test;
// impl MutexProvider for Test {
//     type Mutex<T> = MyMutex;
// }
