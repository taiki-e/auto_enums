use proc_macro2::TokenStream;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["rayon::ParallelExtend"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let iter = quote!(::rayon::iter);

    derive_trait!(
        data,
        parse_quote!(#iter::ParallelExtend)?,
        parse_quote! {
            trait ParallelExtend<__T: Send> {
                #[inline]
                fn par_extend<__I>(&mut self, par_iter: __I)
                where
                    __I: #iter::IntoParallelIterator<Item = __T>;
            }
        }?,
    )
}
