use crate::utils::*;

pub(crate) const NAME: &[&str] = &["rayon::ParallelExtend"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    derive_trait(data, parse_quote!(::rayon::iter::ParallelExtend), None, parse_quote! {
        trait ParallelExtend<__T: Send> {
            #[inline]
            fn par_extend<__I>(&mut self, par_iter: __I)
            where
                __I: ::rayon::iter::IntoParallelIterator<Item = __T>;
        }
    })
}
