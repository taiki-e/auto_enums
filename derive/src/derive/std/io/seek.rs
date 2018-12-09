use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Seek", "io::Seek"];

pub(crate) fn enum_derive(data: &syn::ItemEnum) -> Result<TokenStream> {
    EnumData::parse(data, false, true).map(|data| seek(&data, &std_root()))
}

fn seek(data: &EnumData<'_>, root: &TokenStream) -> TokenStream {
    let EnumData {
        name,
        impl_generics,
        ty_generics,
        where_clause,
        variants,
        fields,
    } = data;

    let io = quote!(#root::io);
    let trait_ = quote!(#io::Seek);

    let where_clause = fields.iter().fold(where_clause.clone(), |t, f| {
        t.extend_and_return(quote!(#f: #trait_,))
    });

    quote! {
        impl #impl_generics #trait_ for #name #ty_generics #where_clause {
            #[inline]
            fn seek(&mut self, pos: #io::SeekFrom) -> #io::Result<u64> {
                match self { #(#variants(x) => x.seek(pos),)* }
            }
        }
    }
}
