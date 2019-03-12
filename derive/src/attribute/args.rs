use proc_macro2::{token_stream::IntoIter, Ident, TokenStream, TokenTree};
use smallvec::smallvec;
use syn::Path;

use crate::utils::*;

#[derive(Clone)]
pub(super) enum Arg {
    Ident(Ident),
    Path(Path),
}

impl Arg {
    pub(super) fn to_trimed_string(&self) -> String {
        match self {
            Arg::Ident(i) => i.to_string(),
            Arg::Path(p) => p.clone().into_token_stream().to_string().replace(" ", ""),
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

impl From<Ident> for Arg {
    fn from(ident: Ident) -> Self {
        Arg::Ident(ident)
    }
}

impl From<Path> for Arg {
    fn from(path: Path) -> Self {
        Arg::Path(path)
    }
}

pub(super) fn parse_args(args: TokenStream) -> Result<Stack<(String, Arg)>> {
    fn push(args: &mut Stack<(String, Arg)>, arg: Arg) {
        args.push((arg.to_trimed_string(), arg))
    }

    const ERR: &str = "expected one of `,`, `::`, or identifier, found ";

    let mut iter = args.into_iter();
    let mut args = Stack::new();
    while let Some(tt) = iter.next() {
        match tt {
            TokenTree::Ident(i) => push(&mut args, path_or_ident(i, iter.next(), &mut iter)?),
            TokenTree::Punct(p) => match p.as_char() {
                ',' => {}
                ':' => push(&mut args, parse_path(smallvec![p.into()], &mut iter)?),
                _ => Err(invalid_args!("{}`{}`", ERR, p))?,
            },
            _ => Err(invalid_args!("{}`{}`", ERR, tt))?,
        }
    }

    Ok(args)
}

fn parse_path(mut path: Stack<TokenTree>, iter: &mut IntoIter) -> Result<Arg> {
    for tt in iter {
        match tt {
            TokenTree::Punct(ref p) if p.as_char() == ',' => break,
            tt => path.push(tt),
        }
    }

    syn::parse2(path.into_iter().collect())
        .map_err(|e| invalid_args!(e))
        .map(Arg::Path)
}

fn path_or_ident(ident: Ident, tt: Option<TokenTree>, iter: &mut IntoIter) -> Result<Arg> {
    const ERR: &str = "expected one of `,`, or `::`, found ";

    match tt {
        None => Ok(ident.into()),
        Some(TokenTree::Punct(p)) => match p.as_char() {
            ',' => Ok(ident.into()),
            ':' => parse_path(smallvec![ident.into(), p.into()], iter),
            _ => Err(invalid_args!("{}`{}`", ERR, p)),
        },
        Some(tt) => Err(invalid_args!("{}`{}`", ERR, tt)),
    }
}
