use core::{
    async_iter::AsyncIterator,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

pub trait AsyncIterExt {
    fn next(self) -> Next<Self>;
}

impl<T> AsyncIterExt for T {
    fn next(self) -> Next<Self> {
        Next {
            async_iterator: self,
        }
    }
}

pub struct Next<S: ?Sized> {
    async_iterator: S,
}

impl<S: AsyncIterator + Unpin> Future for &mut Next<S> {
    type Output = Option<S::Item>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<S::Item>> {
        Pin::new(&mut self.async_iterator).poll_next(cx)
    }
}
