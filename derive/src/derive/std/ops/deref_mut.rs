use proc_macro2::TokenStream;
use quote::quote;
use smallvec::smallvec;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["DerefMut"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
    let ops = quote!(#root::ops);

    data.impl_trait_with_capacity(
        1,
        root,
        syn::parse2(quote!(#ops::DerefMut))?,
        smallvec![ident_call_site("Target")],
        syn::parse2(quote! {
            trait DerefMut: #ops::Deref {
                #[inline]
                fn deref_mut(&mut self) -> &mut Self::Target;
            }
        })?,
    )
    .map(build)
}
