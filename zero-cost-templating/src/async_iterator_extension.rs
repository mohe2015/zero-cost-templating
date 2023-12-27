use std::{
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
        Next { s: self }
    }
}

pub struct Next<S: ?Sized> {
    s: S,
}

impl<S: AsyncIterator + Unpin> Future for &mut Next<S> {
    type Output = Option<S::Item>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<S::Item>> {
        Pin::new(&mut self.s).poll_next(cx)
    }
}
