use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["futures01::Sink"];

pub(crate) fn enum_derive(data: &syn::ItemEnum) -> Result<TokenStream> {
    EnumData::parse(data, false, true).map(|data| sink(&data, &std_root()))
}

fn sink(data: &EnumData<'_>, _root: &TokenStream) -> TokenStream {
    let EnumData {
        name,
        impl_generics,
        ty_generics,
        where_clause,
        variants,
        fields,
    } = data;

    let crate_ = quote!(::futures);
    let trait_ = quote!(#crate_::sink::Sink);
    let fst = &fields[0];

    let where_clause = fields
        .iter()
        .skip(1)
        .fold(quote!(#where_clause #fst: #trait_,), |t, f| {
            t.extend_and_return(quote!(#f: #trait_<SinkItem = <#fst as #trait_>::SinkItem, SinkError = <#fst as #trait_>::SinkError>,))
        });

    quote! {
        impl #impl_generics #trait_ for #name #ty_generics #where_clause {
            type SinkItem = <#fst as #trait_>::SinkItem;
            type SinkError = <#fst as #trait_>::SinkError;
            #[inline]
            fn start_send(&mut self, item: Self::SinkItem) -> #crate_::StartSend<Self::SinkItem, Self::SinkError> {
                match self { #(#variants(x) => x.start_send(item),)* }
            }
            #[inline]
            fn poll_complete(&mut self) -> #crate_::Poll<(), Self::SinkError> {
                match self { #(#variants(x) => x.poll_complete(),)* }
            }
            #[inline]
            fn close(&mut self) -> #crate_::Poll<(), Self::SinkError> {
                match self { #(#variants(x) => x.close(),)* }
            }
        }
    }
}
