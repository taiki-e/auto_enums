use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Error"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
    let trait_ = quote!(#root::error::Error);

    data.impl_trait_with_capacity(
        2,
        syn::parse2(trait_.clone())?,
        None,
        syn::parse2(quote! {
            trait Error {
                fn description(&self) -> &str;
                #[allow(deprecated)]
                fn cause(&self) -> #root::option::Option<&dyn (#trait_)>;
                fn source(&self) -> #root::option::Option<&(dyn (#trait_) + 'static)>;
            }
        })?,
    )
    .map(build)
}
