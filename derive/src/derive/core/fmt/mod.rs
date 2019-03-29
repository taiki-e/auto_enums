macro_rules! fmt_impl {
    ($trait:ident, $Trait:ident, [$($name:expr),*]) => {
        pub(crate) mod $trait {
            use crate::utils::*;

            pub(crate) const NAME: &[&str] = &[$($name),*];

            pub(crate) fn derive(data: &Data, stack: &mut Stack<ItemImpl>) -> Result<()> {
                let fmt = quote!(::core::fmt);

                derive_trait!(
                    data,
                    parse_quote!(#fmt::$Trait)?,
                    parse_quote! {
                        trait $Trait {
                            #[inline]
                            fn fmt(&self, f: &mut #fmt::Formatter<'_>) -> #fmt::Result;
                        }
                    }?,
                )
                .map(|item| stack.push(item))
            }
        }
    };
}

pub(crate) mod write;

fmt_impl!(debug, Debug, ["Debug", "fmt::Debug"]);
fmt_impl!(display, Display, ["Display", "fmt::Display"]);
#[cfg(feature = "fmt")]
fmt_impl!(binary, Binary, ["fmt::Binary"]);
#[cfg(feature = "fmt")]
fmt_impl!(lower_exp, LowerExp, ["fmt::LowerExp"]);
#[cfg(feature = "fmt")]
fmt_impl!(lower_hex, LowerHex, ["fmt::LowerHex"]);
#[cfg(feature = "fmt")]
fmt_impl!(octal, Octal, ["fmt::Octal"]);
#[cfg(feature = "fmt")]
fmt_impl!(pointer, Pointer, ["fmt::Pointer"]);
#[cfg(feature = "fmt")]
fmt_impl!(upper_exp, UpperExp, ["fmt::UpperExp"]);
#[cfg(feature = "fmt")]
fmt_impl!(upper_hex, UpperHex, ["fmt::UpperHex"]);
