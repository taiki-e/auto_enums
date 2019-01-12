use std::mem;

use proc_macro2::{Ident, Span};
use smallvec::SmallVec;
use syn::{punctuated::Punctuated, *};

#[macro_use]
mod error;

pub(crate) use self::error::{Error, Result, *};

pub(crate) type Stack<T> = SmallVec<[T; 4]>;

pub(crate) fn default<T: Default>() -> T {
    T::default()
}

pub(crate) trait OptionExt {
    fn replace_boxed_expr<F: FnOnce(Expr) -> Expr>(&mut self, f: F);
}

impl<'a> OptionExt for Option<Box<Expr>> {
    fn replace_boxed_expr<F: FnOnce(Expr) -> Expr>(&mut self, f: F) {
        if self.is_none() {
            *self = Some(Box::new(unit()));
        }

        if let Some(expr) = self {
            replace_expr(&mut **expr, f);
        }
    }
}

pub(crate) trait VecExt<T> {
    fn find_remove<P>(&mut self, predicate: P) -> Option<T>
    where
        P: FnMut(&T) -> bool;
}

impl<T> VecExt<T> for Vec<T> {
    fn find_remove<P>(&mut self, predicate: P) -> Option<T>
    where
        P: FnMut(&T) -> bool,
    {
        fn remove<T>(v: &mut Vec<T>, index: usize) -> T {
            match v.len() {
                1 => v.pop().unwrap(),
                2 => v.swap_remove(index),
                _ => v.remove(index),
            }
        }

        self.iter().position(predicate).map(|i| remove(self, i))
    }
}

pub(crate) fn ident_call_site<S: AsRef<str>>(s: S) -> Ident {
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

pub(crate) fn replace_expr<F>(this: &mut Expr, op: F)
where
    F: FnOnce(Expr) -> Expr,
{
    fn expr_continue() -> Expr {
        // probably the lowest cost expression.
        Expr::Continue(ExprContinue {
            attrs: Vec::with_capacity(0),
            continue_token: default(),
            label: None,
        })
    }

    *this = op(mem::replace(this, expr_continue()));
}

pub(crate) fn replace_block<F>(this: &mut Block, op: F)
where
    F: FnOnce(Block) -> Expr,
{
    *this = block(vec![Stmt::Expr(op(mem::replace(
        this,
        block(Vec::with_capacity(0)),
    )))]);
}
