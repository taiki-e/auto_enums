use std::ops::{Deref, DerefMut};

use syn::{
    visit_mut::{self, VisitMut},
    *,
};

use crate::utils::{expr_block, replace_block, replace_expr};

use super::{Attrs, NAME, NESTED, NEVER};

/// Visits last expression.
///
/// Note that do not use this after `cx.visitor()`.
pub(super) fn child_expr(expr: &mut Expr, cx: &mut super::Context) -> Result<()> {
    fn child_expr_inner(expr: &mut Expr, cx: &mut Context<'_>) -> Result<()> {
        cx.last_expr_mut(
            expr,
            (),
            |expr, cx| {
                if expr.any_empty_attr(NESTED) {
                    cx.nested = true;
                }
                !cx.is_unreachable(expr)
            },
            |expr, cx| match expr {
                Expr::Match(expr) => cx.visit_last_expr_match(expr),
                Expr::If(expr) => cx.visit_last_expr_if(expr),
                Expr::Loop(expr) => cx.visit_last_expr_loop(expr),

                // Search recursively
                Expr::MethodCall(ExprMethodCall { receiver: expr, .. })
                | Expr::Paren(ExprParen { expr, .. })
                | Expr::Type(ExprType { expr, .. }) => child_expr_inner(expr, cx),

                _ => Ok(()),
            },
        )
    }

    if cx.visit_last() { child_expr_inner(expr, &mut Context::from(cx)) } else { Ok(()) }
}

// =================================================================================================
// Context

struct Context<'a> {
    cx: &'a mut super::Context,
    nested: bool,
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
        Self { cx, nested: false }
    }
}

impl Context<'_> {
    fn last_expr_mut<T>(
        &mut self,
        expr: &mut Expr,
        success: T,
        mut filter: impl FnMut(&Expr, &mut Self) -> bool,
        f: impl FnOnce(&mut Expr, &mut Self) -> Result<T>,
    ) -> Result<T> {
        if !filter(expr, self) {
            return Ok(success);
        }

        match expr {
            Expr::Block(ExprBlock { block, .. }) | Expr::Unsafe(ExprUnsafe { block, .. }) => {
                match block.stmts.last_mut() {
                    Some(Stmt::Expr(expr)) => return self.last_expr_mut(expr, success, filter, f),
                    Some(Stmt::Semi(expr, _)) => {
                        if !filter(expr, self) {
                            return Ok(success);
                        }
                    }
                    Some(_) => return Ok(success),
                    None => {}
                }
            }
            _ => {}
        }

        f(expr, self)
    }

    fn is_unreachable(&self, expr: &Expr) -> bool {
        const UNREACHABLE_MACROS: &[&str] = &["unreachable", "panic"];

        fn filter(expr: &Expr) -> bool {
            !expr.any_empty_attr(NEVER) && !expr.any_attr(NAME)
        }

        if !filter(expr) {
            return true;
        }

        match expr {
            Expr::Block(ExprBlock { block, .. }) | Expr::Unsafe(ExprUnsafe { block, .. }) => {
                match block.stmts.last() {
                    Some(Stmt::Expr(expr)) => return self.is_unreachable(expr),
                    Some(Stmt::Semi(expr, _)) if !filter(expr) => return true,
                    Some(_) => return true,
                    None => {}
                }
            }
            _ => {}
        }

        match expr {
            Expr::Break(_) | Expr::Continue(_) | Expr::Return(_) => true,

            // `unreachable!`, `panic!` or an expression level marker (`marker!` macro).
            Expr::Macro(ExprMacro { mac, .. }) => {
                UNREACHABLE_MACROS.iter().any(|i| mac.path.is_ident(i)) || self.is_marker_macro(mac)
            }

            Expr::Match(ExprMatch { arms, .. }) => {
                arms.iter().all(|arm| arm.any_empty_attr(NEVER) || self.is_unreachable(&*arm.body))
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
        }
    }

    fn visit_last_expr(&mut self, expr: &mut Expr) -> Result<bool> {
        self.last_expr_mut(
            expr,
            true,
            |expr, cx| !cx.is_unreachable(expr),
            |expr, cx| match expr {
                Expr::Match(expr) => cx.visit_last_expr_match(expr).map(|()| true),
                Expr::If(expr) => cx.visit_last_expr_if(expr).map(|()| true),
                Expr::Loop(expr) => cx.visit_last_expr_loop(expr).map(|()| true),
                _ => Ok(false),
            },
        )
    }

    fn visit_last_expr_match(&mut self, expr: &mut ExprMatch) -> Result<()> {
        fn skip(arm: &mut Arm, cx: &mut Context<'_>) -> Result<bool> {
            Ok(arm.any_empty_attr(NEVER)
                || cx.is_unreachable(&*arm.body)
                || ((arm.any_empty_attr(NESTED) || cx.nested)
                    && cx.visit_last_expr(&mut arm.body)?))
        }

        expr.arms.iter_mut().try_for_each(|arm| {
            if !skip(arm, self)? {
                arm.comma = Some(token::Comma::default());
                replace_expr(&mut *arm.body, |x| self.next_expr(x));
            }
            Ok(())
        })
    }

    fn visit_last_expr_if(&mut self, expr: &mut ExprIf) -> Result<()> {
        #[allow(clippy::needless_pass_by_value)]
        fn skip(last: Option<&mut Stmt>, cx: &mut Context<'_>) -> Result<bool> {
            Ok(match &last {
                Some(Stmt::Expr(expr)) | Some(Stmt::Semi(expr, _)) => cx.is_unreachable(expr),
                _ => true,
            } || match last {
                Some(Stmt::Expr(expr)) => {
                    (expr.any_empty_attr(NESTED) || cx.nested) && cx.visit_last_expr(expr)?
                }
                _ => true,
            })
        }

        if !skip(expr.then_branch.stmts.last_mut(), self)? {
            replace_block(&mut expr.then_branch, |b| self.next_expr(expr_block(b)));
        }

        match expr.else_branch.as_mut().map(|(_, expr)| &mut **expr) {
            Some(Expr::Block(expr)) => {
                if !skip(expr.block.stmts.last_mut(), self)? {
                    replace_block(&mut expr.block, |b| self.next_expr(expr_block(b)));
                }
                Ok(())
            }
            Some(Expr::If(expr)) => self.visit_last_expr_if(expr),

            // TODO: https://docs.rs/proc-macro2/0.4/proc_macro2/struct.Span.html#method.join
            // `expr.span().join(expr.then_branch.span()).unwrap_or_else(|| expr.span())``
            None => Err(error!(expr.if_token, "`if` expression missing an else clause")),

            Some(_) => unreachable!("wrong_if"),
        }
    }

    fn visit_last_expr_loop(&mut self, expr: &mut ExprLoop) -> Result<()> {
        LoopVisitor::new(expr, self).visit_block_mut(&mut expr.body);
        Ok(())
    }
}

// =================================================================================================
// LoopVisitor

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
    fn visit_expr_mut(&mut self, node: &mut Expr) {
        if node.any_empty_attr(NEVER) {
            return;
        }

        let tmp = self.nested;
        match node {
            // Stop at closure bounds
            Expr::Closure(_) => return,

            // Other loop bounds
            Expr::Loop(_) | Expr::ForLoop(_) | Expr::While(_) => {
                if self.label.is_none() {
                    return;
                }
                self.nested = true;
            }

            // Desugar `break <expr>` into `break Enum::VariantN(<expr>)`.
            Expr::Break(ExprBreak { label, expr, .. }) => {
                if (!self.nested && label.is_none()) || self.compare_labels(label.as_ref()) {
                    self.cx.replace_boxed_expr(expr);
                }
            }

            _ => {}
        }

        visit_mut::visit_expr_mut(self, node);
        self.nested = tmp;
    }

    fn visit_item_mut(&mut self, _: &mut Item) {
        // Do not recurse into nested items.
    }
}
