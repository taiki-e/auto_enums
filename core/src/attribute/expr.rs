use syn::{
    visit_mut::{self, VisitMut},
    *,
};

use crate::utils::{Result, *};

use super::{builder::Builder, *};

/// The annotation for recursively parsing.
const REC: &str = "rec";
/// The annotation for skipping branch.
pub(super) const NEVER: &str = "never";
/// The annotations used by `#[auto_enum]`.
pub(super) const EMPTY_ATTRS: &[&str] = &[NEVER, REC];

// =============================================================================
// Params

struct Params<'a> {
    builder: &'a mut Builder,
    marker: &'a Marker,
    rec: bool,
}

impl<'a> From<&'a mut super::Params> for Params<'a> {
    fn from(params: &'a mut super::Params) -> Self {
        let (builder, marker) = params.double();
        Self {
            builder,
            marker,
            rec: false,
        }
    }
}

// =============================================================================
// Functions and trait for visiting last expression

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

fn is_unreachable(expr: &Expr, params: &Params<'_>) -> bool {
    const UNREACHABLE_MACROS: &[&str] = &["unreachable", "panic"];

    last_expr(
        expr,
        true,
        |expr| !expr.any_empty_attr(NEVER) && !expr.any_attr(NAME),
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
                        && path.segments[0].ident == params.builder.ident()
                }
                _ => false,
            },

            Expr::Match(ExprMatch { arms, .. }) => arms
                .iter()
                .all(|arm| arm.any_empty_attr(NEVER) || is_unreachable(&*arm.body, params)),

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
pub(super) fn child_expr(expr: &mut Expr, params: &mut super::Params) -> Result<()> {
    impl VisitLast<()> for Expr {
        fn visit_last(&mut self, params: &mut Params<'_>) -> Result<()> {
            last_expr_mut(
                self,
                params,
                Ok(()),
                |expr, params| {
                    if expr.any_empty_attr(REC) {
                        params.rec = true;
                    }
                    !is_unreachable(expr, params)
                },
                |expr, params| match expr {
                    Expr::Match(expr) => expr.visit_last(params),
                    Expr::If(expr) => expr.visit_last(params),
                    Expr::Loop(expr) => expr.visit_last(params),

                    // Search recursively
                    Expr::MethodCall(ExprMethodCall { receiver: expr, .. })
                    | Expr::Paren(ExprParen { expr, .. })
                    | Expr::Type(ExprType { expr, .. }) => expr.visit_last(params),

                    _ => Ok(()),
                },
            )
        }
    }

    expr.visit_last(&mut Params::from(params))
}

trait VisitLast<T> {
    fn visit_last(&mut self, params: &mut Params<'_>) -> Result<T>;
}

impl VisitLast<bool> for Expr {
    fn visit_last(&mut self, params: &mut Params<'_>) -> Result<bool> {
        last_expr_mut(
            self,
            params,
            Ok(true),
            |expr, params| !is_unreachable(expr, params),
            |expr, params| match expr {
                Expr::Match(expr) => expr.visit_last(params).map(|_| true),
                Expr::If(expr) => expr.visit_last(params).map(|_| true),
                Expr::Loop(expr) => expr.visit_last(params).map(|_| true),
                _ => Ok(false),
            },
        )
    }
}

impl VisitLast<()> for ExprMatch {
    fn visit_last(&mut self, params: &mut Params<'_>) -> Result<()> {
        fn skip(arm: &mut Arm, params: &mut Params<'_>) -> Result<bool> {
            Ok(arm.any_empty_attr(NEVER)
                || is_unreachable(&*arm.body, params)
                || ((arm.any_empty_attr(REC) || params.rec) && arm.body.visit_last(params)?))
        }

        self.arms.iter_mut().try_for_each(|arm| {
            if !skip(arm, params)? {
                arm.comma = Some(default());
                replace_expr(&mut *arm.body, |x| params.builder.next_expr(x));
            }

            Ok(())
        })
    }
}

impl VisitLast<()> for ExprIf {
    fn visit_last(&mut self, params: &mut Params<'_>) -> Result<()> {
        fn skip(last: Option<&mut Stmt>, params: &mut Params<'_>) -> Result<bool> {
            Ok(match &last {
                Some(Stmt::Expr(expr)) | Some(Stmt::Semi(expr, _)) => is_unreachable(expr, params),
                _ => true,
            } || match last {
                Some(Stmt::Expr(expr)) => {
                    (expr.any_empty_attr(REC) || params.rec) && expr.visit_last(params)?
                }
                _ => true,
            })
        }

        if !skip(self.then_branch.stmts.last_mut(), params)? {
            replace_block(&mut self.then_branch, |b| {
                params.builder.next_expr(expr_block(b))
            });
        }

        match self.else_branch.as_mut().map(|(_, expr)| &mut **expr) {
            Some(Expr::Block(expr)) => {
                if !skip(expr.block.stmts.last_mut(), params)? {
                    replace_block(&mut expr.block, |b| params.builder.next_expr(expr_block(b)));
                }

                Ok(())
            }
            Some(Expr::If(expr)) => expr.visit_last(params),
            Some(_) => Err(invalid_expr("after of `else` required `{` or `if`"))?,
            None => Err(invalid_expr("`if` expression missing an else clause"))?,
        }
    }
}

impl VisitLast<()> for ExprLoop {
    fn visit_last(&mut self, params: &mut Params<'_>) -> Result<()> {
        LoopVisitor::new(params, &self).visit_block_mut(&mut self.body);

        Ok(())
    }
}

struct LoopVisitor<'a> {
    builder: &'a mut Builder,
    marker: &'a Marker,
    depth: usize,
    label: Option<Lifetime>,
}

impl<'a> LoopVisitor<'a> {
    fn new(
        Params {
            marker, builder, ..
        }: &'a mut Params<'_>,
        expr: &ExprLoop,
    ) -> Self {
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
        if expr.any_empty_attr(NEVER) {
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
