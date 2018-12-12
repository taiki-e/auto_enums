use proc_macro2::TokenStream;
use quote::quote;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["FnOnce"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let root = std_root();
    let trait_path = quote!(#root::ops::FnOnce);
    let trait_ = quote!(#trait_path(__T) -> __U);
    let fst = data.fields().iter().next();

    let mut impls = data.impl_with_capacity(2, root)?;

    *impls.trait_() = Some(Trait::new(
        syn::parse2(trait_path.clone())?,
        syn::parse2(quote!(#trait_path<(__T,)>))?,
    ));
    impls.push_generic_param(param_ident("__T"));
    impls.push_generic_param(param_ident("__U"));

    impls.push_where_predicate(syn::parse2(quote!(#fst: #trait_))?);
    data.fields().iter().skip(1).try_for_each(|f| {
        syn::parse2(quote!(#f: #trait_)).map(|f| impls.push_where_predicate(f))
    })?;

    impls.append_items_from_trait(syn::parse2(quote! {
        trait FnOnce {
            type Output;
            #[inline]
            extern "rust-call" fn call_once(self, args: (__T,)) -> Self::Output;
        }
    })?)?;

    Ok(impls.build())
}
