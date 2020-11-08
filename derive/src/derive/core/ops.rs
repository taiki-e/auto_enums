#[cfg(feature = "ops")]
pub(crate) mod deref {
    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["Deref"];

    pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
        Ok(derive_trait(data, parse_quote!(::core::ops::Deref), None, parse_quote! {
            trait Deref {
                type Target;
                #[inline]
                fn deref(&self) -> &Self::Target;
            }
        }))
    }
}

#[cfg(feature = "ops")]
pub(crate) mod deref_mut {
    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["DerefMut"];

    pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
        Ok(derive_trait(
            data,
            parse_quote!(::core::ops::DerefMut),
            Some(format_ident!("Target")),
            parse_quote! {
                trait DerefMut: ::core::ops::Deref {
                    #[inline]
                    fn deref_mut(&mut self) -> &mut Self::Target;
                }
            },
        ))
    }
}

#[cfg(feature = "ops")]
pub(crate) mod index {
    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["Index"];

    pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
        Ok(derive_trait(data, parse_quote!(::core::ops::Index), None, parse_quote! {
            trait Index<__Idx> {
                type Output;
                #[inline]
                fn index(&self, index: __Idx) -> &Self::Output;
            }
        }))
    }
}

#[cfg(feature = "ops")]
pub(crate) mod index_mut {
    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["IndexMut"];

    pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
        Ok(derive_trait(
            data,
            parse_quote!(::core::ops::IndexMut),
            Some(format_ident!("Output")),
            parse_quote! {
                trait IndexMut<__Idx>: ::core::ops::Index<__Idx> {
                    #[inline]
                    fn index_mut(&mut self, index: __Idx) -> &mut Self::Output;
                }
            },
        ))
    }
}

#[cfg(feature = "ops")]
pub(crate) mod range_bounds {
    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["RangeBounds"];

    pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
        Ok(derive_trait(data, parse_quote!(::core::ops::RangeBounds), None, parse_quote! {
            trait RangeBounds<__T: ?Sized> {
                #[inline]
                fn start_bound(&self) -> ::core::ops::Bound<&__T>;
                #[inline]
                fn end_bound(&self) -> ::core::ops::Bound<&__T>;
            }
        }))
    }
}

#[cfg(feature = "generator_trait")]
pub(crate) mod generator {
    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["Generator"];

    pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
        Ok(derive_trait(data, parse_quote!(::core::ops::Generator), None, parse_quote! {
            trait Generator<R> {
                type Yield;
                type Return;
                #[inline]
                fn resume(
                    self: ::core::pin::Pin<&mut Self>,
                    arg: R,
                ) -> ::core::ops::GeneratorState<Self::Yield, Self::Return>;
            }
        }))
    }
}

#[cfg(feature = "fn_traits")]
pub(crate) mod fn_ {
    use derive_utils::EnumImpl;
    use syn::TypeParam;

    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["Fn"];

    pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
        let trait_path = quote!(::core::ops::Fn);
        let trait_ = quote!(#trait_path(__T) -> __U);
        let fst = data.field_types().next();
        let mut impl_ = EnumImpl::new(data);

        impl_.set_trait(parse_quote!(#trait_path<(__T,)>));
        impl_.push_generic_param(TypeParam::from(format_ident!("__T")).into());
        impl_.push_generic_param(TypeParam::from(format_ident!("__U")).into());

        impl_.push_where_predicate(parse_quote!(#fst: #trait_));
        data.field_types()
            .skip(1)
            .for_each(|f| impl_.push_where_predicate(parse_quote!(#f: #trait_)));

        impl_.push_method(parse_quote! {
            #[inline]
            extern "rust-call" fn call(&self, args: (__T,)) -> Self::Output;
        });

        Ok(impl_.build())
    }
}

#[cfg(feature = "fn_traits")]
pub(crate) mod fn_mut {
    use derive_utils::EnumImpl;
    use syn::TypeParam;

    use crate::derive::*;

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
        data.field_types()
            .skip(1)
            .for_each(|f| impl_.push_where_predicate(parse_quote!(#f: #trait_)));

        impl_.push_method(parse_quote! {
            #[inline]
            extern "rust-call" fn call_mut(&mut self, args: (__T,)) -> Self::Output;
        });

        Ok(impl_.build())
    }
}

#[cfg(feature = "fn_traits")]
pub(crate) mod fn_once {
    use derive_utils::EnumImpl;
    use syn::TypeParam;

    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["FnOnce"];

    pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
        let trait_path = quote!(::core::ops::FnOnce);
        let trait_ = quote!(#trait_path(__T) -> __U);
        let fst = data.field_types().next();
        let mut impl_ = EnumImpl::new(data);

        impl_.set_trait(parse_quote!(#trait_path<(__T,)>));
        impl_.push_generic_param(TypeParam::from(format_ident!("__T")).into());
        impl_.push_generic_param(TypeParam::from(format_ident!("__U")).into());

        impl_.push_where_predicate(parse_quote!(#fst: #trait_));
        data.field_types()
            .skip(1)
            .for_each(|f| impl_.push_where_predicate(parse_quote!(#f: #trait_)));

        impl_.append_items_from_trait(parse_quote! {
            trait FnOnce {
                type Output;
                #[inline]
                extern "rust-call" fn call_once(self, args: (__T,)) -> Self::Output;
            }
        });

        Ok(impl_.build())
    }
}
