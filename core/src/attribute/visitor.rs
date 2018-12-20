use proc_macro2::{Group, TokenStream as TokenStream2};
use syn::{
    fold::{self, Fold},
    visit::{self, Visit},
    *,
};

use crate::utils::Result;

use super::*;

pub(super) const DEFAULT_MARKER: &str = "marker";

pub(super) struct MarkerCounter<'a> {
    marker: &'a str,
    count: &'a mut usize,
    attr: &'a mut bool,
    unique_marker: bool,
}

impl<'a> MarkerCounter<'a> {
    pub(super) fn new(marker: &'a str, count: &'a mut usize, attr: &'a mut bool) -> Self {
        MarkerCounter {
            marker,
            count,
            attr,
            unique_marker: marker != DEFAULT_MARKER,
        }
    }
}

impl<'a, 'ast> Visit<'ast> for MarkerCounter<'a> {
    fn visit_expr(&mut self, expr: &'ast Expr) {
        if !expr.any_attr(NAME) || self.unique_marker {
            if let Expr::Macro(expr) = expr {
                visit::visit_expr_macro(self, expr);
                if expr.mac.path.is_ident(self.marker) {
                    *self.count += 1;
                }
            } else {
                visit::visit_expr(self, expr);
            }
        }
    }

    fn visit_local(&mut self, local: &'ast Local) {
        if !local.any_attr(NAME) || self.unique_marker {
            visit::visit_local(self, local);
        }
    }

    fn visit_stmt(&mut self, stmt: &'ast Stmt) {
        visit::visit_stmt(self, stmt);
        match stmt {
            Stmt::Expr(expr) | Stmt::Semi(expr, _) if expr.any_attr(NAME) => *self.attr = true,
            Stmt::Local(local) if local.any_attr(NAME) => *self.attr = true,
            _ => {}
        }
    }

    fn visit_item(&mut self, _item: &'ast Item) {}
}

pub(super) struct ReturnCounter<'a> {
    marker: &'a str,
    count: &'a mut usize,
}

impl<'a> ReturnCounter<'a> {
    pub(super) fn new(marker: &'a str, count: &'a mut usize) -> Self {
        ReturnCounter { marker, count }
    }
}

impl<'a, 'ast> Visit<'ast> for ReturnCounter<'a> {
    fn visit_expr(&mut self, expr: &'ast Expr) {
        if !expr.any_empty_attr(NEVER_ATTR) {
            match expr {
                Expr::Return(expr) => {
                    match expr.expr.as_ref().map(|e| &**e) {
                        Some(Expr::Macro(expr)) if expr.mac.path.is_ident(self.marker) => {}
                        _ => *self.count += 1,
                    }
                    visit::visit_expr_return(self, expr);
                }
                Expr::Closure(_) => {}
                expr => visit::visit_expr(self, expr),
            }
        }
    }

    fn visit_item(&mut self, _item: &'ast Item) {}
}

pub(super) struct Replacer<'a> {
    marker: &'a str,
    marker_count: usize,
    return_count: usize,
    builder: &'a mut Builder,
    empty_attrs: &'static [&'static str],
    foreign: bool,
    in_closure: i32,
}

impl<'a> Replacer<'a> {
    pub(super) fn new(
        marker: &'a str,
        marker_count: usize,
        return_count: usize,
        is_closure: bool,
        builder: &'a mut Builder,
    ) -> Self {
        Replacer {
            marker,
            marker_count,
            return_count,
            builder,
            empty_attrs: EMPTY_ATTRS,
            foreign: false,
            in_closure: if is_closure { 2 } else { 1 },
        }
    }

    pub(super) fn dummy(builder: &'a mut Builder) -> Self {
        Replacer::new(DEFAULT_MARKER, 0, 0, false, builder)
    }

    fn find_remove_empty_attrs(&self, attrs: &mut Vec<Attribute>) {
        self.empty_attrs.iter().for_each(|ident| {
            attrs.find_remove_empty_attr(ident);
        });
    }
}

impl<'a> Fold for Replacer<'a> {
    fn fold_expr(&mut self, mut expr: Expr) -> Expr {
        let tmp = self.in_closure;
        if let Expr::Closure(_) = &expr {
            self.in_closure -= 1;
        }

        if self.in_closure > 0 && self.return_count != 0 && !expr.any_empty_attr(NEVER_ATTR) {
            expr = match expr {
                Expr::Return(mut ret) => {
                    match ret.expr.take().map_or_else(|| Expr::Tuple(unit()), |e| *e) {
                        Expr::Macro(expr) => {
                            if expr.mac.path.is_ident(self.marker) {
                                ret.expr = Some(Box::new(Expr::Macro(expr)));
                            } else {
                                self.return_count -= 1;
                                ret.expr = Some(Box::new(
                                    self.builder
                                        .next_expr(Vec::with_capacity(0), Expr::Macro(expr)),
                                ));
                            }
                        }
                        expr => {
                            self.return_count -= 1;
                            ret.expr = Some(Box::new(
                                self.builder.next_expr(Vec::with_capacity(0), expr),
                            ));
                        }
                    }

                    Expr::Return(ret)
                }
                expr => expr,
            };
        }

        if !expr.any_attr(NAME) || (self.foreign && self.marker_count != 0) {
            expr = fold::fold_expr(self, expr);

            if !self.foreign {
                attrs_mut(&mut expr, |attrs| self.find_remove_empty_attrs(attrs));
            }

            if self.marker_count != 0 {
                expr = match expr {
                    Expr::Macro(expr) => {
                        if expr.mac.path.is_ident(self.marker) {
                            let args = syn::parse2(expr.mac.tts).unwrap_or_else(|_| {
                                panic!("`#[{}]` invalid tokens: the arguments of `{}!` macro must be an expression", NAME, self.marker)
                            });

                            self.marker_count -= 1;
                            self.builder.next_expr(expr.attrs, args)
                        } else {
                            Expr::Macro(expr)
                        }
                    }
                    expr => expr,
                };
            }
        } else {
            let tmp = self.foreign;
            self.foreign = true;
            expr = fold::fold_expr(self, expr);
            self.foreign = tmp;
        }

        self.in_closure = tmp;
        expr
    }

    fn fold_arm(&mut self, mut arm: Arm) -> Arm {
        arm = fold::fold_arm(self, arm);

        if !self.foreign {
            self.find_remove_empty_attrs(&mut arm.attrs);
        }

        arm
    }

    fn fold_local(&mut self, mut local: Local) -> Local {
        if !local.any_attr(NAME) || (self.foreign && self.marker_count != 0) {
            fold::fold_local(self, local)
        } else {
            let tmp = self.foreign;
            self.foreign = true;
            local = fold::fold_local(self, local);
            self.foreign = tmp;
            local
        }
    }

    fn fold_item(&mut self, item: Item) -> Item {
        item
    }

    fn fold_stmt(&mut self, stmt: Stmt) -> Stmt {
        fold_stmt(fold::fold_stmt(self, stmt)).unwrap_or_else(|e| panic!("`#[{}]` {}", NAME, e))
    }
}

fn fold_stmt(stmt: Stmt) -> Result<Stmt> {
    fn parse_tts(tts: TokenStream2) -> Result<Params> {
        syn::parse2(tts)
            .map_err(|e| invalid_args!(e))
            .and_then(|group: Group| parse_args(group.stream()))
    }

    match stmt {
        Stmt::Expr(mut expr) => {
            if let Some(attr) = attrs_mut(&mut expr, |attrs| attrs.find_remove_attr(NAME)) {
                expr = parse_tts(attr.tts).and_then(|params| parent_expr(expr, params))?;
            }
            Ok(Stmt::Expr(expr))
        }
        Stmt::Semi(mut expr, semi) => {
            if let Some(attr) = attrs_mut(&mut expr, |attrs| attrs.find_remove_attr(NAME)) {
                expr = parse_tts(attr.tts).and_then(|params| stmt_semi(expr, params))?;
            }
            Ok(Stmt::Semi(expr, semi))
        }
        Stmt::Local(mut local) => {
            if let Some(attr) = local.find_remove_attr(NAME) {
                local = parse_tts(attr.tts).and_then(|params| stmt_let(local, params))?;
            }
            Ok(Stmt::Local(local))
        }
        stmt => Ok(stmt),
    }
}
