use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Error"];

pub(crate) fn enum_derive(data: &syn::ItemEnum) -> Result<TokenStream> {
    EnumData::parse(data, false, true).map(|data| error(&data, &std_root()))
}

fn error(data: &EnumData<'_>, root: &TokenStream) -> TokenStream {
    let EnumData {
        name,
        impl_generics,
        ty_generics,
        where_clause,
        variants,
        fields,
    } = data;

    let trait_ = quote!(#root::error::Error);

    let where_clause = fields.iter().fold(where_clause.clone(), |t, f| {
        t.extend_and_return(quote!(#f: #trait_,))
    });

    quote! {
        impl #impl_generics #trait_ for #name #ty_generics #where_clause {
            fn description(&self) -> &str {
                match self { #(#variants(x) => x.description(),)* }
            }
            fn cause(&self) -> Option<&dyn (#trait_)> {
                match self { #(#variants(x) => x.cause(),)* }
            }
        }
    }
}
