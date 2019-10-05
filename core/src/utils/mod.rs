use std::{iter, mem};

use proc_macro2::TokenStream;
use syn::{punctuated::Punctuated, visit_mut::VisitMut, *};

mod attrs;

pub(crate) use self::attrs::{Attrs, AttrsMut};

// =================================================================================================
// Extension traits

pub(crate) trait VecExt<T> {
    fn find_remove(&mut self, predicate: impl FnMut(&T) -> bool) -> Option<T>;
}

impl<T> VecExt<T> for Vec<T> {
    fn find_remove(&mut self, predicate: impl FnMut(&T) -> bool) -> Option<T> {
        self.iter().position(predicate).map(|i| self.remove(i))
    }
}

// =================================================================================================
// Functions

pub(crate) fn path(segments: impl IntoIterator<Item = PathSegment>) -> Path {
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

/// Generate an expression to fill in where the error occurred during the visit.
/// These will eventually need to be replaced with the original error message.
pub(super) fn expr_unimplemented() -> Expr {
    syn::parse_quote!(compile_error!("#[auto_enum] failed to generate error message"))
}

pub(crate) fn unit() -> Expr {
    Expr::Tuple(ExprTuple {
        attrs: Vec::new(),
        paren_token: token::Paren::default(),
        elems: Punctuated::new(),
    })
}

pub(crate) fn replace_expr(this: &mut Expr, f: impl FnOnce(Expr) -> Expr) {
    *this = f(mem::replace(this, Expr::Verbatim(TokenStream::new())));
}

pub(crate) fn replace_block(this: &mut Block, f: impl FnOnce(Block) -> Expr) {
    *this = block(vec![Stmt::Expr(f(mem::replace(this, block(Vec::new()))))]);
}

// =================================================================================================
// Visited node

pub(crate) trait VisitedNode {
    fn visited(&mut self, visitor: &mut impl VisitMut);
}

impl VisitedNode for Stmt {
    fn visited(&mut self, visitor: &mut impl VisitMut) {
        visitor.visit_stmt_mut(self);
    }
}

impl VisitedNode for Local {
    fn visited(&mut self, visitor: &mut impl VisitMut) {
        visitor.visit_local_mut(self);
    }
}

impl VisitedNode for Expr {
    fn visited(&mut self, visitor: &mut impl VisitMut) {
        visitor.visit_expr_mut(self);
    }
}

impl VisitedNode for Arm {
    fn visited(&mut self, visitor: &mut impl VisitMut) {
        visitor.visit_arm_mut(self);
    }
}

impl VisitedNode for Block {
    fn visited(&mut self, visitor: &mut impl VisitMut) {
        visitor.visit_block_mut(self);
    }
}

impl VisitedNode for ItemFn {
    fn visited(&mut self, visitor: &mut impl VisitMut) {
        visitor.visit_item_fn_mut(self);
    }
}

// =================================================================================================
// Macros

macro_rules! error {
    ($span:expr, $msg:expr) => {
        syn::Error::new_spanned(&$span, $msg)
    };
    ($span:expr, $($tt:tt)*) => {
        error!($span, format!($($tt)*))
    };
}
