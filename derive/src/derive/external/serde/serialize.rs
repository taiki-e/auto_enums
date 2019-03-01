use crate::utils::*;

pub(crate) const NAME: &[&str] = &["serde::Serialize"];

pub(crate) fn derive(data: &Data, stack: &mut Stack<ItemImpl>) -> Result<()> {
    let ser = quote!(::serde::ser);

    derive_trait!(
        data,
        parse_quote!(#ser::Serialize)?,
        parse_quote! {
            trait Serialize {
                #[inline]
                fn serialize<__S>(&self, serializer: __S) -> ::core::result::Result<__S::Ok, __S::Error>
                where
                    __S: #ser::Serializer;
            }
        }?,
    )
    .map(|item| stack.push(item))
}
