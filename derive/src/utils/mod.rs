use proc_macro2::TokenStream;
use quote::quote;
use smallvec::{Array, SmallVec};

#[macro_use]
mod error;

mod parse;

pub(crate) use self::error::*;
pub(crate) use self::parse::*;

pub(crate) type Stack<T> = SmallVec<[T; 8]>;

pub(crate) fn std_root() -> TokenStream {
    #[cfg(feature = "std")]
    let root = quote!(::std);
    #[cfg(not(feature = "std"))]
    let root = quote!(::core);
    root
}

pub(crate) trait ExtendExt<A>: Extend<A> {
    fn extend_and_return<T: IntoIterator<Item = A>>(mut self, iter: T) -> Self
    where
        Self: Sized,
    {
        self.extend(iter);
        self
    }
}

impl<A, E: Extend<A>> ExtendExt<A> for E {}

pub(crate) trait VecExt<T> {
    fn push_and_return(self, value: T) -> Self;
}

impl<T> VecExt<T> for Vec<T> {
    fn push_and_return(mut self, value: T) -> Self {
        self.push(value);
        self
    }
}

impl<A: Array> VecExt<A::Item> for SmallVec<A> {
    fn push_and_return(mut self, value: A::Item) -> Self {
        self.push(value);
        self
    }
}
