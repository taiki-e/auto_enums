use crate::utils::*;

pub(crate) const NAME: &[&str] = &["DerefMut"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    derive_trait(
        data,
        parse_quote!(::core::ops::DerefMut),
        Some(format_ident!("Target")),
        parse_quote! {
            trait DerefMut: ::core::ops::Deref {
                #[inline]
                fn deref_mut(&mut self) -> &mut Self::Target;
            }
        },
    )
}
