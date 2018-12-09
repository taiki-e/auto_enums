use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["rayon::ParallelIterator"];

pub(crate) fn enum_derive(data: &syn::ItemEnum) -> Result<TokenStream> {
    EnumData::parse(data, false, true).map(|data| parallel_iterator(&data, &std_root()))
}

fn parallel_iterator(data: &EnumData<'_>, root: &TokenStream) -> TokenStream {
    let EnumData {
        name,
        impl_generics,
        ty_generics,
        where_clause,
        variants,
        fields,
    } = data;

    let iter = quote!(::rayon::iter);
    let trait_ = quote!(#iter::ParallelIterator);
    let fst = &fields[0];

    let where_clause = fields
        .iter()
        .skip(1)
        .fold(quote!(#where_clause #fst: #trait_,), |t, f| {
            t.extend_and_return(quote!(#f: #trait_<Item = <#fst as #trait_>::Item>,))
        });

    quote! {
        impl #impl_generics #trait_ for #name #ty_generics #where_clause {
            type Item = <#fst as #trait_>::Item;
            #[inline]
            fn drive_unindexed<__C>(self, consumer: __C) -> __C::Result
            where
                __C: #iter::plumbing::UnindexedConsumer<Self::Item>
            {
                match self { #(#variants(x) => x.drive_unindexed(consumer),)* }
            }
            #[inline]
            fn opt_len(&self) -> #root::option::Option<usize> {
                match self { #(#variants(x) => x.opt_len(),)* }
            }
        }
    }
}
