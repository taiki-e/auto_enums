use proc_macro2::{Group, TokenStream as TokenStream2};
use syn::{
    visit::{self, Visit},
    visit_mut::{self, VisitMut},
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
        Self {
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

    // Stop at item bounds
    fn visit_item(&mut self, _item: &'ast Item) {}
}

pub(super) struct FnVisitor<'a> {
    marker: &'a str,
    builder: &'a mut Builder,
    in_closure: isize,
}

impl<'a> FnVisitor<'a> {
    pub(super) fn new(marker: &'a str, is_closure: bool, builder: &'a mut Builder) -> Self {
        Self {
            marker,
            builder,
            in_closure: if is_closure { 2 } else { 1 },
        }
    }
}

impl<'a> VisitMut for FnVisitor<'a> {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        let tmp = self.in_closure;
        if let Expr::Closure(_) = &expr {
            self.in_closure -= 1;
        }

        if self.in_closure > 0 && !expr.any_empty_attr(NEVER_ATTR) {
            if let Expr::Return(ret) = expr {
                let expr = match ret.expr.take().map_or_else(|| Expr::Tuple(unit()), |e| *e) {
                    Expr::Macro(expr) => {
                        if expr.mac.path.is_ident(self.marker) {
                            Expr::Macro(expr)
                        } else {
                            self.builder.next_expr(Expr::Macro(expr))
                        }
                    }
                    expr => self.builder.next_expr(expr),
                };
                ret.expr = Some(Box::new(expr));
            }
        }

        visit_mut::visit_expr_mut(self, expr);
        self.in_closure = tmp;
    }

    // Stop at item bounds
    fn visit_item_mut(&mut self, _item: &mut Item) {}
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
        Self {
            marker,
            marker_count,
            builder,
            empty_attrs: EMPTY_ATTRS,
            foreign: false,
        }
    }

    pub(super) fn dummy(builder: &'a mut Builder) -> Self {
        Self::new(DEFAULT_MARKER, 0, builder)
    }

    fn find_remove_empty_attrs(&self, attrs: &mut Vec<Attribute>) {
        self.empty_attrs.iter().for_each(|ident| {
            attrs.find_remove_empty_attr(ident);
        });
    }
}

impl<'a> VisitMut for Replacer<'a> {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        if !expr.any_attr(NAME) || (self.foreign && self.marker_count != 0) {
            visit_mut::visit_expr_mut(self, expr);

            if !self.foreign {
                attrs_mut(expr, |attrs| self.find_remove_empty_attrs(attrs));
            }

            if self.marker_count != 0 {
                replace_expr(expr, |expr| match expr {
                    Expr::Macro(expr) => {
                        if expr.mac.path.is_ident(self.marker) {
                            let args = syn::parse2(expr.mac.tts).unwrap_or_else(|_| {
                                panic!("`#[{}]` invalid tokens: the arguments of `{}!` macro must be an expression", NAME, self.marker)
                            });

                            self.marker_count -= 1;
                            self.builder.next_expr_with_attrs(expr.attrs, args)
                        } else {
                            Expr::Macro(expr)
                        }
                    }
                    expr => expr,
                });
            }
        } else {
            let tmp = self.foreign;
            self.foreign = true;
            visit_mut::visit_expr_mut(self, expr);
            self.foreign = tmp;
        }
    }

    fn visit_arm_mut(&mut self, arm: &mut Arm) {
        visit_mut::visit_arm_mut(self, arm);

        if !self.foreign {
            self.find_remove_empty_attrs(&mut arm.attrs);
        }
    }

    fn visit_local_mut(&mut self, local: &mut Local) {
        if !local.any_attr(NAME) || (self.foreign && self.marker_count != 0) {
            visit_mut::visit_local_mut(self, local);
        } else {
            let tmp = self.foreign;
            self.foreign = true;
            visit_mut::visit_local_mut(self, local);
            self.foreign = tmp;
        }
    }

    // Stop at item bounds
    fn visit_item_mut(&mut self, _item: &mut Item) {}

    fn visit_stmt_mut(&mut self, stmt: &mut Stmt) {
        visit_mut::visit_stmt_mut(self, stmt);
        visit_stmt_mut(stmt).unwrap_or_else(|e| panic!("`#[{}]` {}", NAME, e));
    }
}

fn visit_stmt_mut(stmt: &mut Stmt) -> Result<()> {
    fn parse_tts(tts: TokenStream2) -> Result<Params> {
        syn::parse2(tts)
            .map_err(|e| invalid_args!(e))
            .and_then(|group: Group| parse_args(group.stream()))
    }

    match stmt {
        Stmt::Expr(expr) => {
            if let Some(attr) = attrs_mut(expr, |attrs| attrs.find_remove_attr(NAME)) {
                parse_tts(attr.tts).and_then(|params| parent_expr(expr, params))?;
            }
        }
        Stmt::Semi(expr, _) => {
            if let Some(attr) = attrs_mut(expr, |attrs| attrs.find_remove_attr(NAME)) {
                parse_tts(attr.tts).and_then(|params| stmt_semi(expr, params))?;
            }
        }
        Stmt::Local(local) => {
            if let Some(attr) = local.find_remove_attr(NAME) {
                parse_tts(attr.tts).and_then(|params| stmt_let(local, params))?;
            }
        }
        _ => {}
    }

    Ok(())
}
