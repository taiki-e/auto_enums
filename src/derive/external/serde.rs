// SPDX-License-Identifier: Apache-2.0 OR MIT

pub(crate) mod serialize {
    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["serde::Serialize"];

    pub(crate) fn derive(_cx: &Context, data: &Data) -> Result<TokenStream> {
        Ok(derive_trait(data, &parse_quote!(::serde::ser::Serialize), None, parse_quote! {
            trait Serialize {
                #[inline]
                fn serialize<__S>(
                    &self,
                    serializer: __S,
                ) -> ::core::result::Result<__S::Ok, __S::Error>
                where
                    __S: ::serde::ser::Serializer;
            }
        }))
    }
}
