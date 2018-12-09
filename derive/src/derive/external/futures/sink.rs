use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["futures::Sink"];

pub(crate) fn enum_derive(data: &syn::ItemEnum) -> Result<TokenStream> {
    EnumData::parse(data, false, true).map(|data| sink(&data, &std_root()))
}

fn sink(data: &EnumData<'_>, root: &TokenStream) -> TokenStream {
    let EnumData {
        name,
        impl_generics,
        ty_generics,
        where_clause,
        variants,
        fields,
    } = data;

    let trait_ = quote!(::futures::sink::Sink);
    let pin = quote!(#root::pin::Pin);
    let fst = &fields[0];

    let where_clause = fields
        .iter()
        .skip(1)
        .fold(quote!(#where_clause #fst: #trait_,), |t, f| {
            t.extend_and_return(quote!(#f: #trait_<SinkItem = <#fst as #trait_>::SinkItem, SinkError = <#fst as #trait_>::SinkError>,))
        });

    // methods
    let poll_ready = variants.iter().fold(TokenStream::new(), |t, v| {
        t.extend_and_return(quote!(#v(x) => #pin::new_unchecked(x).poll_ready(lw),))
    });
    let start_send = variants.iter().fold(TokenStream::new(), |t, v| {
        t.extend_and_return(quote!(#v(x) => #pin::new_unchecked(x).start_send(item),))
    });
    let poll_flush = variants.iter().fold(TokenStream::new(), |t, v| {
        t.extend_and_return(quote!(#v(x) => #pin::new_unchecked(x).poll_flush(lw),))
    });
    let poll_close = variants.iter().fold(TokenStream::new(), |t, v| {
        t.extend_and_return(quote!(#v(x) => #pin::new_unchecked(x).poll_close(lw),))
    });

    quote! {
        impl #impl_generics #trait_ for #name #ty_generics #where_clause {
            type SinkItem = <#fst as #trait_>::SinkItem;
            type SinkError = <#fst as #trait_>::SinkError;
            #[allow(unsafe_code)]
            #[inline]
            fn poll_ready(self: #pin<&mut Self>, lw: &#root::task::LocalWaker) -> #root::task::Poll<#root::result::Result<(), Self::SinkError>> {
                unsafe {
                    match #pin::get_mut_unchecked(self) { #poll_ready }
                }
            }
            #[allow(unsafe_code)]
            #[inline]
            fn start_send(self: #pin<&mut Self>, item: Self::SinkItem) -> #root::result::Result<(), Self::SinkError> {
                unsafe {
                    match #pin::get_mut_unchecked(self) { #start_send }
                }
            }
            #[allow(unsafe_code)]
            #[inline]
            fn poll_flush(self: #pin<&mut Self>, lw: &#root::task::LocalWaker) -> #root::task::Poll<#root::result::Result<(), Self::SinkError>> {
                unsafe {
                    match #pin::get_mut_unchecked(self) { #poll_flush }
                }
            }
            #[allow(unsafe_code)]
            #[inline]
            fn poll_close(self: #pin<&mut Self>, lw: &#root::task::LocalWaker) -> #root::task::Poll<#root::result::Result<(), Self::SinkError>> {
                unsafe {
                    match #pin::get_mut_unchecked(self) { #poll_close }
                }
            }
        }
    }
}
