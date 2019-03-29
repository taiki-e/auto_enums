use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Iterator"];

pub(crate) fn derive(data: &Data, stack: &mut Stack<ItemImpl>) -> Result<()> {
    #[cfg(feature = "try_trait")]
    let try_trait = quote! {
        #[inline]
        fn try_fold<__U, __F, __R>(&mut self, init: __U, f: __F) -> __R
        where
            __F: ::core::ops::FnMut(__U, Self::Item) -> __R,
            __R: ::core::ops::Try<Ok = __U>;
    };
    // It is equally efficient if `try_fold` can be used.
    #[cfg(not(feature = "try_trait"))]
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
    .map(|item| stack.push(item))
}
