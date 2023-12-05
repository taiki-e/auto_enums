// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::{iter, mem};

use proc_macro2::TokenStream;
use syn::{
    punctuated::Punctuated, token, visit_mut::VisitMut, Arm, Attribute, Block, Expr, ExprBlock,
    ExprCall, ExprPath, ExprTuple, ItemFn, Local, Meta, Path, PathSegment, Stmt, StmtMacro,
};

pub(crate) fn path(segments: impl IntoIterator<Item = PathSegment>) -> Path {
    Path { leading_colon: None, segments: segments.into_iter().collect() }
}

pub(crate) fn block(stmts: Vec<Stmt>) -> Block {
    Block { brace_token: token::Brace::default(), stmts }
}

pub(crate) fn expr_block(block: Block) -> Expr {
    Expr::Block(ExprBlock { attrs: vec![], label: None, block })
}

pub(crate) fn expr_call(attrs: Vec<Attribute>, path: Path, arg: Expr) -> Expr {
    Expr::Call(ExprCall {
        attrs,
        func: Box::new(Expr::Path(ExprPath { attrs: vec![], qself: None, path })),
        paren_token: token::Paren::default(),
        args: iter::once(arg).collect(),
    })
}

pub(crate) fn unit() -> Expr {
    Expr::Tuple(ExprTuple {
        attrs: vec![],
        paren_token: token::Paren::default(),
        elems: Punctuated::new(),
    })
}

pub(crate) fn replace_expr(this: &mut Expr, f: impl FnOnce(Expr) -> Expr) {
    *this = f(mem::replace(this, Expr::Verbatim(TokenStream::new())));
}

pub(crate) fn replace_block(this: &mut Block, f: impl FnOnce(Block) -> Expr) {
    // `brace_token` of the block that passed to `f` should have `call_site` span.
    // If `f` generates unused braces containing the span of `this.brace_token`,
    // this will cause confusing warnings: https://github.com/rust-lang/rust/issues/71080
    let stmts = mem::take(&mut this.stmts);
    this.stmts = vec![Stmt::Expr(f(block(stmts)), None)];
}

pub(crate) fn path_eq(path: &syn::Path, expected_crates: &[&str], expected_path: &[&str]) -> bool {
    if path.segments.len() == 1 && path.segments[0].ident == expected_path.last().unwrap() {
        return true;
    }
    if path.segments.len() == expected_path.len() + 1 {
        if !expected_crates.iter().any(|&c| path.segments[0].ident == c) {
            return false;
        }
        for i in 1..path.segments.len() {
            if path.segments[i].ident != expected_path[i - 1] {
                return false;
            }
        }
        return true;
    }
    false
}

// =================================================================================================
// extension traits

pub(crate) trait VecExt<T> {
    fn find_remove(&mut self, predicate: impl FnMut(&T) -> bool) -> Option<T>;
}

impl<T> VecExt<T> for Vec<T> {
    fn find_remove(&mut self, predicate: impl FnMut(&T) -> bool) -> Option<T> {
        self.iter().position(predicate).map(|i| self.remove(i))
    }
}

// =================================================================================================
// node

pub(crate) trait Node {
    fn visited(&mut self, visitor: &mut impl VisitMut);
}

impl Node for Stmt {
    fn visited(&mut self, visitor: &mut impl VisitMut) {
        visitor.visit_stmt_mut(self);
    }
}

impl Node for Local {
    fn visited(&mut self, visitor: &mut impl VisitMut) {
        visitor.visit_local_mut(self);
    }
}

impl Node for Expr {
    fn visited(&mut self, visitor: &mut impl VisitMut) {
        visitor.visit_expr_mut(self);
    }
}

impl Node for Arm {
    fn visited(&mut self, visitor: &mut impl VisitMut) {
        visitor.visit_arm_mut(self);
    }
}

impl Node for Block {
    fn visited(&mut self, visitor: &mut impl VisitMut) {
        visitor.visit_block_mut(self);
    }
}

impl Node for ItemFn {
    fn visited(&mut self, visitor: &mut impl VisitMut) {
        visitor.visit_item_fn_mut(self);
    }
}

// =================================================================================================
// helper for handling attributes

pub(crate) trait Attrs {
    fn attrs(&self) -> &[Attribute];

    fn any_attr(&self, ident: &str) -> bool {
        self.attrs().iter().any(|attr| attr.path().is_ident(ident))
    }

    fn any_empty_attr(&self, ident: &str) -> bool {
        self.attrs().iter().any(|attr| matches!(&attr.meta, Meta::Path(p) if p.is_ident(ident)))
    }

    fn attrs_mut(&mut self) -> Option<&mut Vec<Attribute>>;

    fn find_remove_attr(&mut self, ident: &str) -> Option<Attribute> {
        self.attrs_mut()?.find_remove(|attr| attr.path().is_ident(ident))
    }
}

impl Attrs for Arm {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }

    fn attrs_mut(&mut self) -> Option<&mut Vec<Attribute>> {
        Some(&mut self.attrs)
    }
}

impl Attrs for Local {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }

    fn attrs_mut(&mut self) -> Option<&mut Vec<Attribute>> {
        Some(&mut self.attrs)
    }
}

impl Attrs for StmtMacro {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }

    fn attrs_mut(&mut self) -> Option<&mut Vec<Attribute>> {
        Some(&mut self.attrs)
    }
}

impl Attrs for Stmt {
    fn attrs(&self) -> &[Attribute] {
        match self {
            Stmt::Expr(expr, _) => expr.attrs(),
            Stmt::Local(local) => local.attrs(),
            Stmt::Macro(mac) => mac.attrs(),
            // Ignore nested items.
            Stmt::Item(_) => &[],
        }
    }

    fn attrs_mut(&mut self) -> Option<&mut Vec<Attribute>> {
        match self {
            Stmt::Expr(expr, _) => expr.attrs_mut(),
            Stmt::Local(local) => local.attrs_mut(),
            Stmt::Macro(mac) => mac.attrs_mut(),
            // Ignore nested items.
            Stmt::Item(_) => None,
        }
    }
}

macro_rules! attrs_impl {
    ($($Expr:ident($Struct:ident),)*) => {
        impl Attrs for Expr {
            fn attrs(&self) -> &[Attribute] {
                // #[cfg_attr(test, deny(non_exhaustive_omitted_patterns))]
                match self {
                    $(Expr::$Expr(syn::$Struct { attrs, .. }))|* => &attrs,
                    _ => &[],
                }
            }

            fn attrs_mut(&mut self) -> Option<&mut Vec<Attribute>> {
                // #[cfg_attr(test, deny(non_exhaustive_omitted_patterns))]
                match self {
                    $(Expr::$Expr(syn::$Struct { attrs, .. }))|* => Some(attrs),
                    _ => None,
                }
            }
        }
    };
}

attrs_impl! {
    Array(ExprArray),
    Assign(ExprAssign),
    Async(ExprAsync),
    Await(ExprAwait),
    Binary(ExprBinary),
    Block(ExprBlock),
    Break(ExprBreak),
    Call(ExprCall),
    Cast(ExprCast),
    Closure(ExprClosure),
    Const(ExprConst),
    Continue(ExprContinue),
    Field(ExprField),
    ForLoop(ExprForLoop),
    Group(ExprGroup),
    If(ExprIf),
    Index(ExprIndex),
    Infer(ExprInfer),
    Let(ExprLet),
    Lit(ExprLit),
    Loop(ExprLoop),
    Macro(ExprMacro),
    Match(ExprMatch),
    MethodCall(ExprMethodCall),
    Paren(ExprParen),
    Path(ExprPath),
    Range(ExprRange),
    Reference(ExprReference),
    Repeat(ExprRepeat),
    Return(ExprReturn),
    Struct(ExprStruct),
    Try(ExprTry),
    TryBlock(ExprTryBlock),
    Tuple(ExprTuple),
    Unary(ExprUnary),
    Unsafe(ExprUnsafe),
    While(ExprWhile),
    Yield(ExprYield),
}
