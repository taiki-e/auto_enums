use crate::utils::*;

pub(crate) const NAME: &[&str] = &["DoubleEndedIterator"];

pub(crate) fn derive(data: &Data, items: &mut Vec<ItemImpl>) -> Result<()> {
    // TODO: When `try_trait` stabilized, add `try_rfold` and remove `rfold` and `rfind` conditionally.

    // It is equally efficient if `try_rfold` can be used.
    let try_trait = quote! {
        fn rfold<__U, __F>(self, accum: __U, f: __F) -> __U
        where
            __F: ::core::ops::FnMut(__U, Self::Item) -> __U;
        fn rfind<__P>(&mut self, predicate: __P) -> ::core::option::Option<Self::Item>
        where
            __P: ::core::ops::FnMut(&Self::Item) -> bool;
    };

    derive_trait!(
        data,
        Some(format_ident!("Item")),
        parse_quote!(::core::iter::DoubleEndedIterator)?,
        parse_quote! {
            trait DoubleEndedIterator: ::core::iter::Iterator {
                fn next_back(&mut self) -> ::core::option::Option<Self::Item>;
                #try_trait
            }
        }?,
    )
    .map(|item| items.push(item))
}
