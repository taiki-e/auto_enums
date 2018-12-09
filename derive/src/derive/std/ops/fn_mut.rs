use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["FnMut"];

pub(crate) fn enum_derive(data: &syn::ItemEnum) -> Result<TokenStream> {
    EnumData::parse(data, true, true).map(|data| fn_mut(&data, &std_root()))
}

fn fn_mut(data: &EnumData<'_>, root: &TokenStream) -> TokenStream {
    let EnumData {
        name,
        impl_generics,
        ty_generics,
        where_clause,
        variants,
        fields,
    } = data;

    let trait_ = quote!(#root::ops::FnMut);
    let impl_generics = quote!(#impl_generics __T, __U>);

    let where_clause = fields.iter().fold(quote!(#where_clause), |t, f| {
        t.extend_and_return(quote!(#f: #trait_(__T) -> __U,))
    });

    quote! {
        impl #impl_generics #trait_<(__T,)> for #name #ty_generics #where_clause {
            #[inline]
            extern "rust-call" fn call_mut(&mut self, args: (__T,)) -> Self::Output {
                match self { #(#variants(f) => f.call_mut(args),)* }
            }
        }
    }
}
