use std::mem;

use proc_macro2::{Ident, Span, TokenStream};
use smallvec::SmallVec;
use syn::{punctuated::Punctuated, *};

#[macro_use]
mod error;

pub(crate) use self::error::{Error, Result};

pub(crate) type Stack<T> = SmallVec<[T; 4]>;

// =============================================================================
// Extension traits

pub(crate) trait OptionExt {
    fn replace_boxed_expr<F: FnOnce(Expr) -> Expr>(&mut self, f: F);
}

impl OptionExt for Option<Box<Expr>> {
    fn replace_boxed_expr<F: FnOnce(Expr) -> Expr>(&mut self, f: F) {
        if self.is_none() {
            self.replace(Box::new(unit()));
        }

        if let Some(expr) = self {
            replace_expr(&mut **expr, f);
        }
    }
}

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

pub(crate) fn default<T: Default>() -> T {
    T::default()
}

pub(crate) fn ident<S: AsRef<str>>(s: S) -> Ident {
    Ident::new(s.as_ref(), Span::call_site())
}

pub(crate) fn path<I: IntoIterator<Item = PathSegment>>(segments: I) -> Path {
    Path {
        leading_colon: None,
        segments: segments.into_iter().collect(),
    }
}

pub(crate) fn block(stmts: Vec<Stmt>) -> Block {
    Block {
        brace_token: default(),
        stmts,
    }
}

pub(crate) fn expr_block(block: Block) -> Expr {
    Expr::Block(ExprBlock {
        attrs: Vec::with_capacity(0),
        label: None,
        block,
    })
}

pub(crate) fn unit() -> Expr {
    Expr::Tuple(ExprTuple {
        attrs: Vec::with_capacity(0),
        paren_token: default(),
        elems: Punctuated::new(),
    })
}

pub(crate) fn replace_expr<F: FnOnce(Expr) -> Expr>(this: &mut Expr, op: F) {
    *this = op(mem::replace(
        this,
        Expr::Verbatim(ExprVerbatim {
            tts: TokenStream::new(),
        }),
    ));
}

pub(crate) fn replace_block<F: FnOnce(Block) -> Expr>(this: &mut Block, op: F) {
    *this = block(vec![Stmt::Expr(op(mem::replace(
        this,
        block(Vec::with_capacity(0)),
    )))]);
}
