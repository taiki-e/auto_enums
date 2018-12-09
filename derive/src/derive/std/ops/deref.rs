use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Deref"];

pub(crate) fn enum_derive(data: &syn::ItemEnum) -> Result<TokenStream> {
    EnumData::parse(data, true, true).map(|data| deref(&data, &std_root()))
}

fn deref(data: &EnumData<'_>, root: &TokenStream) -> TokenStream {
    let EnumData {
        name,
        impl_generics,
        ty_generics,
        where_clause,
        variants,
        fields,
    } = data;

    let trait_ = quote!(#root::ops::Deref);
    let impl_generics = quote!(#impl_generics __T: ?Sized>);

    let where_clause = fields.iter().fold(where_clause.clone(), |t, f| {
        t.extend_and_return(quote!(#f: #trait_<Target = __T>,))
    });

    quote! {
        impl #impl_generics #trait_ for #name #ty_generics #where_clause {
            type Target = __T;
            #[inline]
            fn deref(&self) -> &Self::Target {
                match self { #(#variants(x) => x.deref(),)* }
            }
        }
    }
}
