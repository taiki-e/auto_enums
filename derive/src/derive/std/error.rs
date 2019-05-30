use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Error"];

pub(crate) fn derive(data: &Data, items: &mut Vec<ItemImpl>) -> Result<()> {
    let ident = data.ident();
    let source =
        data.variants().iter().map(|v| quote!(#ident::#v(x) => ::std::option::Option::Some(x)));

    let source = parse_quote! {
        fn source(&self) -> ::std::option::Option<&(dyn (::std::error::Error) + 'static)> {
            match self { #(#source,)* }
        }
    }?;

    let mut impl_ = data.impl_trait_with_capacity(
        2,
        parse_quote!(::std::error::Error)?,
        None,
        parse_quote! {
            trait Error {
                fn description(&self) -> &str;
            }
        }?,
    )?;

    data.fields()
        .iter()
        .try_for_each(|f| parse_quote!(#f: 'static).map(|f| impl_.push_where_predicate(f)))?;
    impl_.push_item(source);

    items.push(impl_.build_item());
    Ok(())
}
