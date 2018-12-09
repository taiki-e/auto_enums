use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["DoubleEndedIterator"];

pub(crate) fn enum_derive(data: &syn::ItemEnum) -> Result<TokenStream> {
    EnumData::parse(data, false, true).map(|data| double_ended_iterator(&data, &std_root()))
}

fn double_ended_iterator(data: &EnumData<'_>, root: &TokenStream) -> TokenStream {
    let EnumData {
        name,
        impl_generics,
        ty_generics,
        where_clause,
        variants,
        fields,
    } = data;

    let iter = quote!(#root::iter);
    let trait_ = quote!(#iter::DoubleEndedIterator);
    let fst = &fields[0];

    let where_clause = fields
        .iter()
        .skip(1)
        .fold(quote!(#where_clause #fst: #trait_,), |t, f| {
            t.extend_and_return(quote!(#f: #trait_<Item = <#fst as #iter::Iterator>::Item>,))
        });

    // methods
    #[cfg(feature = "try_trait")]
    let try_rfold = quote! {
        #[inline]
        fn try_rfold<__U, __F, __R>(&mut self, init: __U, f: __F) -> __R
        where
            __F: #root::ops::FnMut(__U, Self::Item) -> __R,
            __R: #root::ops::Try<Ok = __U>
        {
            match self { #(#variants(x) => x.try_rfold(init, f),)* }
        }
    };
    // It is equally efficient if `try_rfold` can be used.
    #[cfg(not(feature = "try_trait"))]
    let try_rfold = quote! {
        #[inline]
        fn rfold<__U, __F>(self, accum: __U, f: __F) -> __U
        where
            __F: #root::ops::FnMut(__U, Self::Item) -> __U,
        {
            match self { #(#variants(x) => x.rfold(accum, f),)* }
        }
        #[inline]
        fn rfind<__P>(&mut self, predicate: __P) -> #root::option::Option<Self::Item>
        where
            __P: #root::ops::FnMut(&Self::Item) -> bool,
        {
            match self { #(#variants(x) => x.rfind(predicate),)* }
        }
    };

    quote! {
        impl #impl_generics #trait_ for #name #ty_generics #where_clause {
            #[inline]
            fn next_back(&mut self) -> #root::option::Option<Self::Item> {
                match self { #(#variants(x) => x.next_back(),)* }
            }
            #try_rfold
        }
    }
}
