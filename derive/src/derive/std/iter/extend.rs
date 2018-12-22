use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Extend"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
    let iter = quote!(#root::iter);

    derive_trait_with_capacity!(
        data,
        1,
        syn::parse2(quote!(#iter::Extend))?,
        syn::parse2(quote! {
            trait Extend<__A> {
                #[inline]
                fn extend<__T: #iter::IntoIterator<Item = __A>>(&mut self, iter: __T);
            }
        })?
    )
}
