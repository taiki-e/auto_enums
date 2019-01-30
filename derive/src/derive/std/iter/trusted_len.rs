use proc_macro2::TokenStream;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["TrustedLen"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();

    derive_trait!(
        data,
        Some(ident_call_site("Item")),
        parse_quote!(#root::iter::TrustedLen)?,
        parse_quote! {
            unsafe trait TrustedLen: #root::iter::Iterator {}
        }?,
    )
}
