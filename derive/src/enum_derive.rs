use std::collections::HashMap;

use lazy_static::lazy_static;
use proc_macro2::TokenStream;
use syn::{
    parse::{Parse, ParseStream},
    token, ItemEnum, Path,
};

use crate::utils::*;

pub(crate) fn attribute(args: TokenStream, input: TokenStream) -> TokenStream {
    expand(args, input).unwrap_or_else(|e| e.to_compile_error())
}

macro_rules! trait_map {
    ($map:ident, $($(#[$meta:meta])* <$ident:expr, [$($deps:expr),*]>,)*) => {$(
        $(#[$meta])*
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
            #[cfg(feature = "ops")]
            <"DerefMut", ["Deref"]>,
            #[cfg(feature = "ops")]
            <"IndexMut", ["Index"]>,
            #[cfg(feature = "fn_traits")]
            <"Fn", ["FnMut", "FnOnce"]>,
            #[cfg(feature = "fn_traits")]
            <"FnMut", ["FnOnce"]>,
            <"DoubleEndedIterator", ["Iterator"]>,
            <"ExactSizeIterator", ["Iterator"]>,
            <"FusedIterator", ["Iterator"]>,
            #[cfg(feature = "trusted_len")]
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

type DeriveFn = fn(&'_ Data, &'_ mut Vec<ItemImpl>) -> Result<()>;

macro_rules! derive_map {
    ($map:expr, $($(#[$meta:meta])* $($arm:ident)::*,)*) => {$(
        $(#[$meta])*
        crate::derive::$($arm)::*::NAME.iter().for_each(|name| {
            if $map.insert(*name, crate::derive::$($arm)::*::derive as DeriveFn).is_some() {
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
            #[cfg(feature = "generator_trait")]
            core::ops::generator,
            #[cfg(stable_1_36)]
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
            #[cfg(feature = "futures")]
            external::futures::async_seek,
            #[cfg(feature = "futures")]
            external::futures::async_buf_read,
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
        );
        map
    };
}

struct Args {
    inner: Vec<(String, Path)>,
}

impl Parse for Args {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        fn to_trimed_string(p: &Path) -> String {
            p.to_token_stream().to_string().replace(" ", "")
        }

        let mut inner = Vec::new();

        while !input.is_empty() {
            let value = input.parse()?;
            inner.push((to_trimed_string(&value), value));

            if !input.is_empty() {
                let _: token::Comma = input.parse()?;
            }
        }

        Ok(Self { inner })
    }
}

fn expand(args: TokenStream, input: TokenStream) -> Result<TokenStream> {
    fn alias_exists(s: &str, v: &[(&str, Option<&Path>)]) -> bool {
        ALIAS_MAP.get(s).map_or(false, |x| v.iter().any(|(s, _)| s == x))
    }

    let span = input.clone();
    let mut item = syn::parse2::<ItemEnum>(input).map_err(|e| {
        error!(span, "the `#[enum_derive]` attribute may only be used on enums: {}", e)
    })?;
    let data = Data { data: EnumData::new(&item)?, span };
    let args = syn::parse2::<Args>(args)?.inner;
    let args = args.iter().fold(Vec::new(), |mut v, (s, arg)| {
        if let Some(traits) = TRAIT_DEPENDENCIES.get(&&**s) {
            traits.iter().filter(|&x| !args.iter().any(|(s, _)| s == x)).for_each(|s| {
                if !alias_exists(s, &v) {
                    v.push((s, None))
                }
            });
        }
        if !alias_exists(s, &v) {
            v.push((s, Some(arg)));
        }
        v
    });

    let mut derive = Vec::new();
    let mut items = Vec::new();
    for (s, arg) in args {
        match (DERIVE_MAP.get(&s), arg) {
            (Some(f), _) => (&*f)(&data, &mut items)
                .map_err(|e| error!(data.span, "`enum_derive({})` {}", s, e))?,
            (_, Some(arg)) => derive.push(arg),
            _ => {}
        }
    }

    if !derive.is_empty() {
        item.attrs.push(syn::parse_quote!(#[derive(#(#derive),*)]));
    }

    let mut item = item.into_token_stream();
    item.extend(items.into_iter().map(ToTokens::into_token_stream));
    Ok(item)
}
