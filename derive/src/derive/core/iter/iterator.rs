use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Iterator"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    // TODO: When `try_trait` stabilized, add `try_fold` and remove `fold`, `find` etc. conditionally.

    // It is equally efficient if `try_fold` can be used.
    let try_trait = quote! {
        #[inline]
        fn fold<__U, __F>(self, init: __U, f: __F) -> __U
        where
            __F: ::core::ops::FnMut(__U, Self::Item) -> __U;
        #[inline]
        fn find<__P>(&mut self, predicate: __P) -> ::core::option::Option<Self::Item>
        where
            __P: ::core::ops::FnMut(&Self::Item) -> bool;
        #[inline]
        fn find_map<__U, __F>(&mut self, f: __F) -> ::core::option::Option<__U>
        where
            __F: ::core::ops::FnMut(Self::Item) -> ::core::option::Option<__U>;
    };

    derive_trait(data, parse_quote!(::core::iter::Iterator), None, parse_quote! {
        trait Iterator {
            type Item;
            #[inline]
            fn next(&mut self) -> ::core::option::Option<Self::Item>;
            #[inline]
            fn size_hint(&self) -> (usize, ::core::option::Option<usize>);
            #[inline]
            fn count(self) -> usize;
            #[inline]
            fn last(self) -> ::core::option::Option<Self::Item>;
            #[inline]
            #[must_use = "if you really need to exhaust the iterator, consider `.for_each(drop)` instead"]
            fn collect<__U: ::core::iter::FromIterator<Self::Item>>(self) -> __U;
            #try_trait
        }
    })
}
