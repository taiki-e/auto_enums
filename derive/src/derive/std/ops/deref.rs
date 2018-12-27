use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Deref"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
    let ops = quote!(#root::ops);

    derive_trait!(
        data,
        parse_quote!(#ops::Deref)?,
        parse_quote! {
            trait Deref {
                type Target;
                #[inline]
                fn deref(&self) -> &Self::Target;
            }
        }?,
    )
}
