use proc_macro2::TokenStream;
use quote::quote;
use smallvec::SmallVec;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Read", "io::Read"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    #[cfg(feature = "read_initializer")]
    const CAPACITY: usize = 5;
    #[cfg(not(feature = "read_initializer"))]
    const CAPACITY: usize = 4;

    let root = std_root();
    let io = quote!(#root::io);

    #[allow(unused_mut)]
    let mut impls = data.impl_trait_with_capacity(
        CAPACITY,
        root.clone(),
        syn::parse2(quote!(#io::Read))?,
        SmallVec::new(),
        syn::parse2(quote! {
            trait Iterator {
                #[inline]
                fn read(&mut self, buf: &mut [u8]) -> #io::Result<usize>;
                #[inline]
                fn read_to_end(&mut self, buf: &mut #root::vec::Vec<u8>) -> #io::Result<usize>;
                #[inline]
                fn read_to_string(&mut self, buf: &mut #root::string::String) -> #io::Result<usize>;
                #[inline]
                fn read_exact(&mut self, buf: &mut [u8]) -> #io::Result<()>;
            }
        })?,
    )?;

    #[cfg(feature = "read_initializer")]
    impls.push_method(syn::parse2(quote! {
        #[inline]
        unsafe fn initializer(&self) -> #io::Initializer;
    })?)?;

    Ok(impls.build())
}
