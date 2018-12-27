use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["RangeBounds"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
    let ops = quote!(#root::ops);

    derive_trait!(
        data,
        parse_quote!(#ops::RangeBounds)?,
        parse_quote! {
            trait RangeBounds<__T: ?Sized> {
                #[inline]
                fn start_bound(&self) -> #ops::Bound<&__T>;
                #[inline]
                fn end_bound(&self) -> #ops::Bound<&__T>;
            }
        }?,
    )
}
