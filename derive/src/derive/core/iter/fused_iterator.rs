use crate::utils::*;

pub(crate) const NAME: &[&str] = &["FusedIterator"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    derive_trait(
        data,
        parse_quote!(::core::iter::FusedIterator),
        Some(format_ident!("Item")),
        parse_quote! {
            trait FusedIterator: ::core::iter::Iterator {}
        },
    )
}
