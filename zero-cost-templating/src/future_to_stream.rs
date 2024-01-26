// this file is duplicated

use core::cell::Cell;
use core::{pin::Pin, task::Poll};

use bytes::Bytes;
use futures_core::{Future, Stream};
use pin_project::pin_project;

pub type T = Bytes;

thread_local! {
    static VALUE: Cell<Option<T>> = const { Cell::new(None) };
}

// Should not be publicly constructable
pub struct FutureToStream(pub ());

impl FutureToStream {
    pub fn _yield(&self, value: T) -> Self {
        VALUE.set(Some(value));
        Self(())
    }
}

impl Future for FutureToStream {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut core::task::Context<'_>) -> Poll<Self::Output> {
        match VALUE.take() {
            Some(value) => {
                VALUE.set(Some(value));
                Poll::Pending
            }
            None => Poll::Ready(()),
        }
    }
}

#[pin_project]
pub struct TheStream<F: Future<Output = ()>> {
    #[pin]
    future: F,
}

impl<F: Future<Output = ()>> TheStream<F> {
    pub fn new(input: impl FnOnce(FutureToStream) -> F) -> Self {
        Self {
            future: input(FutureToStream(())),
        }
    }
}

impl<F: Future<Output = ()>> Stream for TheStream<F> {
    type Item = T;

    fn poll_next(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Option<Self::Item>> {
        let this = self.project();
        let result = this.future.poll(cx);
        match result {
            Poll::Ready(()) => Poll::Ready(None),
            Poll::Pending => {
                if let Some(value) = VALUE.take() {
                    Poll::Ready(Some(value))
                } else {
                    Poll::Pending
                }
            }
        }
    }
}
