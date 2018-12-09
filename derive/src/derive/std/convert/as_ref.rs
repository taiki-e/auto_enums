use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["AsRef"];

pub(crate) fn enum_derive(data: &syn::ItemEnum) -> Result<TokenStream> {
    EnumData::parse(data, true, true).map(|data| as_ref(&data, &std_root()))
}

fn as_ref(data: &EnumData<'_>, root: &TokenStream) -> TokenStream {
    let EnumData {
        name,
        impl_generics,
        ty_generics,
        where_clause,
        variants,
        fields,
    } = data;

    let trait_ = quote!(#root::convert::AsRef);
    let impl_generics = quote!(#impl_generics __T: ?Sized>);

    let where_clause = fields.iter().fold(where_clause.clone(), |t, f| {
        t.extend_and_return(quote!(#f: #trait_<__T>,))
    });

    quote! {
        impl #impl_generics #trait_<__T> for #name #ty_generics #where_clause {
            #[inline]
            fn as_ref(&self) -> &__T {
                match self { #(#variants(x) => x.as_ref(),)* }
            }
        }
    }
}
