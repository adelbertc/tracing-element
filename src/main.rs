use futures::stream::{self, StreamExt};
use owning_ref::{OwningHandle, StableAddress};
use std::ops::Deref;
use tracing::span::Entered;
use tracing::{info, info_span, Span};

struct SpanWrapper(Box<Span>);

impl SpanWrapper {
    fn new(span: Span) -> SpanWrapper {
        SpanWrapper(Box::new(span))
    }

    fn span(&self) -> Span {
        (*(*self).0).clone()
    }
}

impl Deref for SpanWrapper {
    type Target = Span;
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

unsafe impl StableAddress for SpanWrapper {}

struct TracedElement<'a, A> {
    a: A,
    span: OwningHandle<SpanWrapper, Box<Entered<'a>>>,
}

impl<'a, A> TracedElement<'a, A> {
    fn new(a: A, span: Span) -> TracedElement<'a, A> {
        let span = OwningHandle::new_with_fn(SpanWrapper::new(span), |span| {
            let entered = unsafe { span.as_ref().unwrap().enter() };
            Box::new(entered)
        });

        TracedElement { a, span: span }
    }

    fn span(&self) -> Span {
        self.span.as_owner().span()
    }
}

impl<'a, A: Clone> TracedElement<'a, A> {
    fn value(&self) -> A {
        self.a.clone()
    }
}

impl<'a, A> Deref for TracedElement<'a, A> {
    type Target = A;

    fn deref(&self) -> &Self::Target {
        &self.a
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let v: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let stream = stream::iter(v)
        .map(|i| TracedElement::new(i, info_span!("my span", value = i)))
        .filter_map(|i| async {
            if *i % 2 == 0 {
                info!("passed");
                Some(i)
            } else {
                info!("skipping");
                None
            }
        })
        .for_each(|i| async move {
            info!("hi {}", i.value());
        });

    stream.await
}
