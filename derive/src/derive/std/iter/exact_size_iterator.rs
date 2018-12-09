use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["ExactSizeIterator"];

pub(crate) fn enum_derive(data: &syn::ItemEnum) -> Result<TokenStream> {
    EnumData::parse(data, false, true).map(|data| exact_size_iterator(&data, &std_root()))
}

fn exact_size_iterator(data: &EnumData<'_>, root: &TokenStream) -> TokenStream {
    let EnumData {
        name,
        impl_generics,
        ty_generics,
        where_clause,
        variants,
        fields,
    } = data;

    let iter = quote!(#root::iter);
    let trait_ = quote!(#iter::ExactSizeIterator);
    let fst = &fields[0];

    let where_clause = fields
        .iter()
        .skip(1)
        .fold(quote!(#where_clause #fst: #trait_,), |t, f| {
            t.extend_and_return(quote!(#f: #trait_<Item = <#fst as #iter::Iterator>::Item>,))
        });

    // methods
    #[cfg(not(feature = "exact_size_is_empty"))]
    let is_empty = TokenStream::new();
    #[cfg(feature = "exact_size_is_empty")]
    let is_empty = {
        let is_empty = variants.iter().fold(TokenStream::new(), |t, v| {
            t.extend_and_return(quote!(#v(x) => #trait_::is_empty(x),))
        });
        quote! {
            #[inline]
            fn is_empty(&self) -> bool {
                match self { #is_empty }
            }
        }
    };

    quote! {
        impl #impl_generics #trait_ for #name #ty_generics #where_clause {
            #[inline]
            fn len(&self) -> usize {
                match self { #(#variants(x) => x.len(),)* }
            }
            #is_empty
        }
    }
}
