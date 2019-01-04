use proc_macro2::{Group, TokenStream as TokenStream2};
use syn::{
    visit_mut::{self, VisitMut},
    *,
};

use crate::utils::Result;

use super::*;

pub(super) const DEFAULT_MARKER: &str = "marker";

pub(super) struct Visitor<'a> {
    builder: &'a mut Builder,
    marker: &'a str,
    attr: &'a mut bool,
    in_closure: isize,
    count_return: bool,
    unique_marker: bool,
    foreign: bool,
}

impl<'a> Visitor<'a> {
    pub(super) fn new(
        marker: &'a str,
        count_return: bool,
        is_closure: bool,
        attr: &'a mut bool,
        builder: &'a mut Builder,
    ) -> Self {
        Self {
            builder,
            marker,
            attr,
            in_closure: if is_closure { 2 } else { 1 },
            count_return,
            unique_marker: marker != DEFAULT_MARKER,
            foreign: false,
        }
    }

    fn foreign<F: FnOnce(&mut Self)>(&mut self, f: F) {
        let tmp = self.foreign;
        self.foreign = true;
        f(self);
        self.foreign = tmp;
    }
}

impl<'a> VisitMut for Visitor<'a> {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        let tmp = self.in_closure;

        if self.count_return {
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
        }

        if (!self.foreign && !expr.any_attr(NAME)) || self.unique_marker {
            visit_mut::visit_expr_mut(self, expr);

            replace_expr(expr, |expr| match expr {
                Expr::Macro(expr) => {
                    if expr.mac.path.is_ident(self.marker) {
                        let args = syn::parse2(expr.mac.tts).unwrap_or_else(|_| {
                            panic!("`{}` invalid tokens: the arguments of `{}!` macro must be an expression", NAME, self.marker)
                        });

                        self.builder.next_expr_with_attrs(expr.attrs, args)
                    } else {
                        Expr::Macro(expr)
                    }
                }
                expr => expr,
            });
        } else {
            self.foreign(|v| visit_mut::visit_expr_mut(v, expr));
        }

        self.in_closure = tmp;
    }

    fn visit_local_mut(&mut self, local: &mut Local) {
        if (!self.foreign && !local.any_attr(NAME)) || self.unique_marker {
            visit_mut::visit_local_mut(self, local);
        } else {
            self.foreign(|v| visit_mut::visit_local_mut(v, local));
        }
    }

    fn visit_stmt_mut(&mut self, stmt: &mut Stmt) {
        // Record whether other `auto_enum` exists.
        match stmt {
            Stmt::Expr(expr) | Stmt::Semi(expr, _) if expr.any_attr(NAME) => *self.attr = true,
            Stmt::Local(local) if local.any_attr(NAME) => *self.attr = true,
            _ => {}
        }

        visit_mut::visit_stmt_mut(self, stmt);
    }

    // Stop at item bounds
    fn visit_item_mut(&mut self, _item: &mut Item) {}
}

pub(super) struct Replacer {
    foreign: bool,
}

impl Replacer {
    pub(super) fn new() -> Self {
        Self { foreign: false }
    }

    fn foreign<F: FnOnce(&mut Self)>(&mut self, f: F) {
        let tmp = self.foreign;
        self.foreign = true;
        f(self);
        self.foreign = tmp;
    }

    fn find_remove_empty_attrs<A: AttrsMut>(&self, mut attrs: A) {
        EMPTY_ATTRS.iter().for_each(|ident| {
            attrs.find_remove_empty_attr(ident);
        });
    }
}

impl VisitMut for Replacer {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        if !expr.any_attr(NAME) {
            visit_mut::visit_expr_mut(self, expr);

            if !self.foreign {
                self.find_remove_empty_attrs(expr);
            }
        } else {
            self.foreign(|v| visit_mut::visit_expr_mut(v, expr));
        }
    }

    fn visit_arm_mut(&mut self, arm: &mut Arm) {
        visit_mut::visit_arm_mut(self, arm);

        if !self.foreign {
            self.find_remove_empty_attrs(arm);
        }
    }

    fn visit_local_mut(&mut self, local: &mut Local) {
        if !local.any_attr(NAME) {
            visit_mut::visit_local_mut(self, local);

            if !self.foreign {
                self.find_remove_empty_attrs(local);
            }
        } else {
            self.foreign(|v| visit_mut::visit_local_mut(v, local));
        }
    }

    // Stop at item bounds
    fn visit_item_mut(&mut self, _item: &mut Item) {}

    fn visit_stmt_mut(&mut self, stmt: &mut Stmt) {
        visit_mut::visit_stmt_mut(self, stmt);
        visit_stmt_mut(stmt).unwrap_or_else(|e| panic!("`{}` {}", NAME, e));
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
            if let Some(attr) = expr.find_remove_attr(NAME) {
                parse_tts(attr.tts).and_then(|params| parent_expr(expr, params))?;
            }
        }
        Stmt::Semi(expr, _) => {
            if let Some(attr) = expr.find_remove_attr(NAME) {
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
