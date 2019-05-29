use std::ops::{Deref, DerefMut};

use syn::{
    visit_mut::{self, VisitMut},
    *,
};

use crate::utils::{default, expr_block, replace_block, replace_expr, OptionExt};

use super::*;

// =============================================================================
// Context

struct Context<'a> {
    cx: &'a mut super::Context,
    rec: bool,
}

// To avoid `cx.cx`
impl Deref for Context<'_> {
    type Target = super::Context;

    fn deref(&self) -> &Self::Target {
        self.cx
    }
}

impl DerefMut for Context<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.cx
    }
}

impl<'a> From<&'a mut super::Context> for Context<'a> {
    fn from(cx: &'a mut super::Context) -> Self {
        Self { cx, rec: false }
    }
}

// =============================================================================
// Visiting last expression

fn last_expr<T, F, OP>(expr: &Expr, success: T, mut filter: F, op: OP) -> T
where
    F: FnMut(&Expr) -> bool,
    OP: FnOnce(&Expr) -> T,
{
    if !filter(expr) {
        return success;
    }

    match expr {
        Expr::Block(ExprBlock { block, .. }) | Expr::Unsafe(ExprUnsafe { block, .. }) => {
            match block.stmts.last() {
                Some(Stmt::Expr(expr)) => return last_expr(expr, success, filter, op),
                Some(Stmt::Semi(expr, _)) => {
                    if !filter(expr) {
                        return success;
                    }
                }
                Some(_) => return success,
                None => {}
            }
        }
        _ => {}
    }

    op(expr)
}

fn last_expr_mut<T, F, OP>(
    expr: &mut Expr,
    cx: &mut Context<'_>,
    success: T,
    mut filter: F,
    op: OP,
) -> T
where
    F: FnMut(&Expr, &mut Context<'_>) -> bool,
    OP: FnOnce(&mut Expr, &mut Context<'_>) -> T,
{
    if !filter(expr, cx) {
        return success;
    }

    match expr {
        Expr::Block(ExprBlock { block, .. }) | Expr::Unsafe(ExprUnsafe { block, .. }) => {
            match block.stmts.last_mut() {
                Some(Stmt::Expr(expr)) => return last_expr_mut(expr, cx, success, filter, op),
                Some(Stmt::Semi(expr, _)) => {
                    if !filter(expr, cx) {
                        return success;
                    }
                }
                Some(_) => return success,
                None => {}
            }
        }
        _ => {}
    }

    op(expr, cx)
}

fn is_unreachable(expr: &Expr, cx: &Context<'_>) -> bool {
    const UNREACHABLE_MACROS: &[&str] = &["unreachable", "panic"];

    last_expr(
        expr,
        true,
        |expr| !expr.any_empty_attr(NEVER) && !expr.any_attr(NAME),
        |expr| match expr {
            Expr::Break(_) | Expr::Continue(_) | Expr::Return(_) => true,

            // `unreachable!`, `panic!` or Expression level marker (`marker!` macro).
            Expr::Macro(ExprMacro { mac, .. }) => {
                UNREACHABLE_MACROS.iter().any(|i| mac.path.is_ident(i)) || cx.marker_macro(mac)
            }

            /* FIXME: This may not be necessary.
            // Assigned.
            Expr::Call(expr) => cx.assigned_enum(expr),
            */
            Expr::Match(ExprMatch { arms, .. }) => {
                arms.iter().all(|arm| arm.any_empty_attr(NEVER) || is_unreachable(&*arm.body, cx))
            }

            // `Err(expr)?` or `None?`.
            Expr::Try(ExprTry { expr, .. }) => match &**expr {
                Expr::Path(ExprPath { path, qself: None, .. }) => path.is_ident("None"),
                Expr::Call(ExprCall { args, func, .. }) if args.len() == 1 => match &**func {
                    Expr::Path(ExprPath { path, qself: None, .. }) => path.is_ident("Err"),
                    _ => false,
                },
                _ => false,
            },

            _ => false,
        },
    )
}

/// Note that do not use this after `cx.visitor()`.
pub(super) fn child_expr(expr: &mut Expr, cx: &mut super::Context) -> Result<()> {
    impl VisitLast<()> for Expr {
        fn visit_last(&mut self, cx: &mut Context<'_>) -> Result<()> {
            last_expr_mut(
                self,
                cx,
                Ok(()),
                |expr, cx| {
                    if expr.any_empty_attr(NESTED) {
                        cx.rec = true;
                    }
                    !is_unreachable(expr, cx)
                },
                |expr, cx| match expr {
                    Expr::Match(expr) => expr.visit_last(cx),
                    Expr::If(expr) => expr.visit_last(cx),
                    Expr::Loop(expr) => expr.visit_last(cx),

                    // Search recursively
                    Expr::MethodCall(ExprMethodCall { receiver: expr, .. })
                    | Expr::Paren(ExprParen { expr, .. })
                    | Expr::Type(ExprType { expr, .. }) => expr.visit_last(cx),

                    _ => Ok(()),
                },
            )
        }
    }

    if cx.visit_last() {
        expr.visit_last(&mut Context::from(cx))
    } else {
        Ok(())
    }
}

trait VisitLast<T> {
    fn visit_last(&mut self, cx: &mut Context<'_>) -> Result<T>;
}

impl VisitLast<bool> for Expr {
    fn visit_last(&mut self, cx: &mut Context<'_>) -> Result<bool> {
        last_expr_mut(
            self,
            cx,
            Ok(true),
            |expr, cx| !is_unreachable(expr, cx),
            |expr, cx| match expr {
                Expr::Match(expr) => expr.visit_last(cx).map(|()| true),
                Expr::If(expr) => expr.visit_last(cx).map(|()| true),
                Expr::Loop(expr) => expr.visit_last(cx).map(|()| true),
                _ => Ok(false),
            },
        )
    }
}

impl VisitLast<()> for ExprMatch {
    fn visit_last(&mut self, cx: &mut Context<'_>) -> Result<()> {
        fn skip(arm: &mut Arm, cx: &mut Context<'_>) -> Result<bool> {
            Ok(arm.any_empty_attr(NEVER)
                || is_unreachable(&*arm.body, cx)
                || ((arm.any_empty_attr(NESTED) || cx.rec) && arm.body.visit_last(cx)?))
        }

        self.arms.iter_mut().try_for_each(|arm| {
            if !skip(arm, cx)? {
                arm.comma = Some(default());
                replace_expr(&mut *arm.body, |x| cx.next_expr(x));
            }

            Ok(())
        })
    }
}

impl VisitLast<()> for ExprIf {
    fn visit_last(&mut self, cx: &mut Context<'_>) -> Result<()> {
        #[allow(clippy::needless_pass_by_value)]
        fn skip(last: Option<&mut Stmt>, cx: &mut Context<'_>) -> Result<bool> {
            Ok(match &last {
                Some(Stmt::Expr(expr)) | Some(Stmt::Semi(expr, _)) => is_unreachable(expr, cx),
                _ => true,
            } || match last {
                Some(Stmt::Expr(expr)) => {
                    (expr.any_empty_attr(NESTED) || cx.rec) && expr.visit_last(cx)?
                }
                _ => true,
            })
        }

        if !skip(self.then_branch.stmts.last_mut(), cx)? {
            replace_block(&mut self.then_branch, |b| cx.next_expr(expr_block(b)));
        }

        match self.else_branch.as_mut().map(|(_, expr)| &mut **expr) {
            Some(Expr::Block(expr)) => {
                if !skip(expr.block.stmts.last_mut(), cx)? {
                    replace_block(&mut expr.block, |b| cx.next_expr(expr_block(b)));
                }

                Ok(())
            }
            Some(Expr::If(expr)) => expr.visit_last(cx),

            None => Err(err!(self, "`if` expression missing an else clause")),
            // FIXME: This may not be necessary.
            Some(expr) => Err(err!(expr, "after of `else` required `{` or `if`")),
        }
    }
}

impl VisitLast<()> for ExprLoop {
    fn visit_last(&mut self, cx: &mut Context<'_>) -> Result<()> {
        LoopVisitor::new(self, cx).visit_block_mut(&mut self.body);

        Ok(())
    }
}

struct LoopVisitor<'a> {
    cx: &'a mut super::Context,
    label: Option<Lifetime>,
    nested: bool,
}

impl<'a> LoopVisitor<'a> {
    fn new(expr: &ExprLoop, cx: &'a mut super::Context) -> Self {
        Self { cx, label: expr.label.as_ref().map(|l| l.name.clone()), nested: false }
    }

    fn compare_labels(&self, other: Option<&Lifetime>) -> bool {
        match (&self.label, other) {
            (None, None) => true,
            (Some(x), Some(y)) => x.ident == y.ident,
            _ => false,
        }
    }
}

impl VisitMut for LoopVisitor<'_> {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        if expr.any_empty_attr(NEVER) {
            return;
        }

        let tmp = self.nested;
        match expr {
            // Stop at closure bounds
            Expr::Closure(_) => return,

            // Other loop bounds
            Expr::Loop(_) | Expr::ForLoop(_) | Expr::While(_) => {
                if self.label.is_none() {
                    return;
                }
                self.nested = true;
            }

            // `break` in loop
            Expr::Break(ExprBreak { label, expr, .. }) => {
                if (!self.nested && label.is_none()) || self.compare_labels(label.as_ref()) {
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

            _ => {}
        }

        visit_mut::visit_expr_mut(self, expr);
        self.nested = tmp;
    }

    // Stop at item bounds
    fn visit_item_mut(&mut self, _item: &mut Item) {}
}
