use proc_macro2::TokenStream;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Extend"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();

    derive_trait!(
        data,
        parse_quote!(#root::iter::Extend)?,
        parse_quote! {
            trait Extend<__A> {
                #[inline]
                fn extend<__T: #root::iter::IntoIterator<Item = __A>>(&mut self, iter: __T);
            }
        }?,
    )
}
