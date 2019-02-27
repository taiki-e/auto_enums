use proc_macro2::TokenStream;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["Fn"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let trait_path = quote!(::core::ops::Fn);
    let trait_ = quote!(#trait_path(__T) -> __U);
    let fst = data.fields().iter().next();

    let mut impls = data.impl_with_capacity(1)?;

    *impls.trait_() = Some(Trait::new(
        syn::parse2(trait_path.clone())?,
        parse_quote!(#trait_path<(__T,)>)?,
    ));
    impls.push_generic_param(param_ident("__T"));
    impls.push_generic_param(param_ident("__U"));

    impls.push_where_predicate(parse_quote!(#fst: #trait_)?);
    data.fields()
        .iter()
        .skip(1)
        .try_for_each(|f| parse_quote!(#f: #trait_).map(|f| impls.push_where_predicate(f)))?;

    impls.push_method(parse_quote! {
        #[inline]
        extern "rust-call" fn call(&self, args: (__T,)) -> Self::Output;
    }?)?;

    Ok(impls.build())
}
