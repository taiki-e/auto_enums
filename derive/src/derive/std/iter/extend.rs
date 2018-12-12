use proc_macro2::TokenStream;
use quote::quote;
use smallvec::SmallVec;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Extend"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
    let iter = quote!(#root::iter);

    data.impl_trait_with_capacity(
        1,
        root,
        syn::parse2(quote!(#iter::Extend))?,
        SmallVec::new(),
        syn::parse2(quote! {
            trait Extend<__A> {
                #[inline]
                fn extend<__T: #iter::IntoIterator<Item = __A>>(&mut self, iter: __T);
            }
        })?,
    )
    .map(build)
}
