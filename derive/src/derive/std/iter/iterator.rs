use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Iterator"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    #[cfg(feature = "try_trait")]
    const CAPACITY: usize = 9;
    #[cfg(not(feature = "try_trait"))]
    const CAPACITY: usize = 14;

    let root = std_root();
    let iter = quote!(#root::iter);

    let mut impls = data.impl_trait_with_capacity(
        CAPACITY,
        syn::parse2(quote!(#iter::Iterator))?,
        None,
        syn::parse2(quote! {
            trait Iterator {
                type Item;
                #[inline]
                fn next(&mut self) -> #root::option::Option<Self::Item>;
                #[inline]
                fn size_hint(&self) -> (usize, #root::option::Option<usize>);
                #[inline]
                fn count(self) -> usize;
                #[inline]
                fn last(self) -> #root::option::Option<Self::Item>;
                #[inline]
                fn nth(&mut self, n: usize) -> #root::option::Option<Self::Item>;
                #[inline]
                #[must_use = "if you really need to exhaust the iterator, consider `.for_each(drop)` instead"]
                fn collect<__U: #iter::FromIterator<Self::Item>>(self) -> __U;
                #[inline]
                fn partition<__U, __F>(self, f: __F) -> (__U, __U)
                where
                    __U: #root::default::Default + #iter::Extend<Self::Item>,
                    __F: #root::ops::FnMut(&Self::Item) -> bool;
            }
        })?,
    )?;

    #[cfg(feature = "try_trait")]
    impls.push_method(syn::parse2(quote! {
        #[inline]
        fn try_fold<__U, __F, __R>(&mut self, init: __U, f: __F) -> __R
        where
            __F: #root::ops::FnMut(__U, Self::Item) -> __R,
            __R: #root::ops::Try<Ok = __U>;
    })?)?;
    // It is equally efficient if `try_fold` can be used.
    #[cfg(not(feature = "try_trait"))]
    impls.append_items_from_trait(syn::parse2(quote! {
        trait Iterator {
            #[inline]
            fn fold<__U, __F>(self, init: __U, f: __F) -> __U
            where
                __F: #root::ops::FnMut(__U, Self::Item) -> __U;
            #[inline]
            fn all<__F>(&mut self, f: __F) -> bool
            where
                __F: #root::ops::FnMut(Self::Item) -> bool;
            #[inline]
            fn any<__F>(&mut self, f: __F) -> bool
            where
                __F: #root::ops::FnMut(Self::Item) -> bool;
            #[inline]
            fn find<__P>(&mut self, predicate: __P) -> #root::option::Option<Self::Item>
            where
                __P: #root::ops::FnMut(&Self::Item) -> bool;
            #[inline]
            fn find_map<__U, __F>(&mut self, f: __F) -> #root::option::Option<__U>
            where
                __F: #root::ops::FnMut(Self::Item) -> #root::option::Option<__U>;
            #[inline]
            fn position<__P>(&mut self, predicate: __P) -> #root::option::Option<usize>
            where
                __P: #root::ops::FnMut(Self::Item) -> bool;
        }
    })?)?;

    Ok(impls.build())
}
