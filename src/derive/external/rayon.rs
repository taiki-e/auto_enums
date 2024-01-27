// SPDX-License-Identifier: Apache-2.0 OR MIT

pub(crate) mod par_iter {
    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["rayon::ParallelIterator"];

    pub(crate) fn derive(_cx: &Context, data: &Data) -> Result<TokenStream> {
        Ok(derive_trait(data, &parse_quote!(::rayon::iter::ParallelIterator), None, parse_quote! {
            trait ParallelIterator {
                type Item;
                #[inline]
                fn drive_unindexed<__C>(self, consumer: __C) -> __C::Result
                where
                    __C: ::rayon::iter::plumbing::UnindexedConsumer<Self::Item>;
                #[inline]
                fn opt_len(&self) -> ::core::option::Option<usize>;
            }
        }))
    }
}

pub(crate) mod indexed_par_iter {
    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["rayon::IndexedParallelIterator"];

    pub(crate) fn derive(_cx: &Context, data: &Data) -> Result<TokenStream> {
        Ok(derive_trait(
            data,
            &parse_quote!(::rayon::iter::IndexedParallelIterator),
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
        ))
    }
}

pub(crate) mod par_extend {
    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["rayon::ParallelExtend"];

    pub(crate) fn derive(_cx: &Context, data: &Data) -> Result<TokenStream> {
        Ok(derive_trait(data, &parse_quote!(::rayon::iter::ParallelExtend), None, parse_quote! {
            trait ParallelExtend<__T: Send> {
                #[inline]
                fn par_extend<__I>(&mut self, par_iter: __I)
                where
                    __I: ::rayon::iter::IntoParallelIterator<Item = __T>;
            }
        }))
    }
}
