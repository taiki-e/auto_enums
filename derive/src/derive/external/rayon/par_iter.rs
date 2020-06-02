use crate::utils::*;

pub(crate) const NAME: &[&str] = &["rayon::ParallelIterator"];

pub(crate) fn derive(data: &Data, items: &mut Vec<ItemImpl>) -> Result<()> {
    let iter = quote!(::rayon::iter);

    derive_trait!(data, parse_quote!(#iter::ParallelIterator)?, parse_quote! {
        trait ParallelIterator {
            type Item;
            #[inline]
            fn drive_unindexed<__C>(self, consumer: __C) -> __C::Result
            where
                __C: #iter::plumbing::UnindexedConsumer<Self::Item>;
            #[inline]
            fn opt_len(&self) -> ::core::option::Option<usize>;
        }
    }?,)
    .map(|item| items.push(item))
}
