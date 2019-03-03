use std::collections::HashMap;

use lazy_static::lazy_static;
use proc_macro2::TokenStream;
use syn::{
    parse::{Parse, ParseStream},
    Attribute, ItemEnum,
};

use crate::utils::*;

mod args;

use self::args::*;

/// The attribute name.
const NAME: &str = "enum_derive";

pub(crate) fn attribute(args: TokenStream, input: TokenStream) -> TokenStream {
    expand(args, input).unwrap_or_else(|e| compile_err(&e.to_string()))
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
            std::fmt::debug,
            std::fmt::display,
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
                panic!("`{}` internal error: there are multiple `{}`", NAME, name);
            }
        });
    )*};
}

lazy_static! {
    static ref DERIVE_MAP: HashMap<&'static str, DeriveFn> = {
        let mut map = HashMap::new();
        derive_map!(
            map,
            std::iter::iterator,
            std::iter::double_ended_iterator,
            std::iter::exact_size_iterator,
            std::iter::fused_iterator,
            std::iter::trusted_len,
            std::iter::extend,
            std::ops::deref,
            std::ops::deref_mut,
            std::ops::index,
            std::ops::index_mut,
            std::ops::range_bounds,
            std::ops::fn_,
            std::ops::fn_mut,
            std::ops::fn_once,
            std::ops::generator,
            std::convert::as_mut,
            std::convert::as_ref,
            std::fmt::debug,
            std::fmt::display,
            #[cfg(feature = "fmt")]
            std::fmt::pointer,
            #[cfg(feature = "fmt")]
            std::fmt::binary,
            #[cfg(feature = "fmt")]
            std::fmt::octal,
            #[cfg(feature = "fmt")]
            std::fmt::upper_hex,
            #[cfg(feature = "fmt")]
            std::fmt::lower_hex,
            #[cfg(feature = "fmt")]
            std::fmt::upper_exp,
            #[cfg(feature = "fmt")]
            std::fmt::lower_exp,
            std::fmt::write,
            std::future,
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

    let mut item = syn::parse2::<ItemEnum>(input)
        .map_err(|_| format!("`{}` may only be used on enums", NAME))?;
    let data = Data::new(&item).map_err(|e| format!("`{}` {}", NAME, e))?;
    let args = parse_args(args).map_err(|e| format!("`{}` {}", NAME, e))?;
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
            (Some(f), _) => {
                (&**f)(&data, &mut impls).map_err(|e| format!("`{}({})` {}", NAME, s, e))?
            }
            (_, Some(arg)) => derive.push(arg),
            _ => {}
        }
    }

    if !derive.is_empty() {
        let mut derive: Attributes = syn::parse_quote!(#[derive(#(#derive),*)]);
        if let Some(derive) = derive.0.pop() {
            item.attrs.push(derive);
        }
    }

    let mut item = item.into_token_stream();
    item.extend(impls.into_iter().map(ToTokens::into_token_stream));
    Ok(item)
}

struct Attributes(Vec<Attribute>);

impl Parse for Attributes {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        input.call(Attribute::parse_outer).map(Attributes)
    }
}
