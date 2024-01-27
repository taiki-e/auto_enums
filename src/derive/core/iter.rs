// SPDX-License-Identifier: Apache-2.0 OR MIT

pub(crate) mod iterator {
    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["Iterator"];

    pub(crate) fn derive(_cx: &Context, data: &Data) -> Result<TokenStream> {
        // TODO: Add try_fold once try_trait_v2 is stabilized https://github.com/rust-lang/rust/issues/84277
        Ok(derive_trait(data, &parse_quote!(::core::iter::Iterator), None, parse_quote! {
            trait Iterator {
                type Item;
                #[inline]
                fn next(&mut self) -> ::core::option::Option<Self::Item>;
                #[inline]
                fn size_hint(&self) -> (usize, ::core::option::Option<usize>);
                #[inline]
                fn count(self) -> usize;
                #[inline]
                fn last(self) -> ::core::option::Option<Self::Item>;
                #[inline]
                fn nth(&mut self, n: usize) -> ::core::option::Option<Self::Item>;
                #[inline]
                #[must_use = "if you really need to exhaust the iterator, consider `.for_each(drop)` instead"]
                fn collect<__U: ::core::iter::FromIterator<Self::Item>>(self) -> __U;
                #[inline]
                fn partition<__U, __F>(self, f: __F) -> (__U, __U)
                where
                    __U: ::core::default::Default + ::core::iter::Extend<Self::Item>,
                    __F: ::core::ops::FnMut(&Self::Item) -> bool;

                // Once try_trait_v2 is stabilized, we can replace these by implementing try_rfold.
                #[inline]
                fn fold<__U, __F>(self, init: __U, f: __F) -> __U
                where
                    __F: ::core::ops::FnMut(__U, Self::Item) -> __U;
                #[inline]
                fn all<__F>(&mut self, f: __F) -> bool
                where
                    __F: ::core::ops::FnMut(Self::Item) -> bool;
                #[inline]
                fn any<__F>(&mut self, f: __F) -> bool
                where
                    __F: ::core::ops::FnMut(Self::Item) -> bool;
                #[inline]
                fn find<__P>(&mut self, predicate: __P) -> ::core::option::Option<Self::Item>
                where
                    __P: ::core::ops::FnMut(&Self::Item) -> bool;
                #[inline]
                fn find_map<__U, __F>(&mut self, f: __F) -> ::core::option::Option<__U>
                where
                    __F: ::core::ops::FnMut(Self::Item) -> ::core::option::Option<__U>;
                #[inline]
                fn position<__P>(&mut self, predicate: __P) -> ::core::option::Option<usize>
                where
                    __P: ::core::ops::FnMut(Self::Item) -> bool;
            }
        }))
    }
}

pub(crate) mod double_ended_iterator {
    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["DoubleEndedIterator"];

    pub(crate) fn derive(_cx: &Context, data: &Data) -> Result<TokenStream> {
        // TODO: Add try_rfold once try_trait_v2 is stabilized https://github.com/rust-lang/rust/issues/84277
        // TODO: Add advance_back_by once stabilized https://github.com/rust-lang/rust/issues/77404
        Ok(derive_trait(
            data,
            &parse_quote!(::core::iter::DoubleEndedIterator),
            Some(format_ident!("Item")),
            parse_quote! {
                trait DoubleEndedIterator: ::core::iter::Iterator {
                    #[inline]
                    fn next_back(&mut self) -> ::core::option::Option<Self::Item>;
                    #[inline]
                    fn nth_back(&mut self, n: usize) -> ::core::option::Option<Self::Item>;

                    // Once try_trait_v2 is stabilized, we can replace these by implementing try_rfold.
                    #[inline]
                    fn rfold<__U, __F>(self, init: __U, f: __F) -> __U
                    where
                        __F: ::core::ops::FnMut(__U, Self::Item) -> __U;
                    #[inline]
                    fn rfind<__P>(&mut self, predicate: __P) -> ::core::option::Option<Self::Item>
                    where
                        __P: ::core::ops::FnMut(&Self::Item) -> bool;
                }
            },
        ))
    }
}

pub(crate) mod exact_size_iterator {
    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["ExactSizeIterator"];

    pub(crate) fn derive(_cx: &Context, data: &Data) -> Result<TokenStream> {
        // TODO: Add is_empty once stabilized https://github.com/rust-lang/rust/issues/35428
        Ok(derive_trait(
            data,
            &parse_quote!(::core::iter::ExactSizeIterator),
            Some(format_ident!("Item")),
            parse_quote! {
                trait ExactSizeIterator: ::core::iter::Iterator {
                    #[inline]
                    fn len(&self) -> usize;
                }
            },
        ))
    }
}

pub(crate) mod fused_iterator {
    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["FusedIterator"];

    pub(crate) fn derive(_cx: &Context, data: &Data) -> Result<TokenStream> {
        Ok(derive_trait(
            data,
            &parse_quote!(::core::iter::FusedIterator),
            Some(format_ident!("Item")),
            parse_quote! {
                trait FusedIterator: ::core::iter::Iterator {}
            },
        ))
    }
}

#[cfg(feature = "trusted_len")]
pub(crate) mod trusted_len {
    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["TrustedLen"];

    pub(crate) fn derive(_cx: &Context, data: &Data) -> Result<TokenStream> {
        Ok(derive_trait(
            data,
            &parse_quote!(::core::iter::TrustedLen),
            Some(format_ident!("Item")),
            parse_quote! {
                unsafe trait TrustedLen: ::core::iter::Iterator {}
            },
        ))
    }
}

pub(crate) mod extend {
    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["Extend"];

    pub(crate) fn derive(_cx: &Context, data: &Data) -> Result<TokenStream> {
        // TODO: Add extend_one,extend_reserve once stabilized https://github.com/rust-lang/rust/issues/72631
        Ok(derive_trait(data, &parse_quote!(::core::iter::Extend), None, parse_quote! {
            trait Extend<__A> {
                #[inline]
                fn extend<__T: ::core::iter::IntoIterator<Item = __A>>(&mut self, iter: __T);
            }
        }))
    }
}
