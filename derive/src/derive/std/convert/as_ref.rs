use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["AsRef"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
    let trait_ = quote!(#root::convert::AsRef);

    data.impl_trait_with_capacity(
        1,
        syn::parse2(trait_)?,
        None,
        syn::parse2(quote! {
            trait AsRef<__T: ?Sized> {
                #[inline]
                fn as_ref(&self) -> &__T;
            }
        })?,
    )
    .map(build)
}
