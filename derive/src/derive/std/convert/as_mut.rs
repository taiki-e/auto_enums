use proc_macro2::TokenStream;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["AsMut"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();

    derive_trait!(
        data,
        parse_quote!(#root::convert::AsMut)?,
        parse_quote! {
            trait AsMut<__T: ?Sized> {
                #[inline]
                fn as_mut(&mut self) -> &mut __T;
            }
        }?,
    )
}
