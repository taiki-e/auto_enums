use proc_macro2::Group;
use quote::ToTokens;
use syn::{
    parse::Nothing,
    visit_mut::{self, VisitMut},
    *,
};

use crate::utils::{expr_unimplemented, replace_expr, Attrs, AttrsMut};

use super::{Context, VisitMode, DEFAULT_MARKER, NAME, NESTED, NEVER};

// =================================================================================================
// Visitor

#[derive(Clone, Copy, Default)]
struct Scope {
    /// in closures
    closure: bool,
    /// in try blocks
    try_block: bool,
    /// in the other `auto_enum` attributes
    foreign: bool,
}

pub(super) struct Visitor<'a> {
    cx: &'a mut Context,
    scope: Scope,
}

impl<'a> Visitor<'a> {
    pub(super) fn new(cx: &'a mut Context) -> Self {
        Self { cx, scope: Scope::default() }
    }

    fn find_remove_attrs(&mut self, attrs: &mut impl AttrsMut) {
        if !self.scope.foreign {
            if let Some(attr) = attrs.find_remove_attr(NEVER) {
                if let Err(e) = syn::parse2::<Nothing>(attr.tokens) {
                    self.cx.diagnostic.error(e);
                }
            }

            if let Some(old) = attrs.find_remove_attr(super::NESTED_OLD) {
                self.cx.diagnostic.error(error!(
                    old,
                    "#[{}] has been removed and replaced with #[{}]",
                    super::NESTED_OLD,
                    super::NESTED
                ));
            }
        }
    }

    /// `return` in functions or closures
    fn visit_return(&mut self, node: &mut Expr) {
        debug_assert!(self.cx.visit_mode == VisitMode::Return);

        if !self.scope.closure && !node.any_empty_attr(NEVER) {
            // Desugar `return <expr>` into `return Enum::VariantN(<expr>)`.
            if let Expr::Return(ExprReturn { expr, .. }) = node {
                self.cx.replace_boxed_expr(expr);
            }
        }
    }

    /// `?` operator in functions or closures
    fn visit_try(&mut self, node: &mut Expr) {
        debug_assert!(self.cx.visit_mode == VisitMode::Try);

        if !self.scope.try_block && !self.scope.closure && !node.any_empty_attr(NEVER) {
            match &node {
                // https://github.com/rust-lang/rust/blob/1.35.0/src/librustc/hir/lowering.rs#L4578-L4682

                // Desugar `ExprKind::Try`
                // from: `<expr>?`
                Expr::Try(ExprTry { expr, .. })
                    // Skip if `<expr>` is a marker macro.
                    if !self.cx.is_marker_expr(&**expr) =>
                {
                    // into:
                    //
                    // match <expr> {
                    //     Ok(val) => val,
                    //     Err(err) => return Err(Enum::VariantN(err)),
                    // }

                    replace_expr(node, |expr| {
                        #[allow(unused_mut)]
                        let ExprTry { attrs, mut expr, .. } =
                            if let Expr::Try(expr) = expr { expr } else { unreachable!() };

                        let mut arms = Vec::with_capacity(2);
                        arms.push(syn::parse_quote!(::core::result::Result::Ok(val) => val,));

                        let err = self.cx.next_expr(syn::parse_quote!(err));
                        arms.push(syn::parse_quote!(::core::result::Result::Err(err) => return ::core::result::Result::Err(#err),));

                        Expr::Match(ExprMatch {
                            attrs,
                            match_token: token::Match::default(),
                            expr,
                            brace_token: token::Brace::default(),
                            arms,
                        })
                    })
                }
                _ => {}
            }
        }
    }

    /// `#[nested]`
    fn visit_nested(&mut self, node: &mut Expr, attr: Attribute) {
        debug_assert!(!self.scope.foreign);

        if let Err(e) =
            syn::parse2::<Nothing>(attr.tokens).and_then(|_| super::expr::child_expr(node, self.cx))
        {
            self.cx.diagnostic.error(e);
        }
    }

    /// Expression level marker (`marker!` macro)
    fn visit_marker_macro(&mut self, node: &mut Expr) {
        debug_assert!(!self.scope.foreign || self.cx.marker != DEFAULT_MARKER);

        match &node {
            // Desugar `marker!(<expr>)` into `Enum::VariantN(<expr>)`.
            Expr::Macro(ExprMacro { mac, .. })
                // Skip if `marker!` is not a marker macro.
                if self.cx.is_marker_macro_exact(mac) =>
            {
                replace_expr(node, |expr| {
                    let expr = if let Expr::Macro(expr) = expr { expr } else { unreachable!() };
                    let args = syn::parse2(expr.mac.tokens).unwrap_or_else(|e| {
                        self.cx.diagnostic.error(e);
                        expr_unimplemented()
                    });

                    if self.cx.failed() {
                        args
                    } else {
                        self.cx.next_expr_with_attrs(expr.attrs, args)
                    }
                })
            }
            _ => {}
        }
    }
}

impl VisitMut for Visitor<'_> {
    fn visit_expr_mut(&mut self, node: &mut Expr) {
        if !self.cx.failed() {
            match node {
                Expr::Closure(_) => self.scope.closure = true,
                // `?` operator in try blocks are not supported.
                Expr::TryBlock(_) => self.scope.try_block = true,
                _ => {}
            }

            match self.cx.visit_mode {
                VisitMode::Return => self.visit_return(node),
                VisitMode::Try => self.visit_try(node),
                VisitMode::Default => {}
            }

            if !self.scope.foreign {
                if let Some(attr) = node.find_remove_attr(NESTED) {
                    self.visit_nested(node, attr);
                }
            }

            visit_mut::visit_expr_mut(self, node);

            if !self.scope.foreign || self.cx.marker != DEFAULT_MARKER {
                self.visit_marker_macro(node);
                self.find_remove_attrs(node);
            }
        }
    }

    fn visit_arm_mut(&mut self, node: &mut Arm) {
        if !self.cx.failed() {
            if !self.scope.foreign {
                if let Some(attr) = node.find_remove_attr(NESTED) {
                    self.visit_nested(&mut *node.body, attr);
                }
            }

            visit_mut::visit_arm_mut(self, node);

            self.find_remove_attrs(node);
        }
    }

    fn visit_local_mut(&mut self, node: &mut Local) {
        if !self.cx.failed() {
            if !self.scope.foreign {
                if let Some(attr) = node.find_remove_attr(NESTED) {
                    if let Some((_, expr)) = &mut node.init {
                        self.visit_nested(&mut **expr, attr);
                    }
                }
            }

            visit_mut::visit_local_mut(self, node);

            self.find_remove_attrs(node);
        }
    }

    fn visit_stmt_mut(&mut self, node: &mut Stmt) {
        if !self.cx.failed() {
            let tmp = self.scope;

            if node.any_attr(NAME) {
                self.scope.foreign = true;
                // Record whether other `auto_enum` attribute exists.
                self.cx.other_attr = true;
            }

            visit_stmt(self, node);

            self.scope = tmp;
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
// Dummy visitor

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
        if !self.cx.failed() {
            if node.any_attr(NAME) {
                self.cx.other_attr = true;
            }

            visit_stmt(self, node);
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
}

fn visit_stmt(visitor: &mut impl VisitStmt, node: &mut Stmt) {
    let attr = match node {
        Stmt::Expr(expr) | Stmt::Semi(expr, _) => expr.find_remove_attr(NAME),
        Stmt::Local(local) => local.find_remove_attr(NAME),
        // Do not recurse into nested items.
        Stmt::Item(_) => None,
    };

    if let Some(Attribute { tokens, .. }) = attr {
        let res = syn::parse2::<Group>(tokens)
            .and_then(|group| visitor.cx().make_child(node.to_token_stream(), group.stream()));

        visit_mut::visit_stmt_mut(visitor, node);

        match res {
            Err(e) => {
                visitor.cx().diagnostic.error(e);
                *node = Stmt::Expr(expr_unimplemented());
            }
            Ok(mut cx) => {
                super::expand_parent_stmt(node, &mut cx).unwrap_or_else(|e| {
                    cx.diagnostic.error(e);
                    *node = Stmt::Expr(expr_unimplemented());
                });
                visitor.cx().join_child(cx)
            }
        }
    } else {
        visit_mut::visit_stmt_mut(visitor, node);
    }
}

// =================================================================================================
// FindNested

/// Find `#[nested]` attribute.
pub(super) struct FindNested {
    pub(super) has: bool,
}

impl FindNested {
    pub(super) fn new() -> Self {
        Self { has: false }
    }
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

// =================================================================================================
// FindTry

/// Find `?` operator.
pub(super) struct FindTry<'a> {
    cx: &'a Context,
    scope: Scope,
    pub(super) has: bool,
}

impl<'a> FindTry<'a> {
    pub(super) fn new(cx: &'a Context) -> Self {
        Self { cx, scope: Scope::default(), has: false }
    }
}

impl VisitMut for FindTry<'_> {
    fn visit_expr_mut(&mut self, node: &mut Expr) {
        let tmp = self.scope;

        if let Expr::Closure(_) = &node {
            self.scope.closure = true;
        }

        if !self.scope.closure && !node.any_empty_attr(NEVER) {
            if let Expr::Try(ExprTry { expr, .. }) = node {
                // Skip if `<expr>` is a marker macro.
                if !self.cx.is_marker_expr(&**expr) {
                    self.has = true;
                }
            }
        }

        if node.any_attr(NAME) {
            self.scope.foreign = true;
        }
        if !self.has {
            visit_mut::visit_expr_mut(self, node);
        }

        self.scope = tmp;
    }

    fn visit_local_mut(&mut self, node: &mut Local) {
        let tmp = self.scope;

        if node.any_attr(NAME) {
            self.scope.foreign = true;
        }

        visit_mut::visit_local_mut(self, node);
        self.scope = tmp;
    }

    fn visit_item_mut(&mut self, _: &mut Item) {
        // Do not recurse into nested items.
    }
}
