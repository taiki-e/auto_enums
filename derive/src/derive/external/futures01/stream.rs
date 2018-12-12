use proc_macro2::TokenStream;
use quote::quote;
use smallvec::SmallVec;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["futures01::Stream"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
    let crate_ = quote!(::futures);

    data.impl_trait_with_capacity(
        3,
        root.clone(),
        syn::parse2(quote!(#crate_::stream::Stream))?,
        SmallVec::new(),
        syn::parse2(quote! {
            trait Stream {
                type Item;
                type Error;
                #[inline]
                fn poll(&mut self) -> #crate_::Poll<#root::option::Option<Self::Item>, Self::Error>;
            }
        })?,
    )
    .map(build)
}
