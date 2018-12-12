use proc_macro2::TokenStream;
use quote::quote;
use smallvec::SmallVec;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Error"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
    let trait_ = quote!(#root::error::Error);

    data.impl_trait_with_capacity(
        2,
        root,
        syn::parse2(trait_.clone())?,
        SmallVec::new(),
        syn::parse2(quote! {
            trait Error {
                fn description(&self) -> &str;
                fn cause(&self) -> Option<&dyn (#trait_)>;
            }
        })?,
    )
    .map(build)
}
