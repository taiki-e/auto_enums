use proc_macro2::TokenStream;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Index"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    #[cfg(not(feature = "unsized_locals"))]
    let bounds = TokenStream::new();
    #[cfg(feature = "unsized_locals")]
    let bounds = quote!(: ?Sized);

    derive_trait!(
        data,
        parse_quote!(::core::ops::Index)?,
        parse_quote! {
            trait Index<__Idx #bounds> {
                type Output;
                #[inline]
                fn index(&self, index: __Idx) -> &Self::Output;
            }
        }?,
    )
}
