use std::mem::MaybeUninit;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use futures::{Future, FutureExt};
use nrs_qq::provider::{TJoinHandle, TaskProvider};
use smol::Task;
use smol::Timer;
// thread_local! {
//     pub static GLOBAL_EXECUTOR:smol::LocalExecutor<'static> = smol::LocalExecutor::new();
// }

pub static GLOBAL_EXECUTOR: smol::Executor<'static> = smol::Executor::new();

pub struct MyJoinHandle<T>(Task<T>);

impl<T: Send> futures::Future for MyJoinHandle<T> {
    type Output = Result<T, <Self as TJoinHandle<T>>::Error>;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        self.0.poll_unpin(cx).map(|v| Ok(v))
    }
}


struct YieldFuture(bool);
impl YieldFuture {
    #[allow(dead_code)]
    pub fn new() -> Self {
        YieldFuture(false)
    }
}
impl futures::Future for YieldFuture {
    type Output = ();

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        log::info!("yield poll()");
        if self.0 {
            std::task::Poll::Ready(())
        } else {
            self.0 = true;
            log::info!("yield waker");
            cx.waker().wake_by_ref();
            log::info!("yield waker ok");
            std::task::Poll::Pending
        }
    }
}

pub struct MyTimer {
    begin: std::time::Instant,
    dur: std::time::Duration,
}
impl MyTimer {
    pub fn after(d: std::time::Duration) -> Self {
        Self {
            begin: std::time::Instant::now(),
            dur: d,
        }
    }
}
impl Future for MyTimer {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let e = self.begin.elapsed();
        if e >= self.dur {
            Poll::Ready(())
        } else {
            // cx.waker().wake_by_ref();
            let waker = cx.waker().clone();
            let dur = self.dur - e;
            match std::thread::Builder::new()
                .name("Timer".to_string())
                .stack_size(1 * 1024)
                .spawn(move || {
                    std::thread::sleep(dur);
                    waker.wake();
                }) {
                Ok(_) => {}
                Err(e) => {
                    log::info!("timer thread create error {:?}", e);
                }
            };
            Poll::Pending
        }
    }
}

impl<T: Send> TJoinHandle<T> for MyJoinHandle<T> {
    fn detach(self) -> () {
        self.0.detach();
    }
}

pub struct MyTaskProvider;

impl TaskProvider for MyTaskProvider {
    type YieldFuture = impl Future<Output = ()> + Send;

    type SleepFuture = impl Future<Output = ()> + Send;

    type TimeoutFuture<T: futures::Future + Send> =
        impl Future<Output = Result<T::Output, Self::TimeoutError>> + Send;

    type SpawnJoinHandle<T: Send> = MyJoinHandle<T>;

    fn yield_now() -> Self::YieldFuture {
        MyTimer::after(std::time::Duration::from_millis(30))
    }

    fn sleep(duration: core::time::Duration) -> Self::SleepFuture {
        async move {
            MyTimer::after(duration).await;
        }
    }

    fn timeout<T: futures::Future + Send>(
        duration: core::time::Duration,
        future: T,
    ) -> Self::TimeoutFuture<T> {
        let mut timeout_fu = Box::pin(async move {
            MyTimer::after(duration).await;
        })
        .fuse();
        let mut fu = Box::pin(async move { future.await }).fuse();
        Box::pin(async move {
            futures::select_biased! {
                _ = timeout_fu => {
                    Err(())
                },
                res = fu => {
                    Ok(res)
                }
            }
        })
    }

    fn spawn<T>(future: T) -> Self::SpawnJoinHandle<T::Output>
    where
        T: futures::Future + Send + 'static,
        T::Output: Send + 'static,
    {
        MyJoinHandle(GLOBAL_EXECUTOR.spawn(future))
    }
}
