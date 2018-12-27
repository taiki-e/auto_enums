use proc_macro2::TokenStream;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["AsRef"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();

    derive_trait!(
        data,
        parse_quote!(#root::convert::AsRef)?,
        parse_quote! {
            trait AsRef<__T: ?Sized> {
                #[inline]
                fn as_ref(&self) -> &__T;
            }
        }?,
    )
}
