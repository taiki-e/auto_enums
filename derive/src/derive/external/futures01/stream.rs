use crate::utils::*;

pub(crate) const NAME: &[&str] = &["futures01::Stream"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    derive_trait(data, parse_quote!(::futures::stream::Stream), None, parse_quote! {
        trait Stream {
            type Item;
            type Error;
            #[inline]
            fn poll(&mut self) -> ::futures::Poll<::core::option::Option<Self::Item>, Self::Error>;
        }
    })
}
