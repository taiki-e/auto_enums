use syn::{
    visit_mut::{self, VisitMut},
    *,
};

use crate::utils::{Result, *};

use super::*;

const REC_ATTR: &str = "rec";

pub(super) const NEVER_ATTR: &str = "never";

pub(super) const EMPTY_ATTRS: &[&str] = &[NEVER_ATTR, REC_ATTR];

struct Params<'a> {
    marker: &'a Marker,
    rec: bool,
}

impl<'a> From<&'a super::Params> for Params<'a> {
    fn from(params: &'a super::Params) -> Self {
        Self {
            marker: params.marker(),
            rec: false,
        }
    }
}

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

fn last_expr_mut<T, U, F, OP>(
    expr: &mut Expr,
    state: &mut U,
    success: T,
    mut filter: F,
    op: OP,
) -> T
where
    F: FnMut(&Expr, &mut U) -> bool,
    OP: FnOnce(&mut Expr, &mut U) -> T,
{
    if !filter(expr, state) {
        return success;
    }

    match expr {
        Expr::Block(ExprBlock { block, .. }) | Expr::Unsafe(ExprUnsafe { block, .. }) => {
            match block.stmts.last_mut() {
                Some(Stmt::Expr(expr)) => return last_expr_mut(expr, state, success, filter, op),
                Some(Stmt::Semi(expr, _)) => {
                    if !filter(expr, state) {
                        return success;
                    }
                }
                Some(_) => return success,
                None => {}
            }
        }
        _ => {}
    }

    op(expr, state)
}

fn is_unreachable(expr: &Expr, builder: &Builder, params: &Params<'_>) -> bool {
    const UNREACHABLE_MACROS: &[&str] = &["unreachable", "panic"];

    last_expr(
        expr,
        true,
        |expr| !expr.any_empty_attr(NEVER_ATTR) && !expr.any_attr(NAME),
        |expr| match expr {
            Expr::Break(_) | Expr::Continue(_) | Expr::Return(_) => true,

            // `unreachable!`, `panic!` or Expression level marker (`marker!` macro).
            Expr::Macro(ExprMacro { mac, .. }) => {
                UNREACHABLE_MACROS.iter().any(|i| mac.path.is_ident(i))
                    || params.marker.marker_macro(mac)
            }

            // FIXME: This may not be necessary.
            // Assigned.
            Expr::Call(ExprCall { args, func, .. }) if args.len() == 1 => match &**func {
                Expr::Path(ExprPath {
                    path, qself: None, ..
                }) => {
                    path.leading_colon.is_none()
                        && path.segments.len() == 2
                        && path.segments[0].arguments.is_empty()
                        && path.segments[1].arguments.is_empty()
                        && path.segments[0].ident == builder.ident()
                }
                _ => false,
            },

            Expr::Match(ExprMatch { arms, .. }) => arms.iter().all(|arm| {
                arm.any_empty_attr(NEVER_ATTR) || is_unreachable(&*arm.body, builder, params)
            }),

            // `Err(expr)?` or `None?`.
            Expr::Try(ExprTry { expr, .. }) => match &**expr {
                Expr::Path(ExprPath {
                    path, qself: None, ..
                }) => path.is_ident("None"),
                Expr::Call(ExprCall { args, func, .. }) if args.len() == 1 => match &**func {
                    Expr::Path(ExprPath {
                        path, qself: None, ..
                    }) => path.is_ident("Err"),
                    _ => false,
                },
                _ => false,
            },

            _ => false,
        },
    )
}

/// Note that do not use this after `params.*visitor`.
pub(super) fn child_expr(
    expr: &mut Expr,
    builder: &mut Builder,
    params: &super::Params,
) -> Result<()> {
    fn _child_expr(expr: &mut Expr, builder: &mut Builder, params: &mut Params<'_>) -> Result<()> {
        last_expr_mut(
            expr,
            &mut (builder, params),
            Ok(()),
            |expr, (builder, params)| {
                if expr.any_empty_attr(REC_ATTR) {
                    params.rec = true;
                }
                !is_unreachable(expr, builder, params)
            },
            |expr, (builder, params)| match expr {
                Expr::Match(expr) => expr_match(expr, builder, params),
                Expr::If(expr) => expr_if(expr, builder, params),
                Expr::Loop(expr) => expr_loop(expr, builder, params),

                // Search recursively
                Expr::MethodCall(ExprMethodCall { receiver: expr, .. })
                | Expr::Paren(ExprParen { expr, .. })
                | Expr::Type(ExprType { expr, .. }) => _child_expr(&mut **expr, builder, params),

                _ => Ok(()),
            },
        )
    }

    _child_expr(expr, builder, &mut Params::from(params))
}

fn rec_attr(expr: &mut Expr, builder: &mut Builder, params: &Params<'_>) -> Result<bool> {
    last_expr_mut(
        expr,
        builder,
        Ok(true),
        |expr, builder| !is_unreachable(expr, builder, params),
        |expr, builder| match expr {
            Expr::Match(expr) => expr_match(expr, builder, params).map(|_| true),
            Expr::If(expr) => expr_if(expr, builder, params).map(|_| true),
            Expr::Loop(expr) => expr_loop(expr, builder, params).map(|_| true),
            _ => Ok(false),
        },
    )
}

fn expr_match(expr: &mut ExprMatch, builder: &mut Builder, params: &Params<'_>) -> Result<()> {
    fn skip(arm: &mut Arm, builder: &mut Builder, params: &Params<'_>) -> Result<bool> {
        Ok(arm.any_empty_attr(NEVER_ATTR)
            || is_unreachable(&*arm.body, &builder, params)
            || ((arm.any_empty_attr(REC_ATTR) || params.rec)
                && rec_attr(&mut *arm.body, builder, params)?))
    }

    expr.arms.iter_mut().try_for_each(|arm| {
        if !skip(arm, builder, params)? {
            arm.comma = Some(default());
            replace_expr(&mut *arm.body, |x| builder.next_expr(x));
        }

        Ok(())
    })
}

fn expr_if(expr: &mut ExprIf, builder: &mut Builder, params: &Params<'_>) -> Result<()> {
    fn skip(last: Option<&mut Stmt>, builder: &mut Builder, params: &Params<'_>) -> Result<bool> {
        Ok(match &last {
            Some(Stmt::Expr(expr)) | Some(Stmt::Semi(expr, _)) => {
                is_unreachable(expr, &builder, params)
            }
            _ => true,
        } || match last {
            Some(Stmt::Expr(expr)) => {
                (expr.any_empty_attr(REC_ATTR) || params.rec) && rec_attr(expr, builder, params)?
            }
            _ => true,
        })
    }

    if !skip(expr.then_branch.stmts.last_mut(), builder, params)? {
        replace_block(&mut expr.then_branch, |b| builder.next_expr(expr_block(b)));
    }

    match expr.else_branch.as_mut().map(|(_, expr)| &mut **expr) {
        Some(Expr::Block(expr)) => {
            if !skip(expr.block.stmts.last_mut(), builder, params)? {
                replace_block(&mut expr.block, |b| builder.next_expr(expr_block(b)));
            }

            Ok(())
        }
        Some(Expr::If(expr)) => expr_if(expr, builder, params),
        Some(_) => Err(invalid_expr("after of `else` required `{` or `if`"))?,
        None => Err(invalid_expr("`if` expression missing an else clause"))?,
    }
}

fn expr_loop(expr: &mut ExprLoop, builder: &mut Builder, params: &Params<'_>) -> Result<()> {
    LoopVisitor::new(params.marker, &expr, builder).visit_block_mut(&mut expr.body);

    Ok(())
}

struct LoopVisitor<'a> {
    marker: &'a Marker,
    builder: &'a mut Builder,
    depth: usize,
    label: Option<Lifetime>,
}

impl<'a> LoopVisitor<'a> {
    fn new(marker: &'a Marker, expr: &ExprLoop, builder: &'a mut Builder) -> Self {
        Self {
            marker,
            builder,
            depth: 0,
            label: expr.label.as_ref().map(|l| l.name.clone()),
        }
    }

    fn label_eq(&self, other: Option<&Lifetime>) -> bool {
        match (&self.label, other) {
            (None, None) => true,
            (Some(x), Some(y)) => x == y,
            _ => false,
        }
    }
}

impl VisitMut for LoopVisitor<'_> {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        if expr.any_empty_attr(NEVER_ATTR) {
            return;
        }

        let tmp = self.depth;
        match expr {
            // Stop at closure bounds
            Expr::Closure(_) => return,

            // Other loop bounds
            Expr::Loop(_) | Expr::ForLoop(_) | Expr::While(_) => {
                if self.label.is_none() {
                    return;
                }
                self.depth += 1;
            }

            // `break` in loop
            Expr::Break(ExprBreak { label, expr, .. }) => {
                if (self.depth == 0 && label.is_none()) || self.label_eq(label.as_ref()) {
                    expr.replace_boxed_expr(|expr| match expr {
                        Expr::Macro(expr) => {
                            if self.marker.marker_macro(&expr.mac) {
                                Expr::Macro(expr)
                            } else {
                                self.builder.next_expr(Expr::Macro(expr))
                            }
                        }
                        expr => self.builder.next_expr(expr),
                    });
                }
            }

            _ => {}
        }

        visit_mut::visit_expr_mut(self, expr);
        self.depth = tmp;
    }

    // Stop at item bounds
    fn visit_item_mut(&mut self, _item: &mut Item) {}
}
