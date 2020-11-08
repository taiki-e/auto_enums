pub(crate) mod iterator {
    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["Iterator"];

    pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
        // TODO: When `try_trait` stabilized, add `try_fold` and remove `fold`, `find` etc. conditionally.

        // It is equally efficient if `try_fold` can be used.
        let try_trait = quote! {
            #[inline]
            fn fold<__U, __F>(self, init: __U, f: __F) -> __U
            where
                __F: ::core::ops::FnMut(__U, Self::Item) -> __U;
            #[inline]
            fn find<__P>(&mut self, predicate: __P) -> ::core::option::Option<Self::Item>
            where
                __P: ::core::ops::FnMut(&Self::Item) -> bool;
            #[inline]
            fn find_map<__U, __F>(&mut self, f: __F) -> ::core::option::Option<__U>
            where
                __F: ::core::ops::FnMut(Self::Item) -> ::core::option::Option<__U>;
        };

        Ok(derive_trait(data, parse_quote!(::core::iter::Iterator), None, parse_quote! {
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
                #[must_use = "if you really need to exhaust the iterator, consider `.for_each(drop)` instead"]
                fn collect<__U: ::core::iter::FromIterator<Self::Item>>(self) -> __U;
                #try_trait
            }
        }))
    }
}

pub(crate) mod double_ended_iterator {
    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["DoubleEndedIterator"];

    pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
        // TODO: When `try_trait` stabilized, add `try_rfold` and remove `rfold` and `rfind` conditionally.

        // It is equally efficient if `try_rfold` can be used.
        let try_trait = quote! {
            #[inline]
            fn rfold<__U, __F>(self, accum: __U, f: __F) -> __U
            where
                __F: ::core::ops::FnMut(__U, Self::Item) -> __U;
            #[inline]
            fn rfind<__P>(&mut self, predicate: __P) -> ::core::option::Option<Self::Item>
            where
                __P: ::core::ops::FnMut(&Self::Item) -> bool;
        };

        Ok(derive_trait(
            data,
            parse_quote!(::core::iter::DoubleEndedIterator),
            Some(format_ident!("Item")),
            parse_quote! {
                trait DoubleEndedIterator: ::core::iter::Iterator {
                    #[inline]
                    fn next_back(&mut self) -> ::core::option::Option<Self::Item>;
                    #try_trait
                }
            },
        ))
    }
}

pub(crate) mod exact_size_iterator {
    use crate::derive::*;

    pub(crate) const NAME: &[&str] = &["ExactSizeIterator"];

    pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
        // TODO: When `exact_size_is_empty` stabilized, add `is_empty` conditionally.

        Ok(derive_trait(
            data,
            parse_quote!(::core::iter::ExactSizeIterator),
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

    pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
        Ok(derive_trait(
            data,
            parse_quote!(::core::iter::FusedIterator),
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

    pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
        Ok(derive_trait(
            data,
            parse_quote!(::core::iter::TrustedLen),
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

    pub(crate) fn derive(data: &Data) -> Result<TokenStream> {
        Ok(derive_trait(data, parse_quote!(::core::iter::Extend), None, parse_quote! {
            trait Extend<__A> {
                #[inline]
                fn extend<__T: ::core::iter::IntoIterator<Item = __A>>(&mut self, iter: __T);
            }
        }))
    }
}
