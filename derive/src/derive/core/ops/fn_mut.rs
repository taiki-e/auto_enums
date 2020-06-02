use derive_utils::EnumImpl;
use syn::TypeParam;

use crate::utils::*;

pub(crate) const NAME: &[&str] = &["FnMut"];

pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
    let trait_path = quote!(::core::ops::FnMut);
    let trait_ = quote!(#trait_path(__T) -> __U);
    let fst = data.field_types().next();
    let mut impl_ = EnumImpl::new(data);

    impl_.set_trait(parse_quote!(#trait_path<(__T,)>));
    impl_.push_generic_param(TypeParam::from(format_ident!("__T")).into());
    impl_.push_generic_param(TypeParam::from(format_ident!("__U")).into());

    impl_.push_where_predicate(parse_quote!(#fst: #trait_));
    data.field_types().skip(1).for_each(|f| impl_.push_where_predicate(parse_quote!(#f: #trait_)));

    impl_.push_method(parse_quote! {
        #[inline]
        extern "rust-call" fn call_mut(&mut self, args: (__T,)) -> Self::Output;
    })?;

    Ok(impl_.build())
}
