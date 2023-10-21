// SPDX-License-Identifier: Apache-2.0 OR MIT

use syn::{
    visit_mut::{self, VisitMut},
    Arm, Block, Expr, ExprBlock, ExprBreak, ExprCall, ExprIf, ExprLoop, ExprMacro, ExprMatch,
    ExprMethodCall, ExprParen, ExprPath, ExprTry, ExprUnsafe, Item, Label, Lifetime, LocalInit,
    Macro, Stmt, StmtMacro, Token,
};

use super::{visitor, Context, NAME, NESTED, NEVER};
use crate::utils::{expr_block, path_eq, replace_block, replace_expr, Attrs};

/// Visits last expression.
///
/// Note that do not use this after `cx.visitor()`.
pub(super) fn child_expr(cx: &mut Context, expr: &mut Expr) {
    if !cx.visit_last() || is_unreachable(cx, expr) {
        return;
    }

    match expr {
        Expr::Block(ExprBlock { block, .. }) | Expr::Unsafe(ExprUnsafe { block, .. }) => {
            if let Some(Stmt::Expr(expr, None)) = block.stmts.last_mut() {
                child_expr(cx, expr);
            }
        }

        Expr::Match(expr) => visit_last_expr_match(cx, expr),
        Expr::If(expr) => visit_last_expr_if(cx, expr),
        Expr::Loop(expr) => visit_last_expr_loop(cx, expr),

        // Search recursively
        Expr::MethodCall(ExprMethodCall { receiver: expr, .. })
        | Expr::Paren(ExprParen { expr, .. }) => child_expr(cx, expr),

        _ => {}
    }
}

pub(super) fn is_unreachable(cx: &Context, expr: &Expr) -> bool {
    if expr.any_empty_attr(NEVER) || expr.any_attr(NAME) {
        return true;
    }

    match expr {
        Expr::Block(ExprBlock { block, .. }) | Expr::Unsafe(ExprUnsafe { block, .. }) => {
            is_unreachable_stmt(cx, block.stmts.last())
        }

        Expr::Break(_) | Expr::Continue(_) | Expr::Return(_) => true,

        Expr::Macro(ExprMacro { mac, .. }) => is_unreachable_macro(cx, mac),

        Expr::Match(ExprMatch { arms, .. }) => {
            arms.iter().all(|arm| arm.any_empty_attr(NEVER) || is_unreachable(cx, &arm.body))
        }

        // `Err(expr)?` or `None?`.
        Expr::Try(ExprTry { expr, .. }) => match &**expr {
            Expr::Path(ExprPath { path, qself: None, .. }) => {
                path_eq(path, &["std", "core"], &["option", "Option", "None"])
            }
            Expr::Call(ExprCall { args, func, .. }) if args.len() == 1 => match &**func {
                Expr::Path(ExprPath { path, qself: None, .. }) => {
                    path_eq(path, &["std", "core"], &["result", "Result", "Err"])
                }
                _ => false,
            },
            _ => false,
        },

        // Search recursively
        Expr::MethodCall(ExprMethodCall { receiver: expr, .. })
        | Expr::Paren(ExprParen { expr, .. }) => is_unreachable(cx, expr),

        _ => false,
    }
}

fn is_unreachable_macro(cx: &Context, mac: &Macro) -> bool {
    const UNREACHABLE_MACROS: &[&str] = &["unreachable", "panic"];

    // `unreachable!`, `panic!` or an expression level marker (`marker!` macro).
    UNREACHABLE_MACROS.iter().any(|i| path_eq(&mac.path, &["std", "core"], &[i]))
        || cx.is_marker_macro(mac)
}

fn is_unreachable_stmt(cx: &Context, stmt: Option<&Stmt>) -> bool {
    match stmt {
        Some(Stmt::Expr(expr, _)) => is_unreachable(cx, expr),
        Some(Stmt::Local(local)) => {
            local.init.as_ref().map_or(false, |LocalInit { expr, .. }| is_unreachable(cx, expr))
        }
        Some(Stmt::Item(_)) => true,
        Some(Stmt::Macro(StmtMacro { mac, .. })) => is_unreachable_macro(cx, mac),
        None => false,
    }
}

fn visit_last_expr_match(cx: &mut Context, expr: &mut ExprMatch) {
    fn skip(cx: &Context, arm: &mut Arm) -> bool {
        arm.any_empty_attr(NEVER)
            || arm.any_empty_attr(NESTED)
            || is_unreachable(cx, &arm.body)
            || visitor::find_nested(arm)
    }

    for arm in &mut expr.arms {
        if !skip(cx, arm) {
            arm.comma = Some(<Token![,]>::default());
            replace_expr(&mut arm.body, |x| cx.next_expr(x));
        }
    }
}

fn visit_last_expr_if(cx: &mut Context, expr: &mut ExprIf) {
    fn skip(cx: &Context, block: &mut Block) -> bool {
        match block.stmts.last_mut() {
            Some(Stmt::Expr(expr, None)) => {
                expr.any_empty_attr(NESTED)
                    || is_unreachable(cx, expr)
                    || visitor::find_nested(block)
            }
            _ => is_unreachable_stmt(cx, block.stmts.last()),
        }
    }

    if !skip(cx, &mut expr.then_branch) {
        replace_block(&mut expr.then_branch, |b| cx.next_expr(expr_block(b)));
    }

    match expr.else_branch.as_mut().map(|(_, expr)| &mut **expr) {
        Some(Expr::Block(expr)) => {
            if !skip(cx, &mut expr.block) {
                replace_block(&mut expr.block, |b| cx.next_expr(expr_block(b)));
            }
        }
        Some(Expr::If(expr)) => visit_last_expr_if(cx, expr),

        // TODO: https://docs.rs/proc-macro2/1/proc_macro2/struct.Span.html#method.join
        // `expr.span().join(expr.then_branch.span()).unwrap_or_else(|| expr.span())``
        None => cx.error(format_err!(expr.if_token, "`if` expression missing an else clause")),

        Some(_) => unreachable!("wrong_if"),
    }
}

fn visit_last_expr_loop(cx: &mut Context, expr: &mut ExprLoop) {
    struct LoopVisitor<'a> {
        cx: &'a mut Context,
        label: Option<&'a Label>,
        nested: bool,
    }

    impl LoopVisitor<'_> {
        fn compare_labels(&self, other: Option<&Lifetime>) -> bool {
            match (self.label, other) {
                (None, None) => true,
                (Some(this), Some(other)) => this.name.ident == other.ident,
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
                // Stop at closure / async block bounds
                Expr::Closure(_) | Expr::Async(_) => return,
                // Other loop bounds
                Expr::Loop(_) | Expr::ForLoop(_) | Expr::While(_) => {
                    if self.label.is_none() {
                        return;
                    }
                    self.nested = true;
                }
                // Desugar `break <expr>` into `break Enum::VariantN(<expr>)`.
                Expr::Break(ExprBreak { label, expr, .. }) => {
                    if !self.nested && label.is_none() || self.compare_labels(label.as_ref()) {
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

    LoopVisitor { cx, label: expr.label.as_ref(), nested: false }.visit_block_mut(&mut expr.body);
}
