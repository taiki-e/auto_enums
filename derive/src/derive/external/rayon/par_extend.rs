use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["rayon::ParallelExtend"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let iter = quote!(::rayon::iter);

    data.impl_trait_with_capacity(
        1,
        syn::parse2(quote!(#iter::ParallelExtend))?,
        None,
        syn::parse2(quote! {
            trait ParallelExtend<__T: Send> {
                #[inline]
                fn par_extend<__I>(&mut self, par_iter: __I)
                where
                    __I: #iter::IntoParallelIterator<Item = __T>;
            }
        })?,
    )
    .map(build)
}

