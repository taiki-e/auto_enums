use proc_macro2::{token_stream::IntoIter, Delimiter, Group, Ident, TokenStream, TokenTree};
use quote::ToTokens;
use syn::{Path, Result};

// =============================================================================
// Arg

pub(super) enum Arg {
    Ident(Ident),
    Path(Path),
}

#[cfg(feature = "type_analysis")]
impl Arg {
    pub(super) fn to_trimed_string(&self) -> String {
        match self {
            Arg::Ident(i) => i.to_string(),
            Arg::Path(p) => p.to_token_stream().to_string().replace(" ", ""),
        }
    }
}

impl ToTokens for Arg {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Arg::Ident(i) => i.to_tokens(tokens),
            Arg::Path(p) => p.to_tokens(tokens),
        }
    }
}

#[cfg(feature = "type_analysis")]
impl From<Ident> for Arg {
    fn from(ident: Ident) -> Self {
        Arg::Ident(ident)
    }
}

#[cfg(feature = "type_analysis")]
impl From<Path> for Arg {
    fn from(path: Path) -> Self {
        Arg::Path(path)
    }
}

#[cfg(feature = "type_analysis")]
impl PartialEq for Arg {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Arg::Ident(x) => match other {
                Arg::Ident(y) => x.eq(y),
                Arg::Path(y) => y.is_ident(x),
            },
            Arg::Path(x) => match other {
                Arg::Ident(y) => x.is_ident(y),
                Arg::Path(y) => x.eq(y),
            },
        }
    }
}

#[cfg(feature = "type_analysis")]
impl Eq for Arg {}

// =============================================================================
// Parse

macro_rules! error {
    ($span:expr, $msg:expr) => {
        return Err(syn::Error::new($span.span(), $msg))
    };
    ($span:expr, $($tt:tt)*) => {
        error!($span, format!($($tt)*))
    };
}

pub(super) fn parse_group(group: TokenStream) -> Result<(Vec<Arg>, Option<String>)> {
    syn::parse2::<Group>(group).and_then(|group| parse_args(group.stream()))
}

pub(super) fn parse_args(args: TokenStream) -> Result<(Vec<Arg>, Option<String>)> {
    const ERR: &str = "expected one of `,`, `::`, or identifier, found ";

    let mut iter = args.into_iter();
    let mut args = Vec::new();
    let mut marker = None;
    while let Some(tt) = iter.next() {
        match tt {
            TokenTree::Ident(i) => {
                if i == "marker" {
                    marker_opt(i, &mut iter, &mut args, &mut marker)?;
                } else {
                    args.push(path_or_ident(i, iter.next(), &mut iter)?);
                }
            }
            TokenTree::Punct(p) => match p.as_char() {
                ',' => {}
                ':' => args.push(parse_path(vec![p.into()], &mut iter)?),
                _ => error!(p, "{}`{}`", ERR, p),
            },
            _ => error!(tt, "{}`{}`", ERR, tt),
        }
    }

    Ok((args, marker))
}

fn parse_path(mut path: Vec<TokenTree>, iter: &mut IntoIter) -> Result<Arg> {
    for tt in iter {
        match tt {
            TokenTree::Punct(ref p) if p.as_char() == ',' => break,
            tt => path.push(tt),
        }
    }

    syn::parse2(path.into_iter().collect()).map(Arg::Path)
}

fn path_or_ident(ident: Ident, tt: Option<TokenTree>, iter: &mut IntoIter) -> Result<Arg> {
    const ERR: &str = "expected one of `,`, or `::`, found ";

    match tt {
        None => Ok(Arg::Ident(ident)),
        Some(TokenTree::Punct(p)) => match p.as_char() {
            ',' => Ok(Arg::Ident(ident)),
            ':' => parse_path(vec![ident.into(), p.into()], iter),
            _ => error!(p, "{}`{}`", ERR, p),
        },
        Some(tt) => error!(tt, "{}`{}`", ERR, tt),
    }
}

fn marker_opt(
    ident: Ident,
    iter: &mut IntoIter,
    args: &mut Vec<Arg>,
    marker: &mut Option<String>,
) -> Result<()> {
    match iter.next() {
        Some(TokenTree::Group(ref g)) if g.delimiter() != Delimiter::Parenthesis => {
            error!(g, "invalid delimiter")
        }
        Some(TokenTree::Group(g)) => {
            let mut tokens = g.stream().into_iter();
            let i = match tokens.next() {
                Some(TokenTree::Ident(ref i)) if marker.is_some() => {
                    error!(i, "multiple `marker` option")
                }
                Some(TokenTree::Ident(i)) => i,
                Some(tt) => error!(tt, "expected an identifier, found `{}`", tt),
                None => error!(g, "empty `marker` option"),
            };
            *marker = Some(i.to_string());
            match tokens.next() {
                None => {}
                Some(TokenTree::Punct(ref p)) if p.as_char() == ',' => {
                    if let Some(tt) = tokens.next() {
                        // TODO: https://docs.rs/proc-macro2/0.4/proc_macro2/struct.Span.html#method.join
                        // `i.span().join(tt.span()).unwrap_or_else(|| tt.span())`
                        error!(tt, "multiple identifier in `marker` option")
                    }
                }
                // TODO: https://docs.rs/proc-macro2/0.4/proc_macro2/struct.Span.html#method.join
                Some(tt) => error!(tt, "multiple identifier in `marker` option"),
            }
        }
        Some(TokenTree::Punct(ref p)) if p.as_char() == '=' => {
            match iter.next() {
                Some(TokenTree::Ident(ref i)) if marker.is_some() => {
                    error!(i, "multiple `marker` option")
                }
                Some(TokenTree::Ident(i)) => *marker = Some(i.to_string()),
                Some(tt) => error!(tt, "expected an identifier, found `{}`", tt),
                None => error!(p, "empty `marker` option"),
            }
            match iter.next() {
                None => {}
                Some(TokenTree::Punct(ref p)) if p.as_char() == ',' => {}
                Some(tt) => error!(tt, "expected `,`, found `{}`", tt),
            }
        }
        tt => args.push(path_or_ident(ident, tt, iter)?),
    }

    Ok(())
}
