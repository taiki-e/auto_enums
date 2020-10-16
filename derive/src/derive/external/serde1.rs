pub(crate) mod serialize {
    use crate::utils::*;

    pub(crate) const NAME: &[&str] = &["serde::Serialize"];

    pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
        derive_trait(data, parse_quote!(::serde::ser::Serialize), None, parse_quote! {
            trait Serialize {
                #[inline]
                fn serialize<__S>(
                    &self,
                    serializer: __S,
                ) -> ::core::result::Result<__S::Ok, __S::Error>
                where
                    __S: ::serde::ser::Serializer;
            }
        })
    }
}
