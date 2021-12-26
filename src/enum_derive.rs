use derive_utils::EnumData as Data;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    parse_quote, Error, ItemEnum, Path, Result, Token,
};

pub(crate) fn attribute(args: TokenStream, input: TokenStream) -> TokenStream {
    expand(args, input).unwrap_or_else(Error::into_compile_error)
}

type DeriveFn = fn(&'_ Data) -> Result<TokenStream>;

fn get_derive(s: &str) -> Option<DeriveFn> {
    macro_rules! match_derive {
        ($($(#[$meta:meta])* $($arm:ident)::*,)*) => {$(
            $(#[$meta])*
            {
                if crate::derive::$($arm)::*::NAME.iter().any(|name| *name == s) {
                    return Some(crate::derive::$($arm)::*::derive as DeriveFn)
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
        #[cfg(feature = "generator_trait")]
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

        let mut inner = Vec::new();
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
    let args = args.iter().fold(Vec::new(), |mut v, (s, arg)| {
        if let Some(traits) = get_trait_deps(s) {
            traits.iter().filter(|&x| !args.iter().any(|(s, _)| s == x)).for_each(|s| {
                if !exists_alias(s, &v) {
                    v.push((s, None));
                }
            });
        }
        if !exists_alias(s, &v) {
            v.push((s, Some(arg)));
        }
        v
    });

    let mut derive = Vec::new();
    let mut items = TokenStream::new();
    for (s, arg) in args {
        match (get_derive(s), arg) {
            (Some(f), _) => {
                items
                    .extend(f(&data).map_err(|e| format_err!(data, "`enum_derive({})` {}", s, e))?);
            }
            (_, Some(arg)) => derive.push(arg),
            _ => {}
        }
    }

    let mut item: ItemEnum = data.into();
    if !derive.is_empty() {
        item.attrs.push(parse_quote!(#[derive(#(#derive),*)]));
    }

    let mut item = item.into_token_stream();
    item.extend(items);
    Ok(item)
}
