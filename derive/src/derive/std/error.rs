use proc_macro2::TokenStream;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Error"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
    let trait_ = parse_quote!(#root::error::Error)?;

    let ident = data.ident();
    let source = data
        .variants()
        .iter()
        .map(|v| quote!(#ident::#v(x) => #root::option::Option::Some(x)));

    let source = parse_quote! {
        fn source(&self) -> #root::option::Option<&(dyn (#trait_) + 'static)> {
            match self { #(#source,)* }
        }
    }?;

    let mut impls = data.impl_trait_with_capacity(
        2,
        trait_,
        None,
        parse_quote! {
            trait Error {
                fn description(&self) -> &str;
            }
        }?,
    )?;

    data.fields()
        .iter()
        .try_for_each(|f| parse_quote!(#f: 'static).map(|f| impls.push_where_predicate(f)))?;
    impls.push_item(source);

    Ok(impls.build())
}
