use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Generator"];

pub(crate) fn derive(data: &Data, stack: &mut Stack<ItemImpl>) -> Result<()> {
    derive_trait!(
        data,
        parse_quote!(::core::ops::Generator)?,
        parse_quote! {
            trait Generator {
                type Yield;
                type Return;
                #[inline]
                fn resume(self: ::core::pin::Pin<&mut Self>) -> ::core::ops::GeneratorState<Self::Yield, Self::Return>;
            }
        }?,
    )
    .map(|item| stack.push(item))
}
