use std::{borrow::Cow, collections::HashMap};

use lazy_static::lazy_static;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};

use crate::utils::*;

mod args;

use self::args::*;

const NAME: &str = "enum_derive";

pub(crate) fn attribute(args: TokenStream, input: TokenStream) -> TokenStream {
    expand(args.into(), input)
        .unwrap_or_else(|e| compile_err(&e.to_string()))
        .into()
}

macro_rules! trait_map {
    ($map:ident, $($(#[$meta:meta])* <$ident:expr, [$($deps:expr),*]>,)*) => {$(
        $map.insert($ident, &[$($deps),*]);
    )*};
}

lazy_static! {
    static ref TRAITS: HashMap<&'static str, &'static [&'static str]> = {
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

type DeriveFn = Box<dyn Fn(&'_ Data) -> Result<TokenStream2> + Send + Sync>;

macro_rules! derive_map {
    ($map:expr, $($(#[$meta:meta])* $($arm:ident)::*,)*) => {$(
        $(#[$meta])*
        crate::derive::$($arm)::*::NAME.iter().for_each(|name| {
            if $map.insert(*name, Box::new(crate::derive::$($arm)::*::derive) as DeriveFn).is_some() {
                panic!("`#[{}]` internal error: there are multiple `{}`", NAME, name);
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
            std::ops::fn_,
            std::ops::fn_mut,
            std::ops::fn_once,
            std::ops::range_bounds,
            std::convert::as_mut,
            std::convert::as_ref,
            std::fmt::debug,
            std::fmt::display,
            std::fmt::pointer,
            std::fmt::binary,
            std::fmt::octal,
            std::fmt::upper_hex,
            std::fmt::lower_hex,
            std::fmt::upper_exp,
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
            #[cfg(feature = "serde")]
            external::serde::serialize,
        );
        map
    };
}

fn expand(args: TokenStream2, input: TokenStream) -> Result<TokenStream2> {
    fn alias_exists(x: &str, stack: &[(Cow<'_, str>, Option<Arg>)]) -> bool {
        ALIAS_MAP
            .get(x)
            .map_or(false, |x| stack.iter().any(|(s, _)| s == x))
    }

    let item = syn::parse(input).map_err(|_| format!("`#[{}]` can only be used on enums", NAME))?;
    let data = Data::from_item(&item).map_err(|e| format!("`#[{}]` {}", NAME, e))?;
    let args = parse_args(args).map_err(|e| format!("`#[{}]` {}", NAME, e))?;
    let mut stack = Stack::new();
    args.iter().cloned().for_each(|(s, x)| {
        if let Some(traits) = TRAITS.get(s.as_str()) {
            traits
                .iter()
                .filter(|&x| !args.iter().any(|(s, _)| s == x))
                .for_each(|&x| {
                    if !alias_exists(x, &stack) {
                        stack.push((Cow::Borrowed(x), None))
                    }
                });
        }

        if !alias_exists(s.as_str(), &stack) {
            stack.push((Cow::Owned(s), x));
        }
    });
    drop(args);

    let mut derive = Stack::new();
    let mut ts = TokenStream2::new();
    for (s, x) in stack {
        match DERIVE_MAP.get(&&*s) {
            Some(f) => (&**f)(&data)
                .map_err(|e| format!("`#[{}({})]` {}", NAME, s, e))
                .map(|x| ts.extend(x.into_token_stream()))?,
            None => derive.push(x.unwrap()),
        }
    }

    if derive.is_empty() {
        Ok(quote!(#item #ts))
    } else {
        Ok(quote!(#[derive(#(#derive),*)] #item #ts))
    }
}
