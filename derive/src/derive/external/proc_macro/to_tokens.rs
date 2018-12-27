use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["quote::ToTokens"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let crate_ = quote!(::quote);

    derive_trait!(
        data,
        parse_quote!(#crate_::ToTokens)?,
        parse_quote! {
            trait ToTokens {
                #[inline]
                fn to_tokens(&self, tokens: &mut #crate_::__rt::TokenStream);
                #[inline]
                fn into_token_stream(self) -> #crate_::__rt::TokenStream;
            }
        }?,
    )
}
