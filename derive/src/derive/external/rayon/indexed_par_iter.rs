use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["rayon::IndexedParallelIterator"];

pub(crate) fn enum_derive(data: &syn::ItemEnum) -> Result<TokenStream> {
    EnumData::parse(data, false, true).map(|data| indexed_parallel_iterator(&data, &std_root()))
}

fn indexed_parallel_iterator(data: &EnumData<'_>, _root: &TokenStream) -> TokenStream {
    let EnumData {
        name,
        impl_generics,
        ty_generics,
        where_clause,
        variants,
        fields,
    } = data;

    let iter = quote!(::rayon::iter);
    let trait_ = quote!(#iter::IndexedParallelIterator);
    let fst = &fields[0];

    let where_clause = fields
        .iter()
        .skip(1)
        .fold(quote!(#where_clause #fst: #trait_,), |t, f| {
            t.extend_and_return(
                quote!(#f: #trait_<Item = <#fst as #iter::ParallelIterator>::Item>,),
            )
        });

    quote! {
        impl #impl_generics #trait_ for #name #ty_generics #where_clause {
            #[inline]
            fn drive<__C>(self, consumer: __C) -> __C::Result
            where
                __C: #iter::plumbing::Consumer<Self::Item>
            {
                match self { #(#variants(x) => x.drive(consumer),)* }
            }
            #[inline]
            fn len(&self) -> usize {
                match self { #(#variants(x) => x.len(),)* }
            }
            #[inline]
            fn with_producer<__CB>(self, callback: __CB) -> __CB::Output
            where
                __CB: #iter::plumbing::ProducerCallback<Self::Item>
            {
                match self { #(#variants(x) => x.with_producer(callback),)* }
            }
        }
    }
}
