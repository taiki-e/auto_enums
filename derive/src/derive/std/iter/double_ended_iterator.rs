use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["DoubleEndedIterator"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    #[cfg(feature = "try_trait")]
    const CAPACITY: usize = 2;
    #[cfg(not(feature = "try_trait"))]
    const CAPACITY: usize = 3;

    let root = std_root();
    let iter = quote!(#root::iter);

    let mut impls = data.impl_trait_with_capacity(
        CAPACITY,
        syn::parse2(quote!(#iter::DoubleEndedIterator))?,
        Some(ident_call_site("Item")),
        syn::parse2(quote! {
            trait DoubleEndedIterator: #iter::Iterator {
                #[inline]
                fn next_back(&mut self) -> #root::option::Option<Self::Item>;
            }
        })?,
    )?;

    #[cfg(feature = "try_trait")]
    impls.push_method(syn::parse2(quote! {
        #[inline]
        fn try_rfold<__U, __F, __R>(&mut self, init: __U, f: __F) -> __R
        where
            __F: #root::ops::FnMut(__U, Self::Item) -> __R,
            __R: #root::ops::Try<Ok = __U>;
    })?)?;
    // It is equally efficient if `try_rfold` can be used.
    #[cfg(not(feature = "try_trait"))]
    impls.append_items_from_trait(syn::parse2(quote! {
        trait DoubleEndedIterator {
            #[inline]
            fn rfold<__U, __F>(self, accum: __U, f: __F) -> __U
            where
                __F: #root::ops::FnMut(__U, Self::Item) -> __U;
            #[inline]
            fn rfind<__P>(&mut self, predicate: __P) -> #root::option::Option<Self::Item>
            where
                __P: #root::ops::FnMut(&Self::Item) -> bool;
        }
    })?)?;

    Ok(impls.build())
}
