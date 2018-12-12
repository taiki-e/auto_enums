use proc_macro2::TokenStream;
use quote::quote;
use smallvec::SmallVec;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["futures01::Future"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let crate_ = quote!(::futures);

    data.impl_trait_with_capacity(
        3,
        std_root(),
        syn::parse2(quote!(#crate_::future::Future))?,
        SmallVec::new(),
        syn::parse2(quote! {
            trait Future {
                type Item;
                type Error;
                #[inline]
                fn poll(&mut self) -> #crate_::Poll<Self::Item, Self::Error>;
            }
        })?,
    )
    .map(build)
}
