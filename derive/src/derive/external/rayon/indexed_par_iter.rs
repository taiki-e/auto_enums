use crate::utils::*;

pub(crate) const NAME: &[&str] = &["rayon::IndexedParallelIterator"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    derive_trait(
        data,
        parse_quote!(::rayon::iter::IndexedParallelIterator),
        Some(format_ident!("Item")),
        parse_quote! {
            trait IndexedParallelIterator: ::rayon::iter::ParallelIterator {
                #[inline]
                fn drive<__C>(self, consumer: __C) -> __C::Result
                where
                    __C: ::rayon::iter::plumbing::Consumer<Self::Item>;
                #[inline]
                fn len(&self) -> usize;
                #[inline]
                fn with_producer<__CB>(self, callback: __CB) -> __CB::Output
                where
                    __CB: ::rayon::iter::plumbing::ProducerCallback<Self::Item>;
            }
        },
    )
}
