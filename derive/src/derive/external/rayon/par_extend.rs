use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["rayon::ParallelExtend"];

pub(crate) fn enum_derive(data: &syn::ItemEnum) -> Result<TokenStream> {
    EnumData::parse(data, true, true).map(|data| parallel_extend(&data, &std_root()))
}

fn parallel_extend(data: &EnumData<'_>, _root: &TokenStream) -> TokenStream {
    let EnumData {
        name,
        impl_generics,
        ty_generics,
        where_clause,
        variants,
        fields,
    } = data;

    let iter = quote!(::rayon::iter);
    let trait_ = quote!(#iter::ParallelExtend);
    let impl_generics = quote!(#impl_generics __T: Send>);

    let where_clause = fields.iter().fold(where_clause.clone(), |t, f| {
        t.extend_and_return(quote!(#f: #trait_<__T>,))
    });

    quote! {
        impl #impl_generics #trait_<__T> for #name #ty_generics #where_clause {
            #[inline]
            fn par_extend<__I>(&mut self, par_iter: __I)
            where
                __I: #iter::IntoParallelIterator<Item = __T>
            {
                match self { #(#variants(x) => x.par_extend(par_iter),)* }
            }
        }
    }
}
