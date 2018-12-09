use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Extend"];

pub(crate) fn enum_derive(data: &syn::ItemEnum) -> Result<TokenStream> {
    EnumData::parse(data, true, true).map(|data| extend(&data, &std_root()))
}

fn extend(data: &EnumData<'_>, root: &TokenStream) -> TokenStream {
    let EnumData {
        name,
        impl_generics,
        ty_generics,
        where_clause,
        variants,
        fields,
    } = data;

    let iter = quote!(#root::iter);
    let trait_ = quote!(#iter::Extend);
    let impl_generics = quote!(#impl_generics __A>);

    let where_clause = fields.iter().fold(where_clause.clone(), |t, f| {
        t.extend_and_return(quote!(#f: #trait_<__A>,))
    });

    quote! {
        impl #impl_generics #trait_<__A> for #name #ty_generics #where_clause {
            #[inline]
            fn extend<__T: #iter::IntoIterator<Item = __A>>(&mut self, iter: __T) {
                match self { #(#variants(x) => x.extend(iter),)* }
            }
        }
    }
}
