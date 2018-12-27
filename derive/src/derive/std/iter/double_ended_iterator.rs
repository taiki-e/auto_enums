use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["DoubleEndedIterator"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
    let iter = quote!(#root::iter);

    #[cfg(feature = "try_trait")]
    let try_trait = quote! {
        #[inline]
        fn try_rfold<__U, __F, __R>(&mut self, init: __U, f: __F) -> __R
        where
            __F: #root::ops::FnMut(__U, Self::Item) -> __R,
            __R: #root::ops::Try<Ok = __U>;
    };
    // It is equally efficient if `try_rfold` can be used.
    #[cfg(not(feature = "try_trait"))]
    let try_trait = quote! {
        #[inline]
        fn rfold<__U, __F>(self, accum: __U, f: __F) -> __U
        where
            __F: #root::ops::FnMut(__U, Self::Item) -> __U;
        #[inline]
        fn rfind<__P>(&mut self, predicate: __P) -> #root::option::Option<Self::Item>
        where
            __P: #root::ops::FnMut(&Self::Item) -> bool;
    };

    derive_trait!(
        data,
        Some(ident_call_site("Item")),
        parse_quote!(#iter::DoubleEndedIterator)?,
        parse_quote! {
            trait DoubleEndedIterator: #iter::Iterator {
                #[inline]
                fn next_back(&mut self) -> #root::option::Option<Self::Item>;
                #try_trait
            }
        }?,
    )
}
