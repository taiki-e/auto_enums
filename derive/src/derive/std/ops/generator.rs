use proc_macro2::TokenStream;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Generator"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();

    derive_trait!(
        data,
        parse_quote!(#root::ops::Generator)?,
        parse_quote! {
            trait Generator {
                type Yield;
                type Return;
                #[inline]
                fn resume(self: #root::pin::Pin<&mut Self>) -> #root::ops::GeneratorState<Self::Yield, Self::Return>;
            }
        }?,
    )
}
