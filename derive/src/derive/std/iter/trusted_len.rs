use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["TrustedLen"];

pub(crate) fn enum_derive(data: &syn::ItemEnum) -> Result<TokenStream> {
    EnumData::parse(data, false, true).map(|data| trusted_len(&data, &std_root()))
}

fn trusted_len(data: &EnumData<'_>, root: &TokenStream) -> TokenStream {
    let EnumData {
        name,
        impl_generics,
        ty_generics,
        where_clause,
        fields,
        ..
    } = data;

    let iter = quote!(#root::iter);
    let trait_ = quote!(#iter::TrustedLen);
    let fst = &fields[0];

    let where_clause = fields
        .iter()
        .skip(1)
        .fold(quote!(#where_clause #fst: #trait_,), |t, f| {
            t.extend_and_return(quote!(#f: #trait_<Item = <#fst as #iter::Iterator>::Item>,))
        });

    quote! {
        #[allow(unsafe_code)]
        unsafe impl #impl_generics #trait_ for #name #ty_generics #where_clause {}
    }
}
