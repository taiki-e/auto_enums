use proc_macro2::TokenStream;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["FusedIterator"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    derive_trait!(
        data,
        Some(ident_call_site("Item")),
        parse_quote!(::core::iter::FusedIterator)?,
        parse_quote! {
            trait FusedIterator: ::core::iter::Iterator {}
        }?,
    )
}
