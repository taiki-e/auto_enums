
use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["serde::Serialize"];

pub(crate) fn enum_derive(data: &syn::ItemEnum) -> Result<TokenStream> {
    EnumData::parse(data, false, true).map(|data| serialize(&data, &std_root()))
}

fn serialize(data: &EnumData<'_>, root: &TokenStream) -> TokenStream {
    let EnumData {
        name,
        impl_generics,
        ty_generics,
        where_clause,
        variants,
        fields,
    } = data;

    let ser = quote!(::serde::ser);
    let trait_ = quote!(#ser::Serialize);

    let where_clause = fields.iter().fold(where_clause.clone(), |t, f| {
        t.extend_and_return(quote!(#f: #trait_,))
    });

    quote! {
        impl #impl_generics #trait_ for #name #ty_generics #where_clause {
            #[inline]
            fn serialize<__S>(&self, serializer: __S) -> #root::result::Result<__S::Ok, __S::Error>
            where
                __S: #ser::Serializer
            {
                match self { #(#variants(x) => x.serialize(serializer),)* }
            }
        }
    }
}
