// SPDX-License-Identifier: Apache-2.0 OR MIT

pub(crate) mod serialize {
    use crate::derive::prelude::*;

    pub(crate) const NAME: &[&str] = &["serde_core::Serialize"];

    pub(crate) fn derive(_cx: &Context, data: &Data) -> Result<TokenStream> {
        Ok(derive_trait(data, &parse_quote!(::serde_core::ser::Serialize), None, parse_quote! {
            trait Serialize {
                #[inline]
                fn serialize<__S>(
                    &self,
                    serializer: __S,
                ) -> ::core::result::Result<__S::Ok, __S::Error>
                where
                    __S: ::serde_core::ser::Serializer;
            }
        }))
    }
}
