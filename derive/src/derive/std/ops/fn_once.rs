use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["FnOnce"];

pub(crate) fn enum_derive(data: &syn::ItemEnum) -> Result<TokenStream> {
    EnumData::parse(data, true, true).map(|data| fn_once(&data, &std_root()))
}

fn fn_once(data: &EnumData<'_>, root: &TokenStream) -> TokenStream {
    let EnumData {
        name,
        impl_generics,
        ty_generics,
        where_clause,
        variants,
        fields,
    } = data;

    let trait_ = quote!(#root::ops::FnOnce);
    let impl_generics = quote!(#impl_generics __T, __U>);

    let where_clause = fields.iter().fold(where_clause.clone(), |t, f| {
        t.extend_and_return(quote!(#f: #trait_(__T) -> __U,))
    });

    quote! {
        impl #impl_generics #trait_<(__T,)> for #name #ty_generics #where_clause {
            type Output = __U;
            #[inline]
            extern "rust-call" fn call_once(self, args: (__T,)) -> Self::Output {
                match self { #(#variants(f) => f.call_once(args),)* }
            }
        }
    }
}
