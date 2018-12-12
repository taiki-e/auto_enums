use proc_macro2::TokenStream;
use quote::quote;
use smallvec::SmallVec;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["AsMut"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
    let trait_ = quote!(#root::convert::AsMut);

    data.impl_trait_with_capacity(
        1,
        root,
        syn::parse2(trait_)?,
        SmallVec::new(),
        syn::parse2(quote! {
            trait AsMut<__T: ?Sized> {
                #[inline]
                fn as_mut(&mut self) -> &mut __T;
            }
        })?,
    )
    .map(build)
}
