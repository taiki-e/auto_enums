use std::collections::HashMap;

use lazy_static::lazy_static;
use proc_macro2::TokenStream;
use syn::ItemEnum;

use crate::utils::*;

mod args;

use self::args::{parse_args, Arg};

pub(crate) fn attribute(args: TokenStream, input: TokenStream) -> TokenStream {
    expand(args, input).unwrap_or_else(|e| e.to_compile_error())
}

macro_rules! trait_map {
    ($map:ident, $($(#[$meta:meta])* <$ident:expr, [$($deps:expr),*]>,)*) => {$(
        $map.insert($ident, &[$($deps),*]);
    )*};
}

lazy_static! {
    static ref TRAIT_DEPENDENCIES: HashMap<&'static str, &'static [&'static str]> = {
        let mut map: HashMap<&'static str, &'static [&'static str]> = HashMap::new();
        trait_map! {
            map,
            <"Copy", ["Clone"]>,
            <"Eq", ["PartialEq"]>,
            <"PartialOrd", ["PartialEq"]>,
            <"Ord", ["PartialOrd", "Eq", "PartialEq"]>,
            <"DerefMut", ["Deref"]>,
            <"IndexMut", ["Index"]>,
            <"Fn", ["FnMut", "FnOnce"]>,
            <"FnMut", ["FnOnce"]>,
            <"DoubleEndedIterator", ["Iterator"]>,
            <"ExactSizeIterator", ["Iterator"]>,
            <"FusedIterator", ["Iterator"]>,
            <"TrustedLen", ["Iterator"]>,
            #[cfg(feature = "std")]
            <"BufRead", ["Read"]>,
            #[cfg(feature = "std")]
            <"io::BufRead", ["io::Read"]>,
            #[cfg(feature = "std")]
            <"Error", ["Display", "Debug"]>,
            #[cfg(feature = "rayon")]
            <"rayon::IndexedParallelIterator", ["rayon::ParallelIterator"]>,
        }
        map
    };
}

macro_rules! alias_map {
    ($map:expr, $($(#[$meta:meta])* $($arm:ident)::*,)*) => {$(
        $(#[$meta])*
        $map.insert(crate::derive::$($arm)::*::NAME[0], crate::derive::$($arm)::*::NAME[1]);
        $(#[$meta])*
        $map.insert(crate::derive::$($arm)::*::NAME[1], crate::derive::$($arm)::*::NAME[0]);
    )*};
}

lazy_static! {
    static ref ALIAS_MAP: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        alias_map! {
            map,
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
        map
    };
}

type DeriveFn = &'static (dyn Fn(&'_ Data, &'_ mut Stack<ItemImpl>) -> Result<()> + Send + Sync);

macro_rules! derive_map {
    ($map:expr, $($(#[$meta:meta])* $($arm:ident)::*,)*) => {$(
        $(#[$meta])*
        crate::derive::$($arm)::*::NAME.iter().for_each(|name| {
            if $map.insert(*name, (&crate::derive::$($arm)::*::derive) as DeriveFn).is_some() {
                panic!("`#[enum_derive]` internal error: there are multiple `{}`", name);
            }
        });
    )*};
}

lazy_static! {
    static ref DERIVE_MAP: HashMap<&'static str, DeriveFn> = {
        let mut map = HashMap::new();
        derive_map!(
            map,
            // core
            core::convert::as_mut,
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
            core::iter::trusted_len,
            core::iter::extend,
            core::ops::deref,
            core::ops::deref_mut,
            core::ops::index,
            core::ops::index_mut,
            core::ops::range_bounds,
            core::ops::fn_,
            core::ops::fn_mut,
            core::ops::fn_once,
            core::ops::generator,
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

            // futures
            #[cfg(feature = "futures")]
            external::futures::stream,
            #[cfg(feature = "futures")]
            external::futures::sink,
            #[cfg(feature = "futures")]
            external::futures::async_read,
            #[cfg(feature = "futures")]
            external::futures::async_write,
            // futures01
            #[cfg(feature = "futures01")]
            external::futures01::future,
            #[cfg(feature = "futures01")]
            external::futures01::stream,
            #[cfg(feature = "futures01")]
            external::futures01::sink,
            // proc_macro
            #[cfg(feature = "proc_macro")]
            external::proc_macro::to_tokens,
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
        );
        map
    };
}

fn expand(args: TokenStream, input: TokenStream) -> Result<TokenStream> {
    fn alias_exists(s: &str, stack: &[(&str, Option<&Arg>)]) -> bool {
        ALIAS_MAP
            .get(s)
            .map_or(false, |x| stack.iter().any(|(s, _)| s == x))
    }

    let span = span!(input);
    let mut item = syn::parse2::<ItemEnum>(input).map_err(|_| {
        err!(
            span,
            "the `#[enum_derive]` attribute may only be used on enums",
        )
    })?;
    let data = EnumData::new(&item)?;
    let data = Data { data, span };
    let args = parse_args(args)?;
    let mut stack = Stack::new();
    args.iter().for_each(|(s, arg)| {
        if let Some(traits) = TRAIT_DEPENDENCIES.get(&&**s) {
            traits
                .iter()
                .filter(|&x| !args.iter().any(|(s, _)| s == x))
                .for_each(|s| {
                    if !alias_exists(s, &stack) {
                        stack.push((s, None))
                    }
                });
        }

        if !alias_exists(s, &stack) {
            stack.push((s, Some(arg)));
        }
    });

    let mut derive = Stack::new();
    let mut impls = Stack::new();
    for (s, arg) in stack {
        match (DERIVE_MAP.get(&s), arg) {
            (Some(f), _) => (&**f)(&data, &mut impls)
                .map_err(|e| err!(data.span, "`enum_derive({})` {}", s, e))?,
            (_, Some(arg)) => derive.push(arg),
            _ => {}
        }
    }

    if !derive.is_empty() {
        item.attrs.push(syn::parse_quote!(#[derive(#(#derive),*)]));
    }

    let mut item = item.into_token_stream();
    item.extend(impls.into_iter().map(ToTokens::into_token_stream));
    Ok(item)
}
