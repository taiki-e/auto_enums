use proc_macro2::TokenStream;
use quote::quote;
use smallvec::SmallVec;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Seek", "io::Seek"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
    let io = quote!(#root::io);

    data.impl_trait_with_capacity(
        1,
        root.clone(),
        syn::parse2(quote!(#io::Seek))?,
        SmallVec::new(),
        syn::parse2(quote! {
            trait Seek {
                #[inline]
                fn seek(&mut self, pos: #io::SeekFrom) -> #io::Result<u64>;
            }
        })?,
    )
    .map(build)
}
