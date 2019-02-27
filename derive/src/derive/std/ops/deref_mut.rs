use proc_macro2::TokenStream;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["DerefMut"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    derive_trait!(
        data,
        Some(ident_call_site("Target")),
        parse_quote!(::core::ops::DerefMut)?,
        parse_quote! {
            trait DerefMut: ::core::ops::Deref {
                #[inline]
                fn deref_mut(&mut self) -> &mut Self::Target;
            }
        }?,
    )
}
