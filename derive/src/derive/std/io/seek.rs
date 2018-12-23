use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Seek", "io::Seek"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
    let io = quote!(#root::io);

    derive_trait!(
        data,
        syn::parse2(quote!(#io::Seek))?,
        syn::parse2(quote! {
            trait Seek {
                #[inline]
                fn seek(&mut self, pos: #io::SeekFrom) -> #io::Result<u64>;
            }
        })?
    )
}
