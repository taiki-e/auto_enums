use proc_macro2::{Group, TokenStream as TokenStream2};
use syn::{
    fold::{self, Fold},
    visit::{self, Visit},
    *,
};

use crate::utils::Result;

use super::*;

pub(super) const DEFAULT_MARKER: &str = "marker";

pub(super) struct Visitor<'a> {
    marker: &'a str,
    count: &'a mut usize,
    attr: &'a mut bool,
}

impl<'a> Visitor<'a> {
    pub(super) fn new(marker: &'a str, count: &'a mut usize, attr: &'a mut bool) -> Self {
        Visitor {
            marker,
            count,
            attr,
        }
    }

    fn unique_marker(&self) -> bool {
        self.marker != DEFAULT_MARKER
    }
}

impl<'a, 'ast> Visit<'ast> for Visitor<'a> {
    fn visit_expr(&mut self, expr: &'ast Expr) {
        if !expr.any_attr(NAME) || self.unique_marker() {
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
        if !local.any_attr(NAME) || self.unique_marker() {
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

pub(super) struct Replacer<'a> {
    marker: &'a str,
    marker_count: usize,
    builder: &'a mut Builder,
    empty_attrs: &'static [&'static str],
    foreign: bool,
}

impl<'a> Replacer<'a> {
    pub(super) fn new(marker: &'a str, marker_count: usize, builder: &'a mut Builder) -> Self {
        Replacer {
            marker,
            marker_count,
            builder,
            empty_attrs: EMPTY_ATTRS,
            foreign: false,
        }
    }

    pub(super) fn dummy(builder: &'a mut Builder) -> Self {
        Replacer::new(DEFAULT_MARKER, 0, builder)
    }

    fn unique_marker(&self) -> bool {
        self.marker != DEFAULT_MARKER
    }

    fn find_remove_empty_attrs(&self, attrs: &mut Vec<Attribute>) {
        self.empty_attrs.iter().for_each(|ident| {
            attrs.find_remove_empty_attr(ident);
        });
    }
}

impl<'a> Fold for Replacer<'a> {
    fn fold_expr(&mut self, mut expr: Expr) -> Expr {
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
                            self.builder.next_expr_call(expr.attrs, args)
                        } else {
                            Expr::Macro(expr)
                        }
                    }
                    expr => expr,
                };
            }
        } else if self.marker_count != 0 && self.unique_marker() {
            self.foreign = true;
            expr = fold::fold_expr(self, expr);
            self.foreign = false;
        }

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
            local = fold::fold_local(self, local);
        } else if self.unique_marker() && self.marker_count != 0 {
            self.foreign = true;
            local = fold::fold_local(self, local);
            self.foreign = false;
        }

        local
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
