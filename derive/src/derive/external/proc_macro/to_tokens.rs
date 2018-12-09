use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["quote::ToTokens"];

pub(crate) fn enum_derive(data: &syn::ItemEnum) -> Result<TokenStream> {
    EnumData::parse(data, false, true).map(|data| to_tokens(&data, &std_root()))
}

fn to_tokens(data: &EnumData<'_>, _root: &TokenStream) -> TokenStream {
    let EnumData {
        name,
        impl_generics,
        ty_generics,
        where_clause,
        variants,
        fields,
    } = data;

    let crate_ = quote!(::quote);
    let trait_ = quote!(#crate_::ToTokens);

    let where_clause = fields.iter().fold(where_clause.clone(), |t, f| {
        t.extend_and_return(quote!(#f: #trait_,))
    });

    quote! {
        impl #impl_generics #trait_ for #name #ty_generics #where_clause {
            #[inline]
            fn to_tokens(&self, tokens: &mut #crate_::__rt::TokenStream) {
                match self { #(#variants(x) => x.to_tokens(tokens),)* }
            }
            #[inline]
            fn into_token_stream(self) -> #crate_::__rt::TokenStream {
                match self { #(#variants(x) => x.into_token_stream(),)* }
            }
        }
    }
}
