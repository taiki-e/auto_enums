use syn::{
    parse_quote,
    visit_mut::{self, VisitMut},
    Arm, Attribute, Expr, ExprMacro, ExprReturn, ExprTry, Item, Local, Stmt,
};

use crate::utils::{replace_expr, OptionExt};

use super::*;

// =============================================================================
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
        Self {
            cx,
            scope: Scope::default(),
        }
    }

    fn find_remove_empty_attrs<A: AttrsMut>(&self, attrs: &mut A) {
        if !self.scope.foreign {
            EMPTY_ATTRS.iter().for_each(|ident| {
                attrs.find_remove_empty_attr(ident);
            });
        }
    }

    fn other_attr<A: Attrs>(&mut self, attrs: &A) {
        if attrs.any_attr(NAME) {
            self.scope.foreign = true;
            // Record whether other `auto_enum` exists.
            self.cx.attr = true;
        }
    }

    /// `return` in functions or closures
    fn visit_return(&mut self, expr: &mut Expr) {
        if let Expr::Closure(_) = &expr {
            self.scope.closure = true;
        }

        if !self.scope.closure && !expr.any_empty_attr(NEVER) {
            if let Expr::Return(ExprReturn { expr, .. }) = expr {
                expr.replace_boxed_expr(|expr| match expr {
                    Expr::Macro(expr) => {
                        if self.cx.marker_macro(&expr.mac) {
                            Expr::Macro(expr)
                        } else {
                            self.cx.next_expr(Expr::Macro(expr))
                        }
                    }
                    expr => self.cx.next_expr(expr),
                });
            }
        }
    }

    /// `?` operator in functions or closures
    fn visit_try(&mut self, expr: &mut Expr) {
        match expr {
            Expr::Closure(_) => self.scope.closure = true,
            // `?` operator in try blocks are not supported.
            Expr::TryBlock(_) => self.scope.try_block = true,
            _ => {}
        }

        if !self.scope.try_block && !self.scope.closure && !expr.any_empty_attr(NEVER) {
            *expr = match expr {
                Expr::Try(ExprTry { expr, .. }) => {
                    if let Expr::Macro(ExprMacro { mac, .. }) = &**expr {
                        if self.cx.marker_macro(mac) {
                            return;
                        }
                    }

                    // https://github.com/rust-lang/rust/blob/1.33.0/src/librustc/hir/lowering.rs#L4416-L4514
                    let err = self.cx.next_expr(parse_quote!(err));
                    #[cfg(feature = "try_trait")]
                    {
                        parse_quote! {
                            match ::core::ops::Try::into_result(#expr) {
                                ::core::result::Result::Ok(val) => val,
                                ::core::result::Result::Err(err) => return ::core::ops::Try::from_error(#err),
                            }
                        }
                    }
                    #[cfg(not(feature = "try_trait"))]
                    {
                        parse_quote! {
                            match #expr {
                                ::core::result::Result::Ok(val) => val,
                                ::core::result::Result::Err(err) => return ::core::result::Result::Err(#err),
                            }
                        }
                    }
                }
                _ => return,
            };
        }
    }

    /// Expression level marker (`marker!` macro)
    fn visit_marker(&mut self, expr: &mut Expr) {
        if self.scope.foreign && !self.cx.marker.is_unique() {
            return;
        }

        replace_expr(expr, |expr| match expr {
            Expr::Macro(expr) => {
                if expr.mac.path.is_ident(self.cx.marker.ident()) {
                    let args = syn::parse2(expr.mac.tts).unwrap_or_else(|e| {
                        self.cx.error = true;
                        syn::parse2(e.to_compile_error()).unwrap_or_else(|_| unreachable!())
                    });

                    if self.cx.error {
                        args
                    } else {
                        self.cx.next_expr_with_attrs(expr.attrs, args)
                    }
                } else {
                    Expr::Macro(expr)
                }
            }
            expr => expr,
        });

        self.find_remove_empty_attrs(expr);
    }
}

impl VisitMut for Visitor<'_> {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        if !self.cx.error {
            let tmp = self.scope;
            self.other_attr(expr);

            match self.cx.mode() {
                VisitMode::Return => self.visit_return(expr),
                VisitMode::Try => self.visit_try(expr),
                _ => {}
            }

            visit_mut::visit_expr_mut(self, expr);
            self.visit_marker(expr);
            self.scope = tmp;
        }
    }

    fn visit_arm_mut(&mut self, arm: &mut Arm) {
        if !self.cx.error {
            visit_mut::visit_arm_mut(self, arm);
            self.find_remove_empty_attrs(arm);
        }
    }

    fn visit_local_mut(&mut self, local: &mut Local) {
        if !self.cx.error {
            let tmp = self.scope;
            self.other_attr(local);

            visit_mut::visit_local_mut(self, local);
            self.find_remove_empty_attrs(local);
            self.scope = tmp;
        }
    }

    fn visit_stmt_mut(&mut self, stmt: &mut Stmt) {
        if !self.cx.error {
            visit_mut::visit_stmt_mut(self, stmt);
            visit_stmt_mut(stmt, self.cx);
        }
    }

    // Stop at item bounds
    fn visit_item_mut(&mut self, _item: &mut Item) {}
}

// =============================================================================
// FindTry

/// Find `?` operator.
pub(super) struct FindTry<'a> {
    cx: &'a Context,
    scope: Scope,
    pub(super) has: bool,
}

impl<'a> FindTry<'a> {
    pub(super) fn new(cx: &'a Context) -> Self {
        Self {
            cx,
            scope: Scope::default(),
            has: false,
        }
    }
}

impl VisitMut for FindTry<'_> {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        let tmp = self.scope;

        if let Expr::Closure(_) = &expr {
            self.scope.closure = true;
        }

        if !self.scope.closure && !expr.any_empty_attr(NEVER) {
            if let Expr::Try(ExprTry { expr, .. }) = expr {
                match &**expr {
                    Expr::Macro(expr) => {
                        if !self.cx.marker_macro(&expr.mac) {
                            self.has = true;
                        }
                    }
                    _ => self.has = true,
                }
            }
        }

        if expr.any_attr(NAME) {
            self.scope.foreign = true;
        }
        if !self.has {
            visit_mut::visit_expr_mut(self, expr);
        }

        self.scope = tmp;
    }

    fn visit_local_mut(&mut self, local: &mut Local) {
        let tmp = self.scope;

        if local.any_attr(NAME) {
            self.scope.foreign = true;
        }

        visit_mut::visit_local_mut(self, local);
        self.scope = tmp;
    }

    // Stop at item bounds
    fn visit_item_mut(&mut self, _item: &mut Item) {}
}

// =============================================================================
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
    fn visit_stmt_mut(&mut self, stmt: &mut Stmt) {
        if !self.cx.error {
            visit_mut::visit_stmt_mut(self, stmt);
            visit_stmt_mut(stmt, self.cx);
        }
    }

    // Stop at item bounds
    fn visit_item_mut(&mut self, _item: &mut Item) {}
}

fn visit_stmt_mut(stmt: &mut Stmt, cx: &mut Context) {
    // Stop at item bounds
    if let Stmt::Item(_) = stmt {
        return;
    }

    if let Some(Attribute { tts, .. }) = stmt.find_remove_attr(NAME) {
        parse_group(tts)
            .map(|x| Context::child(span!(stmt), x))
            .and_then(|mut cx| stmt.visit_parent(&mut cx))
            .unwrap_or_else(|e| {
                cx.error = true;
                *stmt = syn::parse2(e.to_compile_error()).unwrap_or_else(|_| unreachable!());
            });
    }
}
