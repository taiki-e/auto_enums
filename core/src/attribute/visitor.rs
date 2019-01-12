use proc_macro2::Group;
use syn::{
    visit_mut::{self, VisitMut},
    *,
};

use crate::utils::Result;

use super::*;

const DEFAULT_MARKER: &str = "marker";

#[derive(Debug)]
pub(super) struct Marker {
    ident: Option<String>,
    root: bool,
}

impl Marker {
    pub(super) fn new(ident: Option<String>) -> Self {
        Self { ident, root: false }
    }

    pub(super) fn root(&mut self) {
        self.root = true;
    }

    pub(super) fn is_root(&self) -> bool {
        self.root
    }

    fn is_unique(&self) -> bool {
        self.ident.is_some()
    }

    fn ident(&self) -> &str {
        self.ident.as_ref().map_or(DEFAULT_MARKER, |s| s)
    }

    pub(super) fn marker_macro(&self, mac: &Macro) -> bool {
        match &self.ident {
            None => mac.path.is_ident(DEFAULT_MARKER),
            Some(marker) => {
                mac.path.is_ident(marker) || (!self.is_root() && mac.path.is_ident(DEFAULT_MARKER))
            }
        }
    }
}

pub(super) struct Visitor<'a> {
    builder: &'a mut Builder,
    marker: &'a Marker,
    attr: &'a mut bool,
    in_closure: isize,
    visit_return: bool,
    foreign: bool,
}

impl<'a> Visitor<'a> {
    pub(super) fn new(
        marker: &'a Marker,
        visit_return: bool,
        is_closure: bool,
        attr: &'a mut bool,
        builder: &'a mut Builder,
    ) -> Self {
        Self {
            builder,
            marker,
            attr,
            in_closure: if is_closure { 2 } else { 1 },
            visit_return,
            foreign: false,
        }
    }

    fn find_remove_empty_attrs<A: AttrsMut>(&self, attrs: &mut A) {
        if !self.foreign {
            EMPTY_ATTRS.iter().for_each(|ident| {
                attrs.find_remove_empty_attr(ident);
            });
        }
    }

    /// `return` in functions or closures
    fn visit_return(&mut self, expr: &mut Expr) {
        if !self.visit_return {
            return;
        } else if let Expr::Closure(_) = &expr {
            self.in_closure -= 1;
        }

        if self.in_closure > 0 && !expr.any_empty_attr(NEVER_ATTR) {
            if let Expr::Return(ExprReturn { expr, .. }) = expr {
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
    }

    /// Expression level marker (`marker!` macro)
    fn visit_marker(&mut self, expr: &mut Expr) {
        #[inline(never)]
        fn err(ident: &str) -> ! {
            panic!(
                "`{}` invalid tokens: the arguments of `{}!` macro must be an expression",
                NAME, ident
            )
        }

        if self.foreign && !self.marker.is_unique() {
            return;
        }

        replace_expr(expr, |expr| match expr {
            Expr::Macro(expr) => {
                if expr.mac.path.is_ident(self.marker.ident()) {
                    let args =
                        syn::parse2(expr.mac.tts).unwrap_or_else(|_| err(self.marker.ident()));

                    self.builder.next_expr_with_attrs(expr.attrs, args)
                } else {
                    Expr::Macro(expr)
                }
            }
            expr => expr,
        });

        self.find_remove_empty_attrs(expr);
    }
}

impl<'a> VisitMut for Visitor<'a> {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        let tmp_in_closure = self.in_closure;
        let tmp_foreign = self.foreign;

        self.visit_return(expr);

        if expr.any_attr(NAME) {
            self.foreign = true;
            // Record whether other `auto_enum` exists.
            *self.attr = true;
        }
        visit_mut::visit_expr_mut(self, expr);

        self.visit_marker(expr);
        self.in_closure = tmp_in_closure;
        self.foreign = tmp_foreign;
    }

    fn visit_arm_mut(&mut self, arm: &mut Arm) {
        visit_mut::visit_arm_mut(self, arm);
        self.find_remove_empty_attrs(arm);
    }

    fn visit_local_mut(&mut self, local: &mut Local) {
        let tmp = self.foreign;

        if local.any_attr(NAME) {
            self.foreign = true;
            // Record whether other `auto_enum` exists.
            *self.attr = true;
        }

        visit_mut::visit_local_mut(self, local);
        self.find_remove_empty_attrs(local);
        self.foreign = tmp;
    }

    fn visit_stmt_mut(&mut self, stmt: &mut Stmt) {
        visit_stmt_mut(self, stmt);
    }

    // Stop at item bounds
    fn visit_item_mut(&mut self, _item: &mut Item) {}
}

pub(super) struct Replacer;

impl VisitMut for Replacer {
    fn visit_stmt_mut(&mut self, stmt: &mut Stmt) {
        visit_stmt_mut(self, stmt);
    }

    // Stop at item bounds
    fn visit_item_mut(&mut self, _item: &mut Item) {}
}

fn visit_stmt_mut<V: VisitMut + ?Sized>(visitor: &mut V, stmt: &mut Stmt) {
    visit_mut::visit_stmt_mut(visitor, stmt);
    _visit_stmt_mut(stmt).unwrap_or_else(|e| panic!("`{}` {}", NAME, e));
}

fn _visit_stmt_mut(stmt: &mut Stmt) -> Result<()> {
    fn parse_attr<A: AttrsMut, F>(attrs: &mut A, f: F) -> Result<()>
    where
        F: FnOnce(&mut A, Params) -> Result<()>,
    {
        match attrs.find_remove_attr(NAME) {
            None => Ok(()),
            Some(Attribute { tts, .. }) => syn::parse2(tts)
                .map_err(|e| invalid_args!(e))
                .and_then(|group: Group| parse_args(group.stream()))
                .and_then(|params| f(attrs, params)),
        }
    }

    match stmt {
        Stmt::Expr(expr) => parse_attr(expr, parent_expr),
        Stmt::Semi(expr, _) => parse_attr(expr, stmt_semi),
        Stmt::Local(local) => parse_attr(local, stmt_let),
        _ => Ok(()),
    }
}
