use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Deref"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    derive_trait(data, parse_quote!(::core::ops::Deref), None, parse_quote! {
        trait Deref {
            type Target;
            #[inline]
            fn deref(&self) -> &Self::Target;
        }
    })
}
