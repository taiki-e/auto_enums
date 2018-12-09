use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Write", "io::Write"];

pub(crate) fn enum_derive(data: &syn::ItemEnum) -> Result<TokenStream> {
    EnumData::parse(data, false, true).map(|data| write(&data, &std_root()))
}

fn write(data: &EnumData<'_>, root: &TokenStream) -> TokenStream {
    let EnumData {
        name,
        impl_generics,
        ty_generics,
        where_clause,
        variants,
        fields,
    } = data;

    let io = quote!(#root::io);
    let trait_ = quote!(#io::Write);

    let where_clause = fields.iter().fold(where_clause.clone(), |t, f| {
        t.extend_and_return(quote!(#f: #trait_,))
    });

    quote! {
        impl #impl_generics #trait_ for #name #ty_generics #where_clause {
            #[inline]
            fn write(&mut self, buf: &[u8]) -> #io::Result<usize> {
                match self { #(#variants(x) => x.write(buf),)* }
            }
            #[inline]
            fn flush(&mut self) -> #io::Result<()> {
                match self { #(#variants(x) => x.flush(),)* }
            }
            #[inline]
            fn write_all(&mut self, buf: &[u8]) -> #io::Result<()> {
                match self { #(#variants(x) => x.write_all(buf),)* }
            }
            #[inline]
            fn write_fmt(&mut self, fmt: #root::fmt::Arguments) -> #io::Result<()> {
                match self { #(#variants(x) => x.write_fmt(fmt),)* }
            }
        }
    }
}
