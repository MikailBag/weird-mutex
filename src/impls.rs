use crate::WeirdMutex;

use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

impl<T> core::fmt::Debug for WeirdMutex<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("WeirdMutex")
            .field("type", &core::any::type_name::<T>())
            .finish()
    }
}

impl<T: Future> Future for WeirdMutex<T> {
    type Output = T::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.pinned_get_mut().poll(cx)
    }
}

#[cfg(feature = "stream")]
mod stream_impl {
    use super::*;
    use futures_core::stream::Stream;

    impl<T: Stream> Stream for WeirdMutex<T> {
        type Item = T::Item;

        fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            self.pinned_get_mut().poll_next(cx)
        }
    }
}
