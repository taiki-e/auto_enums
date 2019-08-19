use proc_macro2::{token_stream::IntoIter, Delimiter, Ident, TokenStream, TokenTree};
use syn::{Path, Result};

macro_rules! error {
    ($span:expr, $msg:expr) => {
         Err(syn::Error::new($span.span(), $msg))
    };
    ($span:expr, $($tt:tt)*) => {
        error!($span, format!($($tt)*))
    };
}

pub(super) fn parse_args(args: TokenStream) -> Result<(Vec<Path>, Option<String>)> {
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
                    args.push(parse_path(i.into(), None, &mut iter)?);
                }
            }
            TokenTree::Punct(p) => match p.as_char() {
                ',' => {}
                ':' => args.push(parse_path(p.into(), None, &mut iter)?),
                _ => return error!(p, "{}`{}`", ERR, p),
            },
            _ => return error!(tt, "{}`{}`", ERR, tt),
        }
    }

    Ok((args, marker))
}

fn parse_path(tt1: TokenTree, tt2: Option<TokenTree>, iter: &mut IntoIter) -> Result<Path> {
    let mut tokens = TokenStream::new();
    tokens.extend(Some(tt1));
    tokens.extend(tt2);
    let iter =
        iter.take_while(|tt| if let TokenTree::Punct(p) = tt { p.as_char() != ',' } else { true });
    tokens.extend(iter);

    syn::parse2(tokens)
}

fn marker_opt(
    ident: Ident,
    iter: &mut IntoIter,
    args: &mut Vec<Path>,
    marker: &mut Option<String>,
) -> Result<()> {
    match iter.next() {
        Some(TokenTree::Group(ref g)) if g.delimiter() != Delimiter::Parenthesis => {
            return error!(g, "invalid delimiter")
        }
        Some(TokenTree::Group(g)) => {
            let mut tokens = g.stream().into_iter();
            let i = match tokens.next() {
                Some(TokenTree::Ident(ref i)) if marker.is_some() => {
                    return error!(i, "multiple `marker` option")
                }
                Some(TokenTree::Ident(i)) => i,
                Some(tt) => return error!(tt, "expected an identifier, found `{}`", tt),
                None => return error!(g, "empty `marker` option"),
            };
            *marker = Some(i.to_string());
            match tokens.next() {
                None => {}
                Some(TokenTree::Punct(ref p)) if p.as_char() == ',' => {
                    if let Some(tt) = tokens.next() {
                        // TODO: https://docs.rs/proc-macro2/0.4/proc_macro2/struct.Span.html#method.join
                        // `i.span().join(tt.span()).unwrap_or_else(|| tt.span())`
                        return error!(tt, "multiple identifier in `marker` option");
                    }
                }
                // TODO: https://docs.rs/proc-macro2/0.4/proc_macro2/struct.Span.html#method.join
                Some(tt) => return error!(tt, "multiple identifier in `marker` option"),
            }
        }
        Some(TokenTree::Punct(ref p)) if p.as_char() == '=' => {
            match iter.next() {
                Some(TokenTree::Ident(ref i)) if marker.is_some() => {
                    return error!(i, "multiple `marker` option")
                }
                Some(TokenTree::Ident(i)) => *marker = Some(i.to_string()),
                Some(tt) => return error!(tt, "expected an identifier, found `{}`", tt),
                None => return error!(p, "empty `marker` option"),
            }
            match iter.next() {
                None => {}
                Some(TokenTree::Punct(ref p)) if p.as_char() == ',' => {}
                Some(tt) => return error!(tt, "expected `,`, found `{}`", tt),
            }
        }
        tt => args.push(parse_path(ident.into(), tt, iter)?),
    }

    Ok(())
}
