// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::cell::Cell;

use derive_utils::EnumData as Data;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_quote, Error, ItemEnum, Path, Result, Token,
};

pub(crate) fn attribute(args: TokenStream, input: TokenStream) -> TokenStream {
    expand(args, input).unwrap_or_else(Error::into_compile_error)
}

#[derive(Default)]
pub(crate) struct DeriveContext {
    needs_pin_projection: Cell<bool>,
}

impl DeriveContext {
    pub(crate) fn needs_pin_projection(&self) {
        self.needs_pin_projection.set(true);
    }
}

type DeriveFn = fn(&'_ DeriveContext, &'_ Data) -> Result<TokenStream>;

fn get_derive(s: &str) -> Option<DeriveFn> {
    macro_rules! match_derive {
        ($($(#[$meta:meta])* $($arm:ident)::*,)*) => {$(
            $(#[$meta])*
            {
                if crate::derive::$($arm)::*::NAME.iter().any(|name| *name == s) {
                    return Some(crate::derive::$($arm)::*::derive)
                }
            }
        )*};
    }

    match_derive! {
        // core
        #[cfg(feature = "convert")]
        core::convert::as_mut,
        #[cfg(feature = "convert")]
        core::convert::as_ref,
        core::fmt::debug,
        core::fmt::display,
        #[cfg(feature = "fmt")]
        core::fmt::pointer,
        #[cfg(feature = "fmt")]
        core::fmt::binary,
        #[cfg(feature = "fmt")]
        core::fmt::octal,
        #[cfg(feature = "fmt")]
        core::fmt::upper_hex,
        #[cfg(feature = "fmt")]
        core::fmt::lower_hex,
        #[cfg(feature = "fmt")]
        core::fmt::upper_exp,
        #[cfg(feature = "fmt")]
        core::fmt::lower_exp,
        core::fmt::write,
        core::iter::iterator,
        core::iter::double_ended_iterator,
        core::iter::exact_size_iterator,
        core::iter::fused_iterator,
        #[cfg(feature = "trusted_len")]
        core::iter::trusted_len,
        core::iter::extend,
        #[cfg(feature = "ops")]
        core::ops::deref,
        #[cfg(feature = "ops")]
        core::ops::deref_mut,
        #[cfg(feature = "ops")]
        core::ops::index,
        #[cfg(feature = "ops")]
        core::ops::index_mut,
        #[cfg(feature = "ops")]
        core::ops::range_bounds,
        #[cfg(feature = "fn_traits")]
        core::ops::fn_,
        #[cfg(feature = "fn_traits")]
        core::ops::fn_mut,
        #[cfg(feature = "fn_traits")]
        core::ops::fn_once,
        #[cfg(feature = "coroutine_trait")]
        core::ops::coroutine,
        core::future,
        // std
        #[cfg(feature = "std")]
        std::io::read,
        #[cfg(feature = "std")]
        std::io::buf_read,
        #[cfg(feature = "std")]
        std::io::seek,
        #[cfg(feature = "std")]
        std::io::write,
        #[cfg(feature = "std")]
        std::error,
        // type impls
        #[cfg(feature = "transpose_methods")]
        ty_impls::transpose,
        // futures03
        #[cfg(feature = "futures03")]
        external::futures03::stream,
        #[cfg(feature = "futures03")]
        external::futures03::sink,
        #[cfg(feature = "futures03")]
        external::futures03::async_read,
        #[cfg(feature = "futures03")]
        external::futures03::async_write,
        #[cfg(feature = "futures03")]
        external::futures03::async_seek,
        #[cfg(feature = "futures03")]
        external::futures03::async_buf_read,
        // futures01
        #[cfg(feature = "futures01")]
        external::futures01::future,
        #[cfg(feature = "futures01")]
        external::futures01::stream,
        #[cfg(feature = "futures01")]
        external::futures01::sink,
        // rayon
        #[cfg(feature = "rayon")]
        external::rayon::par_iter,
        #[cfg(feature = "rayon")]
        external::rayon::indexed_par_iter,
        #[cfg(feature = "rayon")]
        external::rayon::par_extend,
        // serde
        #[cfg(feature = "serde")]
        external::serde::serialize,
        // tokio1
        #[cfg(feature = "tokio1")]
        external::tokio1::async_read,
        #[cfg(feature = "tokio1")]
        external::tokio1::async_write,
        #[cfg(feature = "tokio1")]
        external::tokio1::async_seek,
        #[cfg(feature = "tokio1")]
        external::tokio1::async_buf_read,
        // tokio03
        #[cfg(feature = "tokio03")]
        external::tokio03::async_read,
        #[cfg(feature = "tokio03")]
        external::tokio03::async_write,
        #[cfg(feature = "tokio03")]
        external::tokio03::async_seek,
        #[cfg(feature = "tokio03")]
        external::tokio03::async_buf_read,
        // tokio02
        #[cfg(feature = "tokio02")]
        external::tokio02::async_read,
        #[cfg(feature = "tokio02")]
        external::tokio02::async_write,
        #[cfg(feature = "tokio02")]
        external::tokio02::async_seek,
        #[cfg(feature = "tokio02")]
        external::tokio02::async_buf_read,
        // tokio01
        #[cfg(feature = "tokio01")]
        external::tokio01::async_read,
        #[cfg(feature = "tokio01")]
        external::tokio01::async_write,
        // http_body1
        #[cfg(feature = "http_body1")]
        external::http_body1::body,
    }

    None
}

struct Args {
    inner: Vec<(String, Path)>,
}

impl Parse for Args {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        fn to_trimmed_string(p: &Path) -> String {
            p.to_token_stream().to_string().replace(' ', "")
        }

        let mut inner = vec![];
        while !input.is_empty() {
            let path = input.parse()?;
            inner.push((to_trimmed_string(&path), path));

            if input.is_empty() {
                break;
            }
            let _: Token![,] = input.parse()?;
        }

        Ok(Self { inner })
    }
}

fn get_trait_deps(s: &str) -> Option<&'static [&'static str]> {
    Some(match s {
        "Copy" => &["Clone"],
        "Eq" | "PartialOrd" => &["PartialEq"],
        "Ord" => &["PartialOrd", "Eq", "PartialEq"],
        #[cfg(feature = "ops")]
        "DerefMut" => &["Deref"],
        #[cfg(feature = "ops")]
        "IndexMut" => &["Index"],
        #[cfg(feature = "fn_traits")]
        "Fn" => &["FnMut", "FnOnce"],
        #[cfg(feature = "fn_traits")]
        "FnMut" => &["FnOnce"],
        "DoubleEndedIterator" | "ExactSizeIterator" | "FusedIterator" => &["Iterator"],
        #[cfg(feature = "trusted_len")]
        "TrustedLen" => &["Iterator"],
        #[cfg(feature = "std")]
        "BufRead" | "io::BufRead" => &["Read"],
        #[cfg(feature = "std")]
        "Error" => &["Display", "Debug"],
        #[cfg(feature = "rayon")]
        "rayon::IndexedParallelIterator" => &["rayon::ParallelIterator"],
        _ => return None,
    })
}

fn exists_alias(s: &str, v: &[(&str, Option<&Path>)]) -> bool {
    fn get_alias(s: &str) -> Option<&'static str> {
        macro_rules! match_alias {
            ($($(#[$meta:meta])* $($arm:ident)::*,)*) => {$(
                $(#[$meta])*
                {
                    if s == crate::derive::$($arm)::*::NAME[0] {
                        return Some(crate::derive::$($arm)::*::NAME[1]);
                    } else if s == crate::derive::$($arm)::*::NAME[1] {
                        return Some(crate::derive::$($arm)::*::NAME[0]);
                    }
                }
            )*};
        }

        match_alias! {
            // core
            core::fmt::debug,
            core::fmt::display,
            // std
            #[cfg(feature = "std")]
            std::io::read,
            #[cfg(feature = "std")]
            std::io::buf_read,
            #[cfg(feature = "std")]
            std::io::seek,
            #[cfg(feature = "std")]
            std::io::write,
        }

        None
    }

    get_alias(s).map_or(false, |x| v.iter().any(|(s, _)| *s == x))
}

fn expand(args: TokenStream, input: TokenStream) -> Result<TokenStream> {
    let data = syn::parse2::<Data>(input)?;
    let args = syn::parse2::<Args>(args)?.inner;
    let args = args.iter().fold(vec![], |mut v, (s, arg)| {
        if let Some(traits) = get_trait_deps(s) {
            for s in traits.iter().filter(|&x| !args.iter().any(|(s, _)| s == x)) {
                if !exists_alias(s, &v) {
                    v.push((s, None));
                }
            }
        }
        if !exists_alias(s, &v) {
            v.push((s, Some(arg)));
        }
        v
    });

    let mut derive = vec![];
    let mut items = TokenStream::new();
    let cx = DeriveContext::default();
    for (s, arg) in args {
        match (get_derive(s), arg) {
            (Some(f), _) => {
                items.extend(
                    f(&cx, &data).map_err(|e| format_err!(data, "`enum_derive({})` {}", s, e))?,
                );
            }
            (_, Some(arg)) => derive.push(arg),
            _ => {}
        }
    }

    let mut item = if cx.needs_pin_projection.get() {
        // If a user creates their own Unpin or Drop implementation, trait implementations with
        // `Pin<&mut self>` receiver can cause unsoundness.
        //
        // This was not a problem in #[auto_enum] attribute where enums are anonymized,
        // but it becomes a problem when users have access to enums (i.e., when using #[enum_derive]).
        //
        // So, we ensure safety here by an Unpin implementation that implements Unpin
        // only if all fields are Unpin (this also forbids custom Unpin implementation),
        // and a hack that forbids custom Drop implementation. (Both are what pin-project does by default.)
        // The repr(packed) check is not needed since repr(packed) is not available on enum.

        // Automatically create the appropriate conditional `Unpin` implementation.
        // https://github.com/taiki-e/pin-project/blob/v1.0.10/examples/struct-default-expanded.rs#L89
        // TODO: use https://github.com/taiki-e/pin-project/issues/102#issuecomment-540472282's trick.
        items.extend(derive_utils::derive_trait(
            &data,
            &parse_quote!(::core::marker::Unpin),
            None,
            parse_quote! {
                trait Unpin {}
            },
        ));

        let item: ItemEnum = data.into();
        let name = &item.ident;
        let (impl_generics, ty_generics, where_clause) = item.generics.split_for_impl();
        // Ensure that enum does not implement `Drop`.
        // https://github.com/taiki-e/pin-project/blob/v1.0.10/examples/struct-default-expanded.rs#L138
        items.extend(quote! {
            const _: () = {
                trait MustNotImplDrop {}
                #[allow(clippy::drop_bounds, drop_bounds)]
                impl<T: ::core::ops::Drop> MustNotImplDrop for T {}
                impl #impl_generics MustNotImplDrop for #name #ty_generics #where_clause {}
            };
        });
        item
    } else {
        data.into()
    };

    if !derive.is_empty() {
        item.attrs.push(parse_quote!(#[derive(#(#derive),*)]));
    }

    let mut item = item.into_token_stream();
    item.extend(items);
    Ok(item)
}
