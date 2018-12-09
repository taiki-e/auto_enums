use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["BufRead", "io::BufRead"];

pub(crate) fn enum_derive(data: &syn::ItemEnum) -> Result<TokenStream> {
    EnumData::parse(data, false, true).map(|data| buf_read(&data, &std_root()))
}

fn buf_read(data: &EnumData<'_>, root: &TokenStream) -> TokenStream {
    let EnumData {
        name,
        impl_generics,
        ty_generics,
        where_clause,
        variants,
        fields,
    } = data;

    let io = quote!(#root::io);
    let trait_ = quote!(#io::BufRead);

    let where_clause = fields.iter().fold(where_clause.clone(), |t, f| {
        t.extend_and_return(quote!(#f: #trait_,))
    });

    quote! {
        impl #impl_generics #trait_ for #name #ty_generics #where_clause {
            #[inline]
            fn fill_buf(&mut self) -> #io::Result<&[u8]> {
                match self { #(#variants(x) => x.fill_buf(),)* }
            }
            #[inline]
            fn consume(&mut self, amt: usize) {
                match self { #(#variants(x) => x.consume(amt),)* }
            }
            #[inline]
            fn read_until(&mut self, byte: u8, buf: &mut #root::vec::Vec<u8>) -> #io::Result<usize> {
                match self { #(#variants(x) => x.read_until(byte, buf),)* }
            }
            #[inline]
            fn read_line(&mut self, buf: &mut #root::string::String) -> #io::Result<usize> {
                match self { #(#variants(x) => x.read_line(buf),)* }
            }
        }
    }
}
