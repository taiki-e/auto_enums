use crate::utils::*;

pub(crate) const NAME: &[&str] = &["ExactSizeIterator"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    // TODO: When `exact_size_is_empty` stabilized, add `is_empty` conditionally.

    derive_trait(
        data,
        parse_quote!(::core::iter::ExactSizeIterator),
        Some(format_ident!("Item")),
        parse_quote! {
            trait ExactSizeIterator: ::core::iter::Iterator {
                #[inline]
                fn len(&self) -> usize;
            }
        },
    )
}
