macro_rules! fmt_impl {
    ($trait:ident, $Trait:ident, [$($name:expr),*]) => {
        pub(crate) mod $trait {
            use proc_macro2::TokenStream;
            use quote::quote;

            use crate::utils::*;

            pub(crate) const NAME: &[&str] = &[$($name),*];

            pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
                let root = std_root();
                let fmt = quote!(#root::fmt);

                data.impl_trait_with_capacity(
                    1,
                    syn::parse2(quote!(#fmt::$Trait))?,
                    None,
                    syn::parse2(quote! {
                        trait $Trait {
                            #[inline]
                            fn fmt(&self, f: &mut #fmt::Formatter<'_>) -> #fmt::Result;
                        }
                    })?,
                )
                .map(build)
            }
        }
    };
}

pub(crate) mod write;

fmt_impl!(debug, Debug, ["Debug", "fmt::Debug"]);
fmt_impl!(display, Display, ["Display", "fmt::Display"]);
fmt_impl!(binary, Binary, ["fmt::Binary"]);
fmt_impl!(lower_exp, LowerExp, ["fmt::LowerExp"]);
fmt_impl!(lower_hex, LowerHex, ["fmt::LowerHex"]);
fmt_impl!(octal, Octal, ["fmt::Octal"]);
fmt_impl!(pointer, Pointer, ["fmt::Pointer"]);
fmt_impl!(upper_exp, UpperExp, ["fmt::UpperExp"]);
fmt_impl!(upper_hex, UpperHex, ["fmt::UpperHex"]);
