use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["DerefMut"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
    let ops = quote!(#root::ops);

    derive_trait!(
        data,
        Some(ident_call_site("Target")),
        parse_quote!(#ops::DerefMut)?,
        parse_quote! {
            trait DerefMut: #ops::Deref {
                #[inline]
                fn deref_mut(&mut self) -> &mut Self::Target;
            }
        }?,
    )
}
