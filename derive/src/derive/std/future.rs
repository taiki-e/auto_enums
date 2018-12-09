use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Future"];

pub(crate) fn enum_derive(data: &syn::ItemEnum) -> Result<TokenStream> {
    EnumData::parse(data, false, true).map(|data| future(&data, &std_root()))
}

fn future(data: &EnumData<'_>, root: &TokenStream) -> TokenStream {
    let EnumData {
        name,
        impl_generics,
        ty_generics,
        where_clause,
        variants,
        fields,
    } = data;

    let trait_ = quote!(#root::future::Future);
    let pin = quote!(#root::pin::Pin);
    let fst = &fields[0];

    let where_clause = fields
        .iter()
        .skip(1)
        .fold(quote!(#where_clause #fst: #trait_,), |t, f| {
            t.extend_and_return(quote!(#f: #trait_<Output = <#fst as #trait_>::Output>,))
        });

    // method
    let poll = variants.iter().fold(TokenStream::new(), |t, v| {
        t.extend_and_return(quote!(#v(x) => #pin::new_unchecked(x).poll(lw),))
    });

    quote! {
        impl #impl_generics #trait_ for #name #ty_generics #where_clause {
            type Output = <#fst as #trait_>::Output;
            #[allow(unsafe_code)]
            #[inline]
            fn poll(self: #pin<&mut Self>, lw: &#root::task::LocalWaker) -> #root::task::Poll<Self::Output> {
                unsafe {
                    match #pin::get_mut_unchecked(self) { #poll }
                }
            }
        }
    }
}
