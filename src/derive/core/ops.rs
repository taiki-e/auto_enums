// SPDX-License-Identifier: Apache-2.0 OR MIT

#[cfg(feature = "ops")]
pub(crate) mod deref {
    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["Deref"];

    pub(crate) fn derive(_cx: &Context, data: &Data) -> Result<TokenStream> {
        Ok(derive_trait(data, &parse_quote!(::core::ops::Deref), None, parse_quote! {
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

    pub(crate) fn derive(_cx: &Context, data: &Data) -> Result<TokenStream> {
        Ok(derive_trait(
            data,
            &parse_quote!(::core::ops::DerefMut),
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

    pub(crate) fn derive(_cx: &Context, data: &Data) -> Result<TokenStream> {
        Ok(derive_trait(data, &parse_quote!(::core::ops::Index), None, parse_quote! {
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

    pub(crate) fn derive(_cx: &Context, data: &Data) -> Result<TokenStream> {
        Ok(derive_trait(
            data,
            &parse_quote!(::core::ops::IndexMut),
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

    pub(crate) fn derive(_cx: &Context, data: &Data) -> Result<TokenStream> {
        Ok(derive_trait(data, &parse_quote!(::core::ops::RangeBounds), None, parse_quote! {
            trait RangeBounds<__T: ?Sized> {
                #[inline]
                fn start_bound(&self) -> ::core::ops::Bound<&__T>;
                #[inline]
                fn end_bound(&self) -> ::core::ops::Bound<&__T>;
            }
        }))
    }
}

#[cfg(feature = "coroutine_trait")]
pub(crate) mod coroutine {
    use quote::ToTokens;

    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["Coroutine"];

    pub(crate) fn derive(cx: &Context, data: &Data) -> Result<TokenStream> {
        cx.needs_pin_projection();

        let ident = &data.ident;
        let pin = quote!(::core::pin::Pin);
        let trait_ = parse_quote!(::core::ops::Coroutine);
        let mut impl_ = EnumImpl::from_trait(data, &trait_, None, parse_quote! {
            trait Coroutine<R> {
                type Yield;
                type Return;
            }
        })
        .build_impl();

        let resume = data.variant_idents().zip(data.field_types()).map(|(v, ty)| {
            quote! {
                #ident::#v(x) => <#ty as #trait_<R>>::resume(#pin::new_unchecked(x), arg),
            }
        });
        impl_.items.push(parse_quote! {
            #[inline]
            fn resume(
                self: #pin<&mut Self>,
                arg: R,
            ) -> ::core::ops::CoroutineState<Self::Yield, Self::Return> {
                unsafe {
                    match self.get_unchecked_mut() { #(#resume)* }
                }
            }
        });

        Ok(impl_.into_token_stream())
    }
}

#[cfg(feature = "fn_traits")]
pub(crate) mod fn_ {
    use derive_utils::EnumImpl;
    use syn::TypeParam;

    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["Fn"];

    pub(crate) fn derive(_cx: &Context, data: &Data) -> Result<TokenStream> {
        let trait_path = quote!(::core::ops::Fn);
        let trait_ = quote!(#trait_path(__T) -> __U);
        let mut impl_ = EnumImpl::new(data);

        impl_.set_trait(parse_quote!(#trait_path<(__T,)>));
        impl_.push_generic_param(TypeParam::from(format_ident!("__T")).into());
        impl_.push_generic_param(TypeParam::from(format_ident!("__U")).into());

        data.field_types().for_each(|f| impl_.push_where_predicate(parse_quote!(#f: #trait_)));

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

    pub(crate) fn derive(_cx: &Context, data: &Data) -> Result<TokenStream> {
        let trait_path = quote!(::core::ops::FnMut);
        let trait_ = quote!(#trait_path(__T) -> __U);
        let mut impl_ = EnumImpl::new(data);

        impl_.set_trait(parse_quote!(#trait_path<(__T,)>));
        impl_.push_generic_param(TypeParam::from(format_ident!("__T")).into());
        impl_.push_generic_param(TypeParam::from(format_ident!("__U")).into());

        data.field_types().for_each(|f| impl_.push_where_predicate(parse_quote!(#f: #trait_)));

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

    pub(crate) fn derive(_cx: &Context, data: &Data) -> Result<TokenStream> {
        let trait_path = quote!(::core::ops::FnOnce);
        let trait_ = quote!(#trait_path(__T) -> __U);
        let mut impl_ = EnumImpl::new(data);

        impl_.set_trait(parse_quote!(#trait_path<(__T,)>));
        impl_.push_generic_param(TypeParam::from(format_ident!("__T")).into());
        impl_.push_generic_param(TypeParam::from(format_ident!("__U")).into());

        data.field_types().for_each(|f| impl_.push_where_predicate(parse_quote!(#f: #trait_)));

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
