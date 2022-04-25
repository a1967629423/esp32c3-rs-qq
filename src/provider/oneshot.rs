use futures::Future;
use nrs_qq::{OneShotChannelProvider, TOneShotReceiver, TOneShotSender};
// use smol::{channel::{Sender,Receiver,bounded}};
use futures::channel::oneshot::{channel, Receiver, Sender};

pub struct MyOneShotSender<T>(Sender<T>);

pub struct MyOneShotReceiver<T>(Receiver<T>);

pub struct MyOneShotProvider;

impl<T: Send> Future for MyOneShotReceiver<T> {
    type Output = Result<T, <Self as TOneShotReceiver<T>>::Error>;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        // self.0.poll(cx).map_err(|_|())
        // match self.0.try_recv() {
        //     Ok(o) => {
        //         Poll::Ready(Ok(o))
        //     },
        //     Err(err) => {
        //         match err {
        //             smol::channel::TryRecvError::Empty => {
        //                 cx.waker().wake_by_ref();
        //                 Poll::Pending
        //             },
        //             smol::channel::TryRecvError::Closed => {
        //                 Poll::Ready(Err(()))
        //             }
        //         }
        //     },
        // }
        let res = std::pin::Pin::new(&mut self.0).poll(cx).map_err(|_| ());
        res
    }
}

impl<T: Send> TOneShotReceiver<T> for MyOneShotReceiver<T> {}

impl<T: Send> TOneShotSender<T> for MyOneShotSender<T> {
    type Error = ();

    fn send(self, t: T) -> Result<(), Self::Error> {
        self.0.send(t).map_err(|_| ())
    }
}

impl OneShotChannelProvider for MyOneShotProvider {
    type Sender<T: Send> = MyOneShotSender<T>;

    type Receiver<T: Send> = MyOneShotReceiver<T>;

    fn channel<T: Send>() -> (Self::Sender<T>, Self::Receiver<T>) {
        let (s, r) = channel::<T>();

        (MyOneShotSender(s), MyOneShotReceiver(r))
    }
}
