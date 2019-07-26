use core::marker::Unpin;
use core::ops::{Generator, GeneratorState};
use core::pin::Pin;
use core::task::{Context, Poll};

pub use core::future::*;

pub fn from_generator<T: Generator<Yield = ()>>(x: T) -> impl Future<Output = T::Return> {
    GenFuture(x)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct GenFuture<T: Generator<Yield = ()>>(T);

impl<T: Generator<Yield = ()>> !Unpin for GenFuture<T> {}

impl<T: Generator<Yield = ()>> Future for GenFuture<T> {
    type Output = T::Return;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Safe because we're !Unpin + !Drop mapping to a ?Unpin value
        let gen = unsafe { Pin::map_unchecked_mut(self, |s| &mut s.0) };
        set_task_context(cx, || match gen.resume() {
            GeneratorState::Yielded(()) => Poll::Pending,
            GeneratorState::Complete(x) => Poll::Ready(x),
        })
    }
}

pub fn set_task_context<F, R>(_: &mut Context<'_>, f: F) -> R
where
    F: FnOnce() -> R,
{
    f()
}

pub fn get_task_context<F, R>(f: F) -> R
where
    F: FnOnce(&mut Context<'_>) -> R,
{
    f(&mut Context::from_waker(&crate::task::stateless_waker()))
}

pub fn poll_with_tls_context<F>(f: Pin<&mut F>) -> Poll<F::Output>
where
    F: Future,
{
    get_task_context(|cx| F::poll(f, cx))
}
