// SPDX-License-Identifier: Apache-2.0 OR MIT

macro_rules! derive_fmt {
    ($trait:ident, $Trait:ident, [$($name:expr),*]) => {
        pub(crate) mod $trait {
            use crate::derive::*;

            pub(crate) const NAME: &[&str] = &[$($name),*];

            pub(crate) fn derive(_cx: &Context, data: &Data) -> Result<TokenStream> {
                Ok(derive_trait(data, &parse_quote!(::core::fmt::$Trait), None, parse_quote! {
                    trait $Trait {
                        #[inline]
                        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result;
                    }
                }))
            }
        }
    };
}

derive_fmt!(debug, Debug, ["Debug", "fmt::Debug"]);
derive_fmt!(display, Display, ["Display", "fmt::Display"]);

#[cfg(feature = "fmt")]
derive_fmt!(binary, Binary, ["fmt::Binary"]);
#[cfg(feature = "fmt")]
derive_fmt!(lower_exp, LowerExp, ["fmt::LowerExp"]);
#[cfg(feature = "fmt")]
derive_fmt!(lower_hex, LowerHex, ["fmt::LowerHex"]);
#[cfg(feature = "fmt")]
derive_fmt!(octal, Octal, ["fmt::Octal"]);
#[cfg(feature = "fmt")]
derive_fmt!(pointer, Pointer, ["fmt::Pointer"]);
#[cfg(feature = "fmt")]
derive_fmt!(upper_exp, UpperExp, ["fmt::UpperExp"]);
#[cfg(feature = "fmt")]
derive_fmt!(upper_hex, UpperHex, ["fmt::UpperHex"]);

pub(crate) mod write {
    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["fmt::Write"];

    pub(crate) fn derive(_cx: &Context, data: &Data) -> Result<TokenStream> {
        Ok(derive_trait(data, &parse_quote!(::core::fmt::Write), None, parse_quote! {
            trait Write {
                #[inline]
                fn write_str(&mut self, s: &str) -> ::core::fmt::Result;
                #[inline]
                fn write_char(&mut self, c: char) -> ::core::fmt::Result;
                #[inline]
                fn write_fmt(&mut self, args: ::core::fmt::Arguments<'_>) -> ::core::fmt::Result;
            }
        }))
    }
}
