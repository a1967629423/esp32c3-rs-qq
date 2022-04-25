use futures::{AsyncRead as FuAsyncRead, AsyncWrite as FuAsyncWrite, Future};
// use no_std_net::ToSocketAddrs;
use nrs_qq::client::ReadBuf;
use nrs_qq::provider::{AsyncRead, AsyncWrite, TcpStreamProvider};
use smol::Async;
use std::io::{self, Read, Write};
use std::mem::MaybeUninit;
use std::net::TcpStream;
use std::pin::Pin;
use std::sync::{Arc, RwLock};
use std::task::{Context, Poll};
use std::thread;
use std::time::Duration;

use crate::provider::task::MyTimer;
pub struct MyTcpStreamProvider(Async<TcpStream>);

enum IOStatus {
    None,
    Pending,
}
pub struct MyTcpStreamSyncProvider {
    tcp: TcpStream,
    read: Arc<RwLock<IOStatus>>,
}
impl AsyncRead for MyTcpStreamSyncProvider {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<Result<(), Self::IOError>> {
        // let read = *self.read.read().unwrap();
        // match read {
        //     IOStatus::None => {
        //         let bytes = unsafe {
        //             &mut *(buf.unfilled_mut() as *mut [MaybeUninit<u8>] as *mut [u8])
        //         };
        //         let w = cx.waker().clone();
        //         // let mut tcp = self.tcp.try_clone().unwrap();
        //         let pin_bytes = unsafe {
        //             Pin::new_unchecked(bytes)
        //         };
        //         let r = self.read.clone();
        //         // thread::Builder::new().stack_size(4*1024).spawn(move || {
        //         //     match tcp.read(pin_bytes.get_mut()) {
        //         //         Ok(c) => {
        //         //             *r.write().unwrap() = IOStatus::Success(c);
        //         //             w.wake();
        //         //         },
        //         //         Err(_) => {
        //         //             w.wake();
        //         //             //self.read = IOStatus::Failed;
        //         //             *r.write().unwrap() = IOStatus::Failed;
        //         //         },
        //         //     }
        //         // }).ok();
        //         match self.0.read(pin_bytes.get_mut()) {
        //             Ok(c) => {
        //                 *r.write().unwrap() = IOStatus::Success(c);
        //                 Poll::Ready(Ok(()))
        //             },
        //             Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
        //                 //Poll::Ready(Err(()))
        //                 cx.waker().wake_by_ref();
        //                 Poll::Pending
        //             },
        //             Err(_) => {
        //                 Poll::Ready(Err(()))
        //             },
        //         }
        //         *self.read.write().unwrap() = IOStatus::Pending;
        //         return Poll::Pending;
        //     },
        //     IOStatus::Pending => {
        //         return  Poll::Pending;
        //     },
        //     IOStatus::Success(s) => {
        //         unsafe {
        //             buf.assume_init(s);
        //             buf.advance(s);
        //         }
        //         *self.read.write().unwrap() = IOStatus::None;
        //         return Poll::Ready(Ok(()))
        //     },
        //     IOStatus::Failed => {
        //         *self.read.write().unwrap() = IOStatus::None;
        //         return Poll::Ready(Err(()))
        //     }
        // }
        let bytes = unsafe { &mut *(buf.unfilled_mut() as *mut [MaybeUninit<u8>] as *mut [u8]) };
        // let w = cx.waker().clone();
        // let mut tcp = self.tcp.try_clone().unwrap();
        let pin_bytes = unsafe { Pin::new_unchecked(bytes) };
        // thread::Builder::new().stack_size(4*1024).spawn(move || {
        //     match tcp.read(pin_bytes.get_mut()) {
        //         Ok(c) => {
        //             *r.write().unwrap() = IOStatus::Success(c);
        //             w.wake();
        //         },
        //         Err(_) => {
        //             w.wake();
        //             //self.read = IOStatus::Failed;
        //             *r.write().unwrap() = IOStatus::Failed;
        //         },
        //     }
        // }).ok();
        match self.read.read() {
            Ok(s) => match *s {
                IOStatus::Pending => return Poll::Pending,

                _ => {}
            },
            Err(e) => {
                log::info!("read error {:?}", e);
            }
        }
        match self.tcp.read(pin_bytes.get_mut()) {
            Ok(c) => {
                unsafe {
                    buf.assume_init(c);
                    buf.advance(c);
                }
                log::debug!("read ok");
                Poll::Ready(Ok(()))
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                //Poll::Ready(Err(()))
                let waker = cx.waker().clone();
                let mut g = match self.read.write() {
                    Ok(g) => g,
                    Err(e) => {
                        log::info!("write read error {:?}", e);
                        return Poll::Pending;
                    }
                };
                *g = IOStatus::Pending;
                let r = self.read.clone();
                match thread::Builder::new()
                    .name("Tcp-Read".to_string())
                    .stack_size(4 * 1024)
                    .spawn(move || {
                        thread::sleep(Duration::from_millis(200));
                        waker.wake();
                        let mut g = match r.write() {
                            Ok(g) => g,
                            Err(e) => {
                                log::info!("write read error {:?}", e);
                                return;
                            }
                        };
                        *g = IOStatus::None;
                    }) {
                    Ok(_) => {}
                    Err(e) => {
                        log::info!("thread create error {:?}", e);
                    }
                };
                Poll::Pending
            }
            Err(e) => {
                log::info!("tcp error {:?}",e);
                Poll::Ready(Err(()))
            },
        }
    }
}
impl AsyncWrite for MyTcpStreamSyncProvider {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, Self::IOError>> {
        // Pin::new(&mut self.0).poll_write(cx,buf).map_err(|_|())
        // let write = self.write
        // let write = *self.write.read().unwrap();
        // match write {
        //     IOStatus::None => {
        //         let w = self.write.clone();
        //         let mut tcp = self.tcp.try_clone().unwrap();
        //         let new_buf = unsafe {
        //             &*(buf as *const [u8])
        //         };
        //         let pin_bytes = Pin::new(new_buf);
        //         let waker = cx.waker().clone();
        //         thread::Builder::new().stack_size(4*1024).spawn(move || {
        //              match tcp.write(pin_bytes.get_ref()) {
        //                  Ok(size) => {
        //                     waker.wake();
        //                     *w.write().unwrap() = IOStatus::Success(size);

        //                  },
        //                  Err(_) => {
        //                     waker.wake();
        //                      *w.write().unwrap() = IOStatus::Failed;
        //                  },
        //              }
        //         }).ok();
        //         *self.write.write().unwrap() = IOStatus::Pending;
        //         return Poll::Pending;
        //     }
        //     IOStatus::Pending => {
        //         return Poll::Pending;
        //     }
        //     IOStatus::Success(w) => {
        //         *self.write.write().unwrap() = IOStatus::None;
        //         return Poll::Ready(Ok(w));
        //     }
        //     IOStatus::Failed => {
        //         *self.write.write().unwrap() = IOStatus::None;
        //         return Poll::Ready(Err(()))
        //     }
        // }
        //let new_buf = unsafe { &*(buf as *const [u8]) };
        log::debug!("tcp write");
        let pin_bytes = Pin::new(buf);
        match self.tcp.write(pin_bytes.get_ref()) {
            Ok(size) => {
                return Poll::Ready(Ok(size));
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                let waker = cx.waker().clone();
                match thread::Builder::new()
                    .name("Tcp-Write".to_string())
                    .stack_size(1 * 1024)
                    .spawn(move || {
                        thread::sleep(Duration::from_millis(100));
                        waker.wake();
                    }) {
                    Ok(_) => {}
                    Err(e) => {
                        log::info!("tcp-write error {:?}", e);
                    }
                };
                Poll::Pending
            }
            Err(_) => {
                return Poll::Ready(Err(()));
            }
        }
    }
    fn poll_flush(
        mut self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::IOError>> {
        let res = Poll::Ready(self.tcp.flush().map_err(|_| ()));
        thread::sleep(std::time::Duration::from_millis(500));
        res
    }
    fn poll_shutdown(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::IOError>> {
        // self.tcp.shutdown(how)

        Poll::Ready(self.tcp.shutdown(std::net::Shutdown::Both).map_err(|_|()))
    }
}
impl TcpStreamProvider for MyTcpStreamSyncProvider {
    type ConnectFuture =
        impl Future<Output = Result<Self, <Self as TcpStreamProvider>::IOError>> + Send;
    fn connect<A: no_std_net::ToSocketAddrs>(addr: A) -> Self::ConnectFuture {
        let addrs = addrs_conver(addr).into_iter().next().unwrap();
        let source = TcpStream::connect(addrs).unwrap();
        source.set_nonblocking(true).unwrap();
        futures::future::ready(Ok(MyTcpStreamSyncProvider {
            tcp: source,
            read: Arc::new(RwLock::new(IOStatus::None)),
        }))
    }
}
// impl MyTcpStreamProvider {
//     pub async fn new<T:ToSocketAddrs>(addr:T) -> Self {
//         let addres = addrs_conver(addr)[0];
//         let t = smol::net::TcpStream::connect(addres).await.unwrap();
//         todo!()
//     }
// }
impl AsyncRead for MyTcpStreamProvider {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<Result<(), Self::IOError>> {
        let bytes = unsafe { &mut *(buf.unfilled_mut() as *mut [MaybeUninit<u8>] as *mut [u8]) };
        match Pin::new(&mut self.0).poll_read(cx, bytes) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(res) => {
                if let Ok(res) = &res {
                    unsafe {
                        buf.assume_init(*res);
                        buf.advance(*res);
                    }
                    Poll::Ready(Ok(()))
                } else {
                    Poll::Ready(Err(()))
                }
            }
        }
    }
}
impl AsyncWrite for MyTcpStreamProvider {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, Self::IOError>> {
        Pin::new(&mut self.0).poll_write(cx, buf).map_err(|_| ())
    }
    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::IOError>> {
        Pin::new(&mut self.0).poll_flush(cx).map_err(|_| ())
    }
    fn poll_shutdown(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::IOError>> {
        Pin::new(&mut self.0).poll_close(cx).map_err(|_| ())
    }
}
pub fn addrs_conver(n: impl no_std_net::ToSocketAddrs) -> Vec<std::net::SocketAddr> {
    n.to_socket_addrs()
        .unwrap()
        .map(|socket| {
            let s: std::net::SocketAddr = socket.to_string().parse().unwrap();
            s
        })
        .collect::<Vec<_>>()
}
impl TcpStreamProvider for MyTcpStreamProvider {
    type ConnectFuture = Pin<
        Box<
            dyn futures::Future<Output = Result<Self, <Self as TcpStreamProvider>::IOError>>
                + Send
                + 'static,
        >,
    >;
    fn connect<A: no_std_net::ToSocketAddrs>(addr: A) -> Self::ConnectFuture {
        let addrs = addrs_conver(addr).into_iter().next().unwrap();
        Box::pin(async move {
            let source = Async::<TcpStream>::connect(addrs).await.unwrap();
            let my = MyTcpStreamProvider(source);
            Ok(my)
        })
    }
}
