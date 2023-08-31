use async_std::stream::Stream;
use async_std::task::{Context, Poll};
use pin_project_lite::pin_project;
use std::pin::Pin;

pub(super) trait StreamExt: Stream {
    fn dedup(self) -> Dedup<Self>
    where
        Self: Sized,
        Self::Item: Clone,
        Self::Item: PartialEq<Self::Item>,
    {
        Dedup {
            stream: self,
            current: None,
        }
    }
}

impl<T: Stream + ?Sized> StreamExt for T {}

pin_project! {
    pub struct Dedup<S: Stream> {
        #[pin]
        stream: S,
        current: Option<<S as Stream>::Item>,
    }
}

impl<S> Stream for Dedup<S>
where
    S: Stream,
    S::Item: Clone,
    S::Item: PartialEq<S::Item>,
{
    type Item = S::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        let next = futures_core::ready!(this.stream.poll_next(cx));

        match next {
            Some(v) if this.current.as_ref() != Some(&v) => {
                *this.current = Some(v.clone());
                Poll::Ready(Some(v))
            }
            Some(_) => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            None => Poll::Ready(None),
        }
    }
}
