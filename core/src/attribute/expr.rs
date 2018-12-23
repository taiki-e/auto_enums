use std::{cell::Cell, mem};

use syn::{
    visit_mut::{self, VisitMut},
    *,
};

use crate::utils::{Result, *};

use super::*;

const REC_ATTR: &str = "rec";

pub(super) const NEVER_ATTR: &str = "never";

pub(super) const EMPTY_ATTRS: &[&str] = &[NEVER_ATTR, REC_ATTR];

#[derive(Debug)]
struct Params<'a> {
    marker_ident: &'a str,
    marker: bool,
    #[cfg(feature = "type_analysis")]
    attr: bool,
    rec: Cell<bool>,
}

impl<'a> From<&'a super::Params> for Params<'a> {
    fn from(params: &'a super::Params) -> Self {
        Params {
            marker_ident: params.marker_ident(),
            marker: params.count() >= 2,
            #[cfg(feature = "type_analysis")]
            attr: params.attr(),
            rec: Cell::new(false),
        }
    }
}

fn last_expr<T, F, OP>(expr: &Expr, success: T, mut filter: F, op: OP) -> T
where
    F: FnMut(&Expr) -> bool,
    OP: FnOnce(&Expr) -> T,
{
    macro_rules! last_stmt {
        ($expr:expr) => {
            match $expr {
                Some(Stmt::Expr(expr)) => return last_expr(expr, success, filter, op),
                Some(Stmt::Semi(expr, _)) => {
                    if !filter(expr) {
                        return success;
                    }
                }
                Some(_) => return success,
                None => {}
            }
        };
    }

    if !filter(expr) {
        return success;
    }

    match expr {
        Expr::Block(e) => last_stmt!(e.block.stmts.last()),
        Expr::Unsafe(e) => last_stmt!(e.block.stmts.last()),
        _ => {}
    }

    op(expr)
}

fn last_expr_mut<T, F, OP>(expr: &mut Expr, success: T, mut filter: F, op: OP) -> T
where
    F: FnMut(&Expr) -> bool,
    OP: FnOnce(&mut Expr) -> T,
{
    macro_rules! last_stmt {
        ($expr:expr) => {
            match $expr {
                Some(Stmt::Expr(expr)) => return last_expr_mut(expr, success, filter, op),
                Some(Stmt::Semi(expr, _)) => {
                    if !filter(expr) {
                        return success;
                    }
                }
                Some(_) => return success,
                None => {}
            }
        };
    }

    if !filter(expr) {
        return success;
    }

    match expr {
        Expr::Block(expr) => last_stmt!(expr.block.stmts.last_mut()),
        Expr::Unsafe(expr) => last_stmt!(expr.block.stmts.last_mut()),
        _ => {}
    }

    op(expr)
}

fn is_unreachable(expr: &Expr, params: &Params) -> bool {
    const UNREACHABLE_MACROS: &[&str] = &["unreachable", "panic"];

    last_expr(
        expr,
        true,
        |expr| !expr.any_empty_attr(NEVER_ATTR) && !expr.any_attr(NAME),
        |expr| match expr {
            Expr::Break(_) | Expr::Continue(_) | Expr::Return(_) => true,
            Expr::Macro(expr) => {
                UNREACHABLE_MACROS.iter().any(|i| expr.mac.path.is_ident(i))
                    || expr.mac.path.is_ident(params.marker_ident)
            }
            Expr::Match(expr) => expr
                .arms
                .iter()
                .all(|arm| arm.any_empty_attr(NEVER_ATTR) || is_unreachable(&*arm.body, params)),
            Expr::Try(expr) => match &*expr.expr {
                Expr::Path(expr) => expr.path.is_ident("None") && expr.qself.is_none(),
                Expr::Call(expr) if expr.args.len() == 1 => match &*expr.func {
                    Expr::Path(expr) => expr.path.is_ident("Err") && expr.qself.is_none(),
                    _ => false,
                },
                _ => false,
            },
            _ => false,
        },
    )
}

pub(super) fn child_expr(
    expr: &mut Expr,
    builder: &mut Builder,
    params: &super::Params,
) -> Result<()> {
    fn _child_expr(expr: &mut Expr, builder: &mut Builder, params: &Params) -> Result<()> {
        const ERR: &str =
            "for expressions other than `match` or `if`, you need to specify marker macros";

        last_expr_mut(
            expr,
            Ok(()),
            |expr| {
                if expr.any_empty_attr(REC_ATTR) {
                    params.rec.set(true);
                }
                !is_unreachable(expr, params)
            },
            |expr| match expr {
                Expr::Match(expr) => expr_match(expr, builder, params),
                Expr::If(expr) => expr_if(expr, builder, params),
                Expr::Loop(expr) => expr_loop(expr, builder, params),
                Expr::MethodCall(expr) => _child_expr(&mut *expr.receiver, builder, params),
                _ if params.marker => Ok(()),
                #[cfg(feature = "type_analysis")]
                _ if params.attr => Ok(()),
                _ => Err(unsupported_expr(ERR)),
            },
        )
    }

    _child_expr(expr, builder, &Params::from(params))
}

fn rec_attr(expr: &mut Expr, builder: &mut Builder, params: &Params) -> Result<bool> {
    last_expr_mut(
        expr,
        Ok(true),
        |expr| !is_unreachable(expr, params),
        |expr| match expr {
            Expr::Match(expr) => expr_match(expr, builder, params).map(|_| true),
            Expr::If(expr) => expr_if(expr, builder, params).map(|_| true),
            Expr::Loop(expr) => expr_loop(expr, builder, params).map(|_| true),
            _ => Ok(false),
        },
    )
}

fn expr_continue() -> Expr {
    // probably the lowest cost expression.
    Expr::Continue(ExprContinue {
        attrs: Vec::with_capacity(0),
        continue_token: default(),
        label: None,
    })
}

fn expr_match(expr: &mut ExprMatch, builder: &mut Builder, params: &Params) -> Result<()> {
    fn skip(arm: &mut Arm, builder: &mut Builder, params: &Params) -> Result<bool> {
        Ok(arm.any_empty_attr(NEVER_ATTR)
            || is_unreachable(&*arm.body, params)
            || ((arm.any_empty_attr(REC_ATTR) || params.rec.get())
                && rec_attr(&mut *arm.body, builder, params)?))
    }

    expr.arms.iter_mut().try_for_each(|arm| {
        if !skip(arm, builder, params)? {
            arm.comma = Some(default());
            *arm.body = builder.next_expr(
                Vec::with_capacity(0),
                mem::replace(&mut *arm.body, expr_continue()),
            );
        }

        Ok(())
    })
}

fn expr_if(expr: &mut ExprIf, builder: &mut Builder, params: &Params) -> Result<()> {
    fn skip(last: Option<&mut Stmt>, builder: &mut Builder, params: &Params) -> Result<bool> {
        Ok(match &last {
            Some(Stmt::Expr(expr)) | Some(Stmt::Semi(expr, _)) => is_unreachable(expr, params),
            _ => true,
        } || match last {
            Some(Stmt::Expr(expr)) => {
                (expr.any_empty_attr(REC_ATTR) || params.rec.get())
                    && rec_attr(expr, builder, params)?
            }
            _ => true,
        })
    }

    fn replace_block(branch: &mut Block, builder: &mut Builder) {
        *branch = block(vec![Stmt::Expr(builder.next_expr(
            Vec::with_capacity(0),
            expr_block(mem::replace(branch, block(Vec::with_capacity(0)))),
        ))]);
    }

    if !skip(expr.then_branch.stmts.last_mut(), builder, params)? {
        replace_block(&mut expr.then_branch, builder);
    }

    match expr.else_branch.as_mut().map(|(_, expr)| &mut **expr) {
        Some(Expr::Block(expr)) => {
            if !skip(expr.block.stmts.last_mut(), builder, params)? {
                replace_block(&mut expr.block, builder);
            }

            Ok(())
        }
        Some(Expr::If(expr)) => expr_if(expr, builder, params),
        Some(_) => Err(invalid_expr("after of `else` required `{` or `if`"))?,
        None => Err(invalid_expr("`if` expression missing an else clause"))?,
    }
}

fn expr_loop(expr: &mut ExprLoop, builder: &mut Builder, params: &Params) -> Result<()> {
    LoopVisitor::new(params.marker_ident, &expr, builder).visit_block_mut(&mut expr.body);

    Ok(())
}

struct LoopVisitor<'a> {
    marker: &'a str,
    label: Option<Lifetime>,
    builder: &'a mut Builder,
    depth: usize,
}

impl<'a> LoopVisitor<'a> {
    fn new(marker: &'a str, expr: &ExprLoop, builder: &'a mut Builder) -> Self {
        LoopVisitor {
            marker,
            label: expr.label.as_ref().map(|l| l.name.clone()),
            builder,
            depth: 0,
        }
    }

    fn label_eq(&self, other: Option<&Lifetime>) -> bool {
        match (&self.label, other) {
            (None, None) => true,
            (Some(x), Some(y)) => x == y,
            _ => false,
        }
    }

    fn loop_bounds<F: FnOnce(&mut Self)>(&mut self, f: F) {
        if self.label.is_some() {
            self.depth += 1;
            f(self);
            self.depth -= 1;
        }
    }
}

impl<'a> VisitMut for LoopVisitor<'a> {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        if !expr.any_empty_attr(NEVER_ATTR) {
            match expr {
                // Stop at closure bounds
                Expr::Closure(_) => {}

                // Other loop bounds
                Expr::Loop(expr) => {
                    self.loop_bounds(|v| visit_mut::visit_expr_loop_mut(v, expr));
                }
                Expr::ForLoop(expr) => {
                    self.loop_bounds(|v| visit_mut::visit_expr_for_loop_mut(v, expr));
                }
                Expr::While(expr) => {
                    self.loop_bounds(|v| visit_mut::visit_expr_while_mut(v, expr));
                }

                Expr::Break(br) => {
                    if (self.depth == 0 && br.label.is_none()) || self.label_eq(br.label.as_ref()) {
                        match br.expr.take().map_or_else(|| Expr::Tuple(unit()), |e| *e) {
                            Expr::Macro(expr) => {
                                if expr.mac.path.is_ident(self.marker) {
                                    br.expr = Some(Box::new(Expr::Macro(expr)));
                                } else {
                                    br.expr = Some(Box::new(
                                        self.builder
                                            .next_expr(Vec::with_capacity(0), Expr::Macro(expr)),
                                    ));
                                }
                            }
                            expr => {
                                br.expr = Some(Box::new(
                                    self.builder.next_expr(Vec::with_capacity(0), expr),
                                ));
                            }
                        }
                    }

                    visit_mut::visit_expr_break_mut(self, br);
                }

                expr => visit_mut::visit_expr_mut(self, expr),
            }
        }
    }

    // Stop at item bounds
    fn visit_item_mut(&mut self, _item: &mut Item) {}
}
