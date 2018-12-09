macro_rules! fmt_impl {
    ($trait:ident, $Trait:ident, [$($name:expr),*]) => {
        pub(crate) mod $trait {
            use proc_macro2::TokenStream;
            use quote::quote;

            use crate::utils::*;

            pub(crate) const NAME: &[&str] = &[$($name),*];

            pub(crate) fn enum_derive(data: &syn::ItemEnum) -> Result<TokenStream> {
                EnumData::parse(data, false, true).map(|data| fmt(&data, &std_root()))
            }

            fn fmt(data: &EnumData<'_>, root: &TokenStream) -> TokenStream {
                let EnumData {
                    name,
                    impl_generics,
                    ty_generics,
                    where_clause,
                    variants,
                    fields,
                } = data;

                let fmt = quote!(#root::fmt);
                let trait_ = quote!(#fmt::$Trait);

                let where_clause = fields.iter().fold(where_clause.clone(), |t, f| {
                    t.extend_and_return(quote!(#f: #trait_,))
                });

                // method
                let f = variants.iter().fold(TokenStream::new(), |t, v| {
                    t.extend_and_return(quote!(#v(x) => #trait_::fmt(x, f),))
                });

                quote! {
                    impl #impl_generics #trait_ for #name #ty_generics #where_clause {
                        #[inline]
                        fn fmt(&self, f: &mut #fmt::Formatter<'_>) -> #fmt::Result {
                            match self { #f }
                        }
                    }
                }
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
