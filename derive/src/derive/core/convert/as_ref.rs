use crate::utils::*;

pub(crate) const NAME: &[&str] = &["AsRef"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    derive_trait(data, parse_quote!(::core::convert::AsRef), None, parse_quote! {
        trait AsRef<__T: ?Sized> {
            #[inline]
            fn as_ref(&self) -> &__T;
        }
    })
}
