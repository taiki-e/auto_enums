use proc_macro2::TokenStream;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["TrustedLen"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    derive_trait!(
        data,
        Some(ident_call_site("Item")),
        parse_quote!(::core::iter::TrustedLen)?,
        parse_quote! {
            unsafe trait TrustedLen: ::core::iter::Iterator {}
        }?,
    )
}
