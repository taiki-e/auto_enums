use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Index"];

pub(crate) fn enum_derive(data: &syn::ItemEnum) -> Result<TokenStream> {
    EnumData::parse(data, true, true).map(|data| index(&data, &std_root()))
}

fn index(data: &EnumData<'_>, root: &TokenStream) -> TokenStream {
    let EnumData {
        name,
        impl_generics,
        ty_generics,
        where_clause,
        variants,
        fields,
    } = data;

    let ops = quote!(#root::ops);
    let trait_ = quote!(#ops::Index);
    let fst = &fields[0];

    #[cfg(feature = "unsized_locals")]
    let impl_generics = quote!(#impl_generics __Idx: ?Sized>);
    #[cfg(not(feature = "unsized_locals"))]
    let impl_generics = quote!(#impl_generics __Idx>);

    let where_clause =
        fields
            .iter()
            .skip(1)
            .fold(quote!(#where_clause #fst: #trait_<__Idx>,), |t, f| {
                t.extend_and_return(
                    quote!(#f: #trait_<__Idx, Output = <#fst as #trait_<__Idx>>::Output>,),
                )
            });

    quote! {
        impl #impl_generics #trait_<__Idx> for #name #ty_generics #where_clause {
            type Output = <#fst as #trait_<__Idx>>::Output;
            #[inline]
            fn index(&self, index: __Idx) -> &Self::Output {
                match self { #(#variants(x) => x.index(index),)* }
            }
        }
    }
}
