use proc_macro2::TokenStream;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["RangeBounds"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();

    derive_trait!(
        data,
        parse_quote!(#root::ops::RangeBounds)?,
        parse_quote! {
            trait RangeBounds<__T: ?Sized> {
                #[inline]
                fn start_bound(&self) -> #root::ops::Bound<&__T>;
                #[inline]
                fn end_bound(&self) -> #root::ops::Bound<&__T>;
            }
        }?,
    )
}
