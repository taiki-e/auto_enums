// SPDX-License-Identifier: Apache-2.0 OR MIT

use core::{
    future::Future,
    marker::PhantomPinned,
    pin::Pin,
    task::{Context, Poll},
};

use auto_enums::enum_derive;

#[enum_derive(Future)] //~ ERROR cannot be unpinned
enum Enum1 {
    A(PinnedFuture),
}

struct PinnedFuture(PhantomPinned);

impl Future for PinnedFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Pending
    }
}

fn main() {
    fn is_unpin<T: Unpin>() {}
    is_unpin::<Enum1>();
}
