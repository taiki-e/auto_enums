use crate::utils::*;

pub(crate) const NAME: &[&str] = &["rayon::IndexedParallelIterator"];

pub(crate) fn derive(data: &Data, items: &mut Vec<ItemImpl>) -> Result<()> {
    let iter = quote!(::rayon::iter);

    derive_trait!(
        data,
        Some(format_ident!("Item")),
        parse_quote!(#iter::IndexedParallelIterator)?,
        parse_quote! {
            trait IndexedParallelIterator: #iter::ParallelIterator {
                fn drive<__C>(self, consumer: __C) -> __C::Result
                where
                    __C: #iter::plumbing::Consumer<Self::Item>;
                fn len(&self) -> usize;
                fn with_producer<__CB>(self, callback: __CB) -> __CB::Output
                where
                    __CB: #iter::plumbing::ProducerCallback<Self::Item>;
            }
        }?,
    )
    .map(|item| items.push(item))
}
