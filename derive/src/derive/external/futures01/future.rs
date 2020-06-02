use crate::utils::*;

pub(crate) const NAME: &[&str] = &["futures01::Future"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    derive_trait(data, parse_quote!(::futures::future::Future), None, parse_quote! {
        trait Future {
            type Item;
            type Error;
            #[inline]
            fn poll(&mut self) -> ::futures::Poll<Self::Item, Self::Error>;
        }
    })
}
