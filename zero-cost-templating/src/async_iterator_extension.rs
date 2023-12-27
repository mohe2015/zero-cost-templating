use core::{
    async_iter::AsyncIterator,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use pin_project::pin_project;

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

#[pin_project]
pub struct AsyncIteratorStream<Item, AI: AsyncIterator<Item = Item>>(#[pin] AI);

impl<Item, AI: AsyncIterator<Item = Item>> futures_core::stream::Stream
    for AsyncIteratorStream<Item, AI>
{
    type Item = Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        this.0.poll_next(cx)
    }
}
