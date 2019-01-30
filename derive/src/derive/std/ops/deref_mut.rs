use proc_macro2::TokenStream;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["DerefMut"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();

    derive_trait!(
        data,
        Some(ident_call_site("Target")),
        parse_quote!(#root::ops::DerefMut)?,
        parse_quote! {
            trait DerefMut: #root::ops::Deref {
                #[inline]
                fn deref_mut(&mut self) -> &mut Self::Target;
            }
        }?,
    )
}
