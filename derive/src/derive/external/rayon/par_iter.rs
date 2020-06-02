use crate::utils::*;

pub(crate) const NAME: &[&str] = &["rayon::ParallelIterator"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    derive_trait(data, parse_quote!(::rayon::iter::ParallelIterator), None, parse_quote! {
        trait ParallelIterator {
            type Item;
            #[inline]
            fn drive_unindexed<__C>(self, consumer: __C) -> __C::Result
            where
                __C: ::rayon::iter::plumbing::UnindexedConsumer<Self::Item>;
            #[inline]
            fn opt_len(&self) -> ::core::option::Option<usize>;
        }
    })
}
