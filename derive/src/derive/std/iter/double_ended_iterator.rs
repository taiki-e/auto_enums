use crate::utils::*;

pub(crate) const NAME: &[&str] = &["DoubleEndedIterator"];

pub(crate) fn derive(data: &Data, stack: &mut Stack<ItemImpl>) -> Result<()> {
    #[cfg(feature = "try_trait")]
    let try_trait = quote! {
        #[inline]
        fn try_rfold<__U, __F, __R>(&mut self, init: __U, f: __F) -> __R
        where
            __F: ::core::ops::FnMut(__U, Self::Item) -> __R,
            __R: ::core::ops::Try<Ok = __U>;
    };
    // It is equally efficient if `try_rfold` can be used.
    #[cfg(not(feature = "try_trait"))]
    let try_trait = quote! {
        #[inline]
        fn rfold<__U, __F>(self, accum: __U, f: __F) -> __U
        where
            __F: ::core::ops::FnMut(__U, Self::Item) -> __U;
        #[inline]
        fn rfind<__P>(&mut self, predicate: __P) -> ::core::option::Option<Self::Item>
        where
            __P: ::core::ops::FnMut(&Self::Item) -> bool;
    };

    derive_trait!(
        data,
        Some(ident("Item")),
        parse_quote!(::core::iter::DoubleEndedIterator)?,
        parse_quote! {
            trait DoubleEndedIterator: ::core::iter::Iterator {
                #[inline]
                fn next_back(&mut self) -> ::core::option::Option<Self::Item>;
                #try_trait
            }
        }?,
    )
    .map(|item| stack.push(item))
}
