use crate::utils::*;

pub(crate) const NAME: &[&str] = &["TrustedLen"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    derive_trait(
        data,
        parse_quote!(::core::iter::TrustedLen),
        Some(format_ident!("Item")),
        parse_quote! {
            unsafe trait TrustedLen: ::core::iter::Iterator {}
        },
    )
}
