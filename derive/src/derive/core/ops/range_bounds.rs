use crate::utils::*;

pub(crate) const NAME: &[&str] = &["RangeBounds"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    derive_trait(data, parse_quote!(::core::ops::RangeBounds), None, parse_quote! {
        trait RangeBounds<__T: ?Sized> {
            #[inline]
            fn start_bound(&self) -> ::core::ops::Bound<&__T>;
            #[inline]
            fn end_bound(&self) -> ::core::ops::Bound<&__T>;
        }
    })
}
