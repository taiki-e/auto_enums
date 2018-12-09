use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Iterator"];

pub(crate) fn enum_derive(data: &syn::ItemEnum) -> Result<TokenStream> {
    EnumData::parse(data, false, true).map(|data| iterator(&data, &std_root()))
}

fn iterator(data: &EnumData<'_>, root: &TokenStream) -> TokenStream {
    let EnumData {
        name,
        impl_generics,
        ty_generics,
        where_clause,
        variants,
        fields,
    } = data;

    let iter = quote!(#root::iter);
    let trait_ = quote!(#iter::Iterator);
    let fst = &fields[0];

    let where_clause = fields
        .iter()
        .skip(1)
        .fold(quote!(#where_clause #fst: #trait_,), |t, f| {
            t.extend_and_return(quote!(#f: #trait_<Item = <#fst as #trait_>::Item>,))
        });

    // methods
    #[cfg(feature = "try_trait")]
    let try_fold = quote! {
        #[inline]
        fn try_fold<__U, __F, __R>(&mut self, init: __U, f: __F) -> __R
        where
            __F: #root::ops::FnMut(__U, Self::Item) -> __R,
            __R: #root::ops::Try<Ok = __U>
        {
            match self { #(#variants(x) => x.try_fold(init, f),)* }
        }
    };
    // It is equally efficient if `try_fold` can be used.
    #[cfg(not(feature = "try_trait"))]
    let try_fold = quote! {
        #[inline]
        fn fold<__U, __F>(self, init: __U, f: __F) -> __U
        where
            __F: #root::ops::FnMut(__U, Self::Item) -> __U,
        {
            match self { #(#variants(x) => x.fold(init, f),)* }
        }
        #[inline]
        fn all<__F>(&mut self, f: __F) -> bool
        where
            __F: #root::ops::FnMut(Self::Item) -> bool
        {
            match self { #(#variants(x) => x.all(f),)* }
        }
        #[inline]
        fn any<__F>(&mut self, f: __F) -> bool
        where
            __F: #root::ops::FnMut(Self::Item) -> bool
        {
            match self { #(#variants(x) => x.any(f),)* }
        }
        #[inline]
        fn find<__P>(&mut self, predicate: __P) -> #root::option::Option<Self::Item>
        where
            __P: #root::ops::FnMut(&Self::Item) -> bool,
        {
            match self { #(#variants(x) => x.find(predicate),)* }
        }
        #[inline]
        fn find_map<__U, __F>(&mut self, f: __F) -> #root::option::Option<__U>
        where
            __F: #root::ops::FnMut(Self::Item) -> #root::option::Option<__U>,
        {
            match self { #(#variants(x) => x.find_map(f),)* }
        }
        #[inline]
        fn position<__P>(&mut self, predicate: __P) -> #root::option::Option<usize>
        where
            __P: #root::ops::FnMut(Self::Item) -> bool,
        {
            match self { #(#variants(x) => x.position(predicate),)* }
        }
    };

    quote! {
        impl #impl_generics #trait_ for #name #ty_generics #where_clause {
            type Item = <#fst as #trait_>::Item;
            #[inline]
            fn next(&mut self) -> #root::option::Option<Self::Item> {
                match self { #(#variants(x) => x.next(),)* }
            }
            #[inline]
            fn size_hint(&self) -> (usize, #root::option::Option<usize>) {
                match self { #(#variants(x) => x.size_hint(),)* }
            }
            #[inline]
            fn count(self) -> usize {
                match self { #(#variants(x) => x.count(),)* }
            }
            #[inline]
            fn last(self) -> #root::option::Option<Self::Item> {
                match self { #(#variants(x) => x.last(),)* }
            }
            #[inline]
            fn nth(&mut self, n: usize) -> #root::option::Option<Self::Item> {
                match self { #(#variants(x) => x.nth(n),)* }
            }
            #[inline]
            #[must_use = "if you really need to exhaust the iterator, consider `.for_each(drop)` instead"]
            fn collect<__U: #iter::FromIterator<Self::Item>>(self) -> __U {
                match self { #(#variants(x) => x.collect(),)* }
            }
            #[inline]
            fn partition<__U, __F>(self, f: __F) -> (__U, __U)
            where
                __U: #root::default::Default + #iter::Extend<Self::Item>,
                __F: #root::ops::FnMut(&Self::Item) -> bool
            {
                match self { #(#variants(x) => x.partition(f),)* }
            }
            #try_fold
        }
    }
}
