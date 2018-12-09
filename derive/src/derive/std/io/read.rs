use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Read", "io::Read"];

pub(crate) fn enum_derive(data: &syn::ItemEnum) -> Result<TokenStream> {
    EnumData::parse(data, false, true).map(|data| read(&data, &std_root()))
}

fn read(data: &EnumData<'_>, root: &TokenStream) -> TokenStream {
    let EnumData {
        name,
        impl_generics,
        ty_generics,
        where_clause,
        variants,
        fields,
    } = data;

    let io = quote!(#root::io);
    let trait_ = quote!(#io::Read);

    let where_clause = fields.iter().fold(where_clause.clone(), |t, f| {
        t.extend_and_return(quote!(#f: #trait_,))
    });

    // method
    #[cfg(not(feature = "read_initializer"))]
    let initializer = TokenStream::new();
    #[cfg(feature = "read_initializer")]
    let initializer = quote! {
        #[allow(unsafe_code)]
        #[inline]
        unsafe fn initializer(&self) -> #io::Initializer {
            match self { #(#variants(x) => x.initializer(),)* }
        }
    };

    quote! {
        impl #impl_generics #trait_ for #name #ty_generics #where_clause {
            #[inline]
            fn read(&mut self, buf: &mut [u8]) -> #io::Result<usize> {
                match self { #(#variants(x) => x.read(buf),)* }
            }
            #[inline]
            fn read_to_end(&mut self, buf: &mut #root::vec::Vec<u8>) -> #io::Result<usize> {
                match self { #(#variants(x) => x.read_to_end(buf),)* }
            }
            #[inline]
            fn read_to_string(&mut self, buf: &mut #root::string::String) -> #io::Result<usize> {
                match self { #(#variants(x) => x.read_to_string(buf),)* }
            }
            #[inline]
            fn read_exact(&mut self, buf: &mut [u8]) -> #io::Result<()> {
                match self { #(#variants(x) => x.read_exact(buf),)* }
            }
            #initializer
        }
    }
}
