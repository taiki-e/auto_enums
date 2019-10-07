use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Iterator"];

pub(crate) fn derive(data: &Data, items: &mut Vec<ItemImpl>) -> Result<()> {
    // TODO: When `try_trait` stabilized, add `try_fold` and remove `fold`, `find` etc. conditionally.

    // It is equally efficient if `try_fold` can be used.
    let try_trait = quote! {
        #[inline]
        fn fold<__U, __F>(self, init: __U, f: __F) -> __U
        where
            __F: ::core::ops::FnMut(__U, Self::Item) -> __U;
        #[inline]
        fn all<__F>(&mut self, f: __F) -> bool
        where
            __F: ::core::ops::FnMut(Self::Item) -> bool;
        #[inline]
        fn any<__F>(&mut self, f: __F) -> bool
        where
            __F: ::core::ops::FnMut(Self::Item) -> bool;
        #[inline]
        fn find<__P>(&mut self, predicate: __P) -> ::core::option::Option<Self::Item>
        where
            __P: ::core::ops::FnMut(&Self::Item) -> bool;
        #[inline]
        fn find_map<__U, __F>(&mut self, f: __F) -> ::core::option::Option<__U>
        where
            __F: ::core::ops::FnMut(Self::Item) -> ::core::option::Option<__U>;
        #[inline]
        fn position<__P>(&mut self, predicate: __P) -> ::core::option::Option<usize>
        where
            __P: ::core::ops::FnMut(Self::Item) -> bool;
    };

    derive_trait!(
        data,
        parse_quote!(::core::iter::Iterator)?,
        parse_quote! {
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
                fn nth(&mut self, n: usize) -> ::core::option::Option<Self::Item>;
                #[inline]
                #[must_use = "if you really need to exhaust the iterator, consider `.for_each(drop)` instead"]
                fn collect<__U: ::core::iter::FromIterator<Self::Item>>(self) -> __U;
                #[inline]
                fn partition<__U, __F>(self, f: __F) -> (__U, __U)
                where
                    __U: ::core::default::Default + ::core::iter::Extend<Self::Item>,
                    __F: ::core::ops::FnMut(&Self::Item) -> bool;
                #try_trait
            }
        }?,
    )
    .map(|item| items.push(item))
}
