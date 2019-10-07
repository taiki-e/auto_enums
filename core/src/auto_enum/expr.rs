use syn::{
    visit_mut::{self, VisitMut},
    *,
};

use crate::utils::{expr_block, replace_block, replace_expr, Attrs};

use super::{Context, NAME, NESTED, NEVER};

/// Visits last expression.
///
/// Note that do not use this after `cx.visitor()`.
pub(super) fn child_expr(expr: &mut Expr, cx: &mut Context) -> Result<()> {
    if !cx.visit_last() || is_unreachable(cx, expr) {
        return Ok(());
    }

    match expr {
        Expr::Block(ExprBlock { block, .. }) | Expr::Unsafe(ExprUnsafe { block, .. }) => {
            if let Some(Stmt::Expr(expr)) = block.stmts.last_mut() {
                return child_expr(expr, cx);
            }
        }
        _ => {}
    }

    match expr {
        Expr::Match(expr) => visit_last_expr_match(cx, expr),
        Expr::If(expr) => visit_last_expr_if(cx, expr),
        Expr::Loop(expr) => visit_last_expr_loop(cx, expr),

        // Search recursively
        Expr::MethodCall(ExprMethodCall { receiver: expr, .. })
        | Expr::Paren(ExprParen { expr, .. })
        | Expr::Type(ExprType { expr, .. }) => child_expr(expr, cx),

        _ => Ok(()),
    }
}

pub(super) fn is_unreachable(cx: &Context, expr: &Expr) -> bool {
    const UNREACHABLE_MACROS: &[&str] = &["unreachable", "panic"];

    if expr.any_empty_attr(NEVER) || expr.any_attr(NAME) {
        return true;
    }

    match expr {
        Expr::Block(ExprBlock { block, .. }) | Expr::Unsafe(ExprUnsafe { block, .. }) => {
            is_unreachable_stmt(cx, block.stmts.last())
        }

        Expr::Break(_) | Expr::Continue(_) | Expr::Return(_) => true,

        // `unreachable!`, `panic!` or an expression level marker (`marker!` macro).
        Expr::Macro(ExprMacro { mac, .. }) => {
            UNREACHABLE_MACROS.iter().any(|i| mac.path.is_ident(i)) || cx.is_marker_macro(mac)
        }

        Expr::Match(ExprMatch { arms, .. }) => {
            arms.iter().all(|arm| arm.any_empty_attr(NEVER) || is_unreachable(cx, &*arm.body))
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

        // Search recursively
        Expr::MethodCall(ExprMethodCall { receiver: expr, .. })
        | Expr::Paren(ExprParen { expr, .. })
        | Expr::Type(ExprType { expr, .. }) => is_unreachable(cx, expr),

        _ => false,
    }
}

fn is_unreachable_stmt(cx: &Context, stmt: Option<&Stmt>) -> bool {
    match stmt {
        Some(Stmt::Expr(expr)) | Some(Stmt::Semi(expr, _)) => is_unreachable(cx, expr),
        Some(Stmt::Local(local)) => {
            local.init.as_ref().map_or(false, |(_, expr)| is_unreachable(cx, expr))
        }
        Some(Stmt::Item(_)) => true,
        None => false,
    }
}

fn visit_last_expr_match(cx: &mut Context, expr: &mut ExprMatch) -> Result<()> {
    fn skip(arm: &mut Arm, cx: &mut Context) -> bool {
        arm.any_empty_attr(NEVER)
            || arm.any_empty_attr(NESTED)
            || is_unreachable(cx, &*arm.body)
            || cx.find_nested(arm)
    }

    expr.arms.iter_mut().try_for_each(|arm| {
        if !skip(arm, cx) {
            arm.comma = Some(token::Comma::default());
            replace_expr(&mut *arm.body, |x| cx.next_expr(x));
        }
        Ok(())
    })
}

fn visit_last_expr_if(cx: &mut Context, expr: &mut ExprIf) -> Result<()> {
    fn skip(block: &mut Block, cx: &mut Context) -> bool {
        match block.stmts.last_mut() {
            Some(Stmt::Expr(expr)) => {
                expr.any_empty_attr(NESTED) || is_unreachable(cx, expr) || cx.find_nested(block)
            }
            _ => is_unreachable_stmt(cx, block.stmts.last()),
        }
    }

    if !skip(&mut expr.then_branch, cx) {
        replace_block(&mut expr.then_branch, |b| cx.next_expr(expr_block(b)));
    }

    match expr.else_branch.as_mut().map(|(_, expr)| &mut **expr) {
        Some(Expr::Block(expr)) => {
            if !skip(&mut expr.block, cx) {
                replace_block(&mut expr.block, |b| cx.next_expr(expr_block(b)));
            }
            Ok(())
        }
        Some(Expr::If(expr)) => visit_last_expr_if(cx, expr),

        // TODO: https://docs.rs/proc-macro2/0.4/proc_macro2/struct.Span.html#method.join
        // `expr.span().join(expr.then_branch.span()).unwrap_or_else(|| expr.span())``
        None => Err(error!(expr.if_token, "`if` expression missing an else clause")),

        Some(_) => unreachable!("wrong_if"),
    }
}

fn visit_last_expr_loop(cx: &mut Context, expr: &mut ExprLoop) -> Result<()> {
    LoopVisitor::new(expr.label.as_ref(), cx).visit_block_mut(&mut expr.body);
    Ok(())
}

// =================================================================================================
// LoopVisitor

struct LoopVisitor<'a> {
    cx: &'a mut Context,
    label: Option<&'a Label>,
    nested: bool,
}

impl<'a> LoopVisitor<'a> {
    fn new(label: Option<&'a Label>, cx: &'a mut Context) -> Self {
        Self { cx, label, nested: false }
    }

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
