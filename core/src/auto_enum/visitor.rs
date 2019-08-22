use proc_macro2::Group;
use syn::{
    parse::Nothing,
    token,
    visit_mut::{self, VisitMut},
    Arm, Attribute, Expr, ExprMacro, ExprMatch, ExprReturn, ExprTry, Item, Local, Stmt,
};

#[cfg(feature = "try_trait")]
use crate::utils::expr_call;
use crate::utils::{expr_unimplemented, replace_expr, Attrs, AttrsMut};

use super::{parse_args, Context, Parent, VisitMode, DEFAULT_MARKER, NAME, NEVER};

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
            super::EMPTY_ATTRS.iter().for_each(|ident| {
                if let Some(attr) = attrs.find_remove_attr(ident) {
                    if let Err(e) = syn::parse2::<Nothing>(attr.tokens) {
                        self.cx.diagnostic.error(e);
                    }
                }
            });

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
                    // match // If "try_trait" feature enabled
                    //       Try::into_result(<expr>)
                    //       // Otherwise
                    //       <expr>
                    // {
                    //     Ok(val) => val,
                    //     Err(err) => // If "try_trait" feature enabled
                    //                 return Try::from_error(Enum::VariantN(err)),
                    //                 // Otherwise
                    //                 return Err(Enum::VariantN(err)),
                    // }

                    replace_expr(node, |expr| {
                        #[allow(unused_mut)]
                        let ExprTry { attrs, mut expr, .. } =
                            if let Expr::Try(expr) = expr { expr } else { unreachable!() };

                        #[cfg(feature = "try_trait")]
                        replace_expr(&mut *expr, |expr| {
                            expr_call(
                                Vec::new(),
                                syn::parse_quote!(::core::ops::Try::into_result),
                                expr,
                            )
                        });

                        let mut arms = Vec::with_capacity(2);
                        arms.push(syn::parse_quote!(::core::result::Result::Ok(val) => val,));

                        let err = self.cx.next_expr(syn::parse_quote!(err));
                        #[cfg(feature = "try_trait")]
                        arms.push(syn::parse_quote!(::core::result::Result::Err(err) => return ::core::ops::Try::from_error(#err),));
                        #[cfg(not(feature = "try_trait"))]
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

            visit_mut::visit_expr_mut(self, node);

            if !self.scope.foreign || self.cx.marker != DEFAULT_MARKER {
                self.visit_marker_macro(node);
                self.find_remove_attrs(node);
            }
        }
    }

    fn visit_arm_mut(&mut self, node: &mut Arm) {
        if !self.cx.failed() {
            visit_mut::visit_arm_mut(self, node);
            self.find_remove_attrs(node);
        }
    }

    fn visit_local_mut(&mut self, node: &mut Local) {
        if !self.cx.failed() {
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

            visit_stmt(node, self, |this| this.cx);

            self.scope = tmp;
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

            visit_stmt(node, self, |this| this.cx);
        }
    }

    fn visit_item_mut(&mut self, _: &mut Item) {
        // Do not recurse into nested items.
    }
}

fn visit_stmt<V>(node: &mut Stmt, visitor: &mut V, f: impl Fn(&mut V) -> &mut Context)
where
    V: VisitMut,
{
    let attr = match node {
        Stmt::Expr(expr) | Stmt::Semi(expr, _) => expr.find_remove_attr(NAME),
        Stmt::Local(local) => local.find_remove_attr(NAME),
        // Do not recurse into nested items.
        Stmt::Item(_) => None,
    };

    if let Some(Attribute { tokens, .. }) = attr {
        let res = syn::parse2(tokens)
            .and_then(|group: Group| parse_args(group.stream()))
            .and_then(|x| f(visitor).make_child(&node, x));

        visit_mut::visit_stmt_mut(visitor, node);

        match res {
            Err(e) => {
                f(visitor).diagnostic.error(e);
                *node = Stmt::Expr(expr_unimplemented());
            }
            Ok(mut cx) => {
                node.expand_parent(&mut cx).unwrap_or_else(|e| {
                    f(visitor).diagnostic.error(e);
                    *node = Stmt::Expr(expr_unimplemented());
                });
            }
        }
    } else {
        visit_mut::visit_stmt_mut(visitor, node);
    }
}
