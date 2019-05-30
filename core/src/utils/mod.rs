use std::{iter, mem};

use proc_macro2::{Ident, Span, TokenStream};
use syn::{
    punctuated::Punctuated, token, Attribute, Block, Expr, ExprBlock, ExprCall, ExprPath,
    ExprTuple, ExprVerbatim, Path, PathSegment, Stmt,
};

mod attrs;

pub(crate) use self::attrs::{Attrs, AttrsMut};

// =============================================================================
// Extension traits

pub(crate) trait VecExt<T> {
    fn find_remove<P: FnMut(&T) -> bool>(&mut self, predicate: P) -> Option<T>;
}

impl<T> VecExt<T> for Vec<T> {
    fn find_remove<P: FnMut(&T) -> bool>(&mut self, predicate: P) -> Option<T> {
        self.iter().position(predicate).map(|i| self.remove(i))
    }
}

// =============================================================================
// Functions

pub(crate) fn ident<S: AsRef<str>>(s: S) -> Ident {
    Ident::new(s.as_ref(), Span::call_site())
}

pub(crate) fn path<I: IntoIterator<Item = PathSegment>>(segments: I) -> Path {
    Path { leading_colon: None, segments: segments.into_iter().collect() }
}

pub(crate) fn block(stmts: Vec<Stmt>) -> Block {
    Block { brace_token: token::Brace::default(), stmts }
}

pub(crate) fn expr_block(block: Block) -> Expr {
    Expr::Block(ExprBlock { attrs: Vec::new(), label: None, block })
}

pub(crate) fn expr_call(attrs: Vec<Attribute>, path: Path, arg: Expr) -> Expr {
    Expr::Call(ExprCall {
        attrs,
        func: Box::new(Expr::Path(ExprPath { attrs: Vec::new(), qself: None, path })),
        paren_token: token::Paren::default(),
        args: iter::once(arg).collect(),
    })
}

pub(super) fn expr_compile_error(e: &syn::Error) -> Expr {
    syn::parse2(e.to_compile_error()).unwrap()
}

pub(crate) fn unit() -> Expr {
    Expr::Tuple(ExprTuple {
        attrs: Vec::new(),
        paren_token: token::Paren::default(),
        elems: Punctuated::new(),
    })
}

pub(crate) fn replace_expr<F: FnOnce(Expr) -> Expr>(this: &mut Expr, op: F) {
    *this = op(mem::replace(this, Expr::Verbatim(ExprVerbatim { tts: TokenStream::new() })));
}

pub(crate) fn replace_block<F: FnOnce(Block) -> Expr>(this: &mut Block, op: F) {
    *this = block(vec![Stmt::Expr(op(mem::replace(this, block(Vec::new()))))]);
}

// =============================================================================
// Macros

macro_rules! span {
    ($expr:expr) => {
        $expr.clone()
    };
}

macro_rules! error {
    // FIXME: syntax
    (span => $span:expr, $msg:expr) => {
        syn::Error::new($span, $msg)
    };
    ($msg:expr) => {
        syn::Error::new_spanned(span!($msg), $msg)
    };
    ($span:expr, $msg:expr) => {
        syn::Error::new_spanned(span!($span), $msg)
    };
    ($span:expr, $($tt:tt)*) => {
        error!($span, format!($($tt)*))
    };
}
