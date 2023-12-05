// SPDX-License-Identifier: Apache-2.0 OR MIT

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse_quote, token,
    visit_mut::{self, VisitMut},
    Arm, Attribute, Expr, ExprMacro, ExprMatch, ExprReturn, ExprTry, Item, Local, LocalInit,
    MetaList, Stmt, Token,
};

use super::{Context, VisitMode, DEFAULT_MARKER, NAME, NESTED, NEVER};
use crate::utils::{replace_expr, Attrs, Node};

#[derive(Clone, Copy, Default)]
struct Scope {
    /// in closures
    closure: bool,
    /// in try blocks
    try_block: bool,
    /// in the other `auto_enum` attributes
    foreign: bool,
}

impl Scope {
    // check this scope is in closures or try blocks.
    fn check_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Closure(_) => self.closure = true,
            // `?` operator in try blocks are not supported.
            Expr::TryBlock(_) => self.try_block = true,
            _ => {}
        }
    }
}

// =================================================================================================
// default visitor

pub(super) struct Visitor<'a> {
    cx: &'a mut Context,
    scope: Scope,
}

impl<'a> Visitor<'a> {
    pub(super) fn new(cx: &'a mut Context) -> Self {
        Self { cx, scope: Scope::default() }
    }

    fn find_remove_attrs(&mut self, attrs: &mut impl Attrs) {
        if !self.scope.foreign {
            if let Some(attr) = attrs.find_remove_attr(NEVER) {
                if let Err(e) = attr.meta.require_path_only() {
                    self.cx.error(e);
                }
            }

            // The old annotation `#[rec]` is replaced with `#[nested]`.
            if let Some(old) = attrs.find_remove_attr("rec") {
                self.cx.error(format_err!(
                    old,
                    "#[rec] has been removed and replaced with #[{}]",
                    NESTED
                ));
            }
        }
    }

    /// `return` in functions or closures
    fn visit_return(&mut self, node: &mut Expr, count: usize) {
        debug_assert!(self.cx.visit_mode == VisitMode::Return(count));

        if !self.scope.closure && !node.any_empty_attr(NEVER) {
            // Desugar `return <expr>` into `return Enum::VariantN(<expr>)`.
            if let Expr::Return(ExprReturn { expr, .. }) = node {
                // Skip if `<expr>` is a marker macro.
                if expr.as_ref().map_or(true, |expr| !self.cx.is_marker_expr(expr)) {
                    self.cx.replace_boxed_expr(expr);
                }
            }
        }
    }

    /// `?` operator in functions or closures
    fn visit_try(&mut self, node: &mut Expr) {
        debug_assert!(self.cx.visit_mode == VisitMode::Try);

        if !self.scope.try_block && !self.scope.closure && !node.any_empty_attr(NEVER) {
            match &node {
                // https://github.com/rust-lang/rust/blob/1.35.0/src/librustc/hir/lowering.rs#L4578-L4682

                // Desugar `<expr>?`
                // into:
                //
                // match <expr> {
                //     Ok(val) => val,
                //     Err(err) => return Err(Enum::VariantN(err)),
                // }
                //
                // Skip if `<expr>` is a marker macro.
                Expr::Try(ExprTry { expr, .. }) if !self.cx.is_marker_expr(expr) => {
                    replace_expr(node, |expr| {
                        let ExprTry { attrs, expr, .. } =
                            if let Expr::Try(expr) = expr { expr } else { unreachable!() };

                        let err = self.cx.next_expr(parse_quote!(err));
                        let arms = vec![
                            parse_quote! {
                                ::core::result::Result::Ok(val) => val,
                            },
                            parse_quote! {
                                ::core::result::Result::Err(err) => {
                                    return ::core::result::Result::Err(#err);
                                }
                            },
                        ];

                        Expr::Match(ExprMatch {
                            attrs,
                            match_token: <Token![match]>::default(),
                            expr,
                            brace_token: token::Brace::default(),
                            arms,
                        })
                    });
                }
                _ => {}
            }
        }
    }

    /// `#[nested]`
    fn visit_nested(&mut self, node: &mut Expr, attr: &Attribute) {
        debug_assert!(!self.scope.foreign);

        if let Err(e) = attr.meta.require_path_only() {
            self.cx.error(e);
        } else {
            super::expr::child_expr(self.cx, node);
        }
    }

    /// Expression level marker (`marker!` macro)
    fn visit_marker_macro(&mut self, node: &mut Expr) {
        debug_assert!(!self.scope.foreign || self.cx.current_marker != DEFAULT_MARKER);

        match node {
            // Desugar `marker!(<expr>)` into `Enum::VariantN(<expr>)`.
            // Skip if `marker!` is not a marker macro.
            Expr::Macro(ExprMacro { mac, .. }) if self.cx.is_marker_macro_exact(mac) => {
                replace_expr(node, |expr| {
                    let expr = if let Expr::Macro(expr) = expr { expr } else { unreachable!() };
                    let args = syn::parse2(expr.mac.tokens).unwrap_or_else(|e| {
                        self.cx.error(e);
                        // Generate an expression to fill in where the error occurred during the visit.
                        // These will eventually need to be replaced with the original error message.
                        parse_quote!(compile_error!(
                            "#[auto_enum] failed to generate error message"
                        ))
                    });

                    if self.cx.has_error() {
                        args
                    } else {
                        self.cx.next_expr_with_attrs(expr.attrs, args)
                    }
                });
            }
            _ => {}
        }
    }

    fn visit_expr(&mut self, node: &mut Expr, has_semi: bool) {
        debug_assert!(!self.cx.has_error());

        let tmp = self.scope;

        if node.any_attr(NAME) {
            self.scope.foreign = true;
            // Record whether other `auto_enum` attribute exists.
            self.cx.has_child = true;
        }
        self.scope.check_expr(node);

        match self.cx.visit_mode {
            VisitMode::Return(count) => self.visit_return(node, count),
            VisitMode::Try => self.visit_try(node),
            VisitMode::Default => {}
        }

        if !self.scope.foreign {
            if let Some(attr) = node.find_remove_attr(NESTED) {
                self.visit_nested(node, &attr);
            }
        }

        VisitStmt::visit_expr(self, node, has_semi);

        if !self.scope.foreign || self.cx.current_marker != DEFAULT_MARKER {
            self.visit_marker_macro(node);
            self.find_remove_attrs(node);
        }

        self.scope = tmp;
    }
}

impl VisitMut for Visitor<'_> {
    fn visit_expr_mut(&mut self, node: &mut Expr) {
        if !self.cx.has_error() {
            self.visit_expr(node, false);
        }
    }

    fn visit_arm_mut(&mut self, node: &mut Arm) {
        if !self.cx.has_error() {
            if !self.scope.foreign {
                if let Some(attr) = node.find_remove_attr(NESTED) {
                    self.visit_nested(&mut node.body, &attr);
                }
            }

            visit_mut::visit_arm_mut(self, node);

            self.find_remove_attrs(node);
        }
    }

    fn visit_local_mut(&mut self, node: &mut Local) {
        if !self.cx.has_error() {
            if !self.scope.foreign {
                if let Some(attr) = node.find_remove_attr(NESTED) {
                    if let Some(LocalInit { expr, .. }) = &mut node.init {
                        self.visit_nested(expr, &attr);
                    }
                }
            }

            visit_mut::visit_local_mut(self, node);

            self.find_remove_attrs(node);
        }
    }

    fn visit_stmt_mut(&mut self, node: &mut Stmt) {
        if !self.cx.has_error() {
            if let Stmt::Expr(expr, semi) = node {
                self.visit_expr(expr, semi.is_some());
            } else {
                let tmp = self.scope;

                if node.any_attr(NAME) {
                    self.scope.foreign = true;
                    // Record whether other `auto_enum` attribute exists.
                    self.cx.has_child = true;
                }

                VisitStmt::visit_stmt(self, node);

                self.scope = tmp;
            }
        }
    }

    fn visit_item_mut(&mut self, _: &mut Item) {
        // Do not recurse into nested items.
    }
}

impl VisitStmt for Visitor<'_> {
    fn cx(&mut self) -> &mut Context {
        self.cx
    }
}

// =================================================================================================
// dummy visitor

pub(super) struct Dummy<'a> {
    cx: &'a mut Context,
}

impl<'a> Dummy<'a> {
    pub(super) fn new(cx: &'a mut Context) -> Self {
        Self { cx }
    }
}

impl VisitMut for Dummy<'_> {
    fn visit_stmt_mut(&mut self, node: &mut Stmt) {
        if !self.cx.has_error() {
            if node.any_attr(NAME) {
                self.cx.has_child = true;
            }
            VisitStmt::visit_stmt(self, node);
        }
    }

    fn visit_expr_mut(&mut self, node: &mut Expr) {
        if !self.cx.has_error() {
            if node.any_attr(NAME) {
                self.cx.has_child = true;
            }
            VisitStmt::visit_expr(self, node, false);
        }
    }

    fn visit_item_mut(&mut self, _: &mut Item) {
        // Do not recurse into nested items.
    }
}

impl VisitStmt for Dummy<'_> {
    fn cx(&mut self) -> &mut Context {
        self.cx
    }
}

// =================================================================================================
// VisitStmt

trait VisitStmt: VisitMut {
    fn cx(&mut self) -> &mut Context;

    fn visit_expr(visitor: &mut Self, node: &mut Expr, has_semi: bool) {
        let attr = node.find_remove_attr(NAME);

        let res = attr.map(|attr| {
            attr.meta.require_list().and_then(|MetaList { tokens, .. }| {
                visitor.cx().make_child(node.to_token_stream(), tokens.clone())
            })
        });

        visit_mut::visit_expr_mut(visitor, node);

        match res {
            Some(Err(e)) => visitor.cx().error(e),
            Some(Ok(mut cx)) => {
                super::expand_parent_expr(&mut cx, node, has_semi);
                visitor.cx().join_child(cx);
            }
            None => {}
        }
    }

    fn visit_stmt(visitor: &mut Self, node: &mut Stmt) {
        let attr = match node {
            Stmt::Expr(expr, semi) => {
                Self::visit_expr(visitor, expr, semi.is_some());
                return;
            }
            Stmt::Local(local) => local.find_remove_attr(NAME),
            Stmt::Macro(_) => None,
            // Do not recurse into nested items.
            Stmt::Item(_) => return,
        };

        let res = attr.map(|attr| {
            let args = match attr.meta {
                syn::Meta::Path(_) => TokenStream::new(),
                syn::Meta::List(list) => list.tokens,
                syn::Meta::NameValue(nv) => bail!(nv.eq_token, "expected list"),
            };
            visitor.cx().make_child(node.to_token_stream(), args)
        });

        visit_mut::visit_stmt_mut(visitor, node);

        match res {
            Some(Err(e)) => visitor.cx().error(e),
            Some(Ok(mut cx)) => {
                super::expand_parent_stmt(&mut cx, node);
                visitor.cx().join_child(cx);
            }
            None => {}
        }
    }
}

// =================================================================================================
// FindNested

/// Find `#[nested]` attribute.
pub(super) fn find_nested(node: &mut impl Node) -> bool {
    struct FindNested {
        has: bool,
    }

    impl VisitMut for FindNested {
        fn visit_expr_mut(&mut self, node: &mut Expr) {
            if !node.any_attr(NAME) {
                if node.any_empty_attr(NESTED) {
                    self.has = true;
                } else {
                    visit_mut::visit_expr_mut(self, node);
                }
            }
        }

        fn visit_arm_mut(&mut self, node: &mut Arm) {
            if node.any_empty_attr(NESTED) {
                self.has = true;
            } else {
                visit_mut::visit_arm_mut(self, node);
            }
        }

        fn visit_local_mut(&mut self, node: &mut Local) {
            if !node.any_attr(NAME) {
                if node.any_empty_attr(NESTED) {
                    self.has = true;
                } else {
                    visit_mut::visit_local_mut(self, node);
                }
            }
        }

        fn visit_item_mut(&mut self, _: &mut Item) {
            // Do not recurse into nested items.
        }
    }

    let mut visitor = FindNested { has: false };
    node.visited(&mut visitor);
    visitor.has
}

// =================================================================================================
// FnVisitor

#[derive(Default)]
pub(super) struct FnCount {
    pub(super) try_: usize,
    pub(super) return_: usize,
}

pub(super) fn visit_fn(cx: &Context, node: &mut impl Node) -> FnCount {
    struct FnVisitor<'a> {
        cx: &'a Context,
        scope: Scope,
        count: FnCount,
    }

    impl VisitMut for FnVisitor<'_> {
        fn visit_expr_mut(&mut self, node: &mut Expr) {
            let tmp = self.scope;

            self.scope.check_expr(node);

            if !self.scope.closure && !node.any_empty_attr(NEVER) {
                match node {
                    Expr::Try(ExprTry { expr, .. }) => {
                        // Skip if `<expr>` is a marker macro.
                        if !self.cx.is_marker_expr(expr) {
                            self.count.try_ += 1;
                        }
                    }
                    Expr::Return(ExprReturn { expr, .. }) => {
                        // Skip if `<expr>` is a marker macro.
                        if expr.as_ref().map_or(true, |expr| !self.cx.is_marker_expr(expr)) {
                            self.count.return_ += 1;
                        }
                    }
                    _ => {}
                }
            }

            if node.any_attr(NAME) {
                self.scope.foreign = true;
            }

            visit_mut::visit_expr_mut(self, node);

            self.scope = tmp;
        }

        fn visit_stmt_mut(&mut self, node: &mut Stmt) {
            let tmp = self.scope;

            if node.any_attr(NAME) {
                self.scope.foreign = true;
            }

            visit_mut::visit_stmt_mut(self, node);

            self.scope = tmp;
        }

        fn visit_item_mut(&mut self, _: &mut Item) {
            // Do not recurse into nested items.
        }
    }

    let mut visitor = FnVisitor { cx, scope: Scope::default(), count: FnCount::default() };
    node.visited(&mut visitor);
    visitor.count
}
