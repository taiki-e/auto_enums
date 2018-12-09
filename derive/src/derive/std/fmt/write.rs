use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["fmt::Write"];

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

    let fmt = quote!(#root::fmt);
    let trait_ = quote!(#fmt::Write);

    let where_clause = fields.iter().fold(where_clause.clone(), |t, f| {
        t.extend_and_return(quote!(#f: #trait_,))
    });

    quote! {
        impl #impl_generics #trait_ for #name #ty_generics #where_clause {
            #[inline]
            fn write_str(&mut self, s: &str) -> #fmt::Result {
                match self { #(#variants(x) => x.write_str(s),)* }
            }
            #[inline]
            fn write_char(&mut self, c: char) -> #fmt::Result {
                match self { #(#variants(x) => x.write_char(c),)* }
            }
            #[inline]
            fn write_fmt(&mut self, args: #fmt::Arguments<'_>) -> #fmt::Result {
                match self { #(#variants(x) => x.write_fmt(args),)* }
            }
        }
    }
}
