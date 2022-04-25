use core::pin::Pin;
use core::task::{Context, Poll};
use futures::stream::SplitStream;

pub struct FusedSplitStream<S>(pub SplitStream<S>);
impl<S> Unpin for FusedSplitStream<S> {}
pub trait FusedAlways {}
impl<S> core::ops::Deref for FusedSplitStream<S> {
    type Target = SplitStream<S>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<S> core::ops::DerefMut for FusedSplitStream<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<S: futures::Stream> futures::Stream for FusedSplitStream<S> {
    type Item = S::Item;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<S::Item>> {
        // ready!(self.0.poll_lock(cx)).as_pin_mut().poll_next(cx)
        Pin::new(&mut self.0).poll_next(cx)
    }
}

impl<S> futures::stream::FusedStream for FusedSplitStream<S>
where
    S: futures::Stream,
{
    fn is_terminated(&self) -> bool {
        false
    }
}
