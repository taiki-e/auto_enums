use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Generator"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    derive_trait(data, parse_quote!(::core::ops::Generator), None, parse_quote! {
        trait Generator<R> {
            type Yield;
            type Return;
            #[inline]
            fn resume(
                self: ::core::pin::Pin<&mut Self>,
                arg: R,
            ) -> ::core::ops::GeneratorState<Self::Yield, Self::Return>;
        }
    })
}
