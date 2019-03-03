use proc_macro2::Group;
use quote::quote;
use syn::{
    parse_quote,
    visit_mut::{self, VisitMut},
    Arm, Attribute, Expr, ExprMacro, ExprReturn, ExprTry, Item, Local, Macro, Stmt,
};

use super::{builder::Builder, *};

// =============================================================================
// Expression level marker

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

    pub(super) fn marker_macro(&self, Macro { path, .. }: &Macro) -> bool {
        match &self.ident {
            None => path.is_ident(DEFAULT_MARKER),
            Some(marker) => {
                path.is_ident(marker) || (!self.is_root() && path.is_ident(DEFAULT_MARKER))
            }
        }
    }
}

// =============================================================================
// Visitor

pub(super) enum VisitOption {
    Default,
    Return,
    Try,
}

impl VisitOption {
    pub(super) fn is_try(&self) -> bool {
        match self {
            VisitOption::Try => true,
            _ => false,
        }
    }
}

pub(super) struct Visitor<'a> {
    builder: &'a mut Builder,
    marker: &'a Marker,
    attr: &'a mut bool,
    in_closure: bool,
    in_try_block: bool,
    foreign: bool,
    visit_option: VisitOption,
}

impl<'a> Visitor<'a> {
    pub(super) fn new(
        builder: &'a mut Builder,
        marker: &'a Marker,
        attr: &'a mut bool,
        visit_option: VisitOption,
    ) -> Self {
        Self {
            builder,
            marker,
            attr,
            in_closure: false,
            in_try_block: false,
            foreign: false,
            visit_option,
        }
    }

    fn find_remove_empty_attrs<A: AttrsMut>(&self, attrs: &mut A) {
        if !self.foreign {
            EMPTY_ATTRS.iter().for_each(|ident| {
                attrs.find_remove_empty_attr(ident);
            });
        }
    }

    fn other_attr<A: Attrs>(&mut self, attrs: &A) {
        if attrs.any_attr(NAME) {
            self.foreign = true;
            // Record whether other `auto_enum` exists.
            *self.attr = true;
        }
    }

    /// `return` in functions or closures
    fn visit_return(&mut self, expr: &mut Expr) {
        if let Expr::Closure(_) = &expr {
            self.in_closure = true;
        }

        if !self.in_closure && !expr.any_empty_attr(NEVER) {
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

    /// `?` operator in functions or closures
    fn visit_try(&mut self, expr: &mut Expr) {
        match expr {
            Expr::Closure(_) => self.in_closure = true,
            // `?` operator in try blocks are not supported.
            Expr::TryBlock(_) => self.in_try_block = true,
            _ => {}
        }

        if !self.in_try_block && !self.in_closure && !expr.any_empty_attr(NEVER) {
            *expr = match expr {
                Expr::Try(ExprTry { expr, .. }) => {
                    if let Expr::Macro(ExprMacro { mac, .. }) = &**expr {
                        if self.marker.marker_macro(mac) {
                            return;
                        }
                    }

                    // https://github.com/rust-lang/rust/blob/master/src/librustc/hir/lowering.rs#L4436
                    let err = self.builder.next_expr(parse_quote!(err));
                    #[cfg(feature = "try_trait")]
                    {
                        parse_quote! {{
                            match ::core::ops::Try::into_result(#expr) {
                                ::core::result::Result::Ok(val) => val,
                                ::core::result::Result::Err(err) => return ::core::ops::Try::from_error(#err),
                            }
                        }}
                    }
                    #[cfg(not(feature = "try_trait"))]
                    {
                        parse_quote! {{
                            match #expr {
                                ::core::result::Result::Ok(val) => val,
                                ::core::result::Result::Err(err) => return ::core::result::Result::Err(#err),
                            }
                        }}
                    }
                }
                _ => return,
            };
        }
    }

    /// Expression level marker (`marker!` macro)
    fn visit_marker(&mut self, expr: &mut Expr) {
        if self.foreign && !self.marker.is_unique() {
            return;
        }

        replace_expr(expr, |expr| match expr {
            Expr::Macro(expr) => {
                if expr.mac.path.is_ident(self.marker.ident()) {
                    let args = syn::parse2(expr.mac.tts)
                        .unwrap_or_else(|_| parse_failed(self.marker.ident()));

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

struct Tmp {
    in_closure: bool,
    in_try_block: bool,
    foreign: bool,
}

impl Tmp {
    fn store(visitor: &Visitor<'_>) -> Self {
        Self {
            in_closure: visitor.in_closure,
            in_try_block: visitor.in_try_block,
            foreign: visitor.foreign,
        }
    }

    fn restore(self, visitor: &mut Visitor<'_>) {
        visitor.in_closure = self.in_closure;
        visitor.in_try_block = self.in_try_block;
        visitor.foreign = self.foreign;
    }
}

impl VisitMut for Visitor<'_> {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        let tmp = Tmp::store(self);
        self.other_attr(expr);

        match &self.visit_option {
            VisitOption::Return => self.visit_return(expr),
            VisitOption::Try => self.visit_try(expr),
            _ => {}
        }

        visit_mut::visit_expr_mut(self, expr);
        self.visit_marker(expr);
        tmp.restore(self);
    }

    fn visit_arm_mut(&mut self, arm: &mut Arm) {
        visit_mut::visit_arm_mut(self, arm);
        self.find_remove_empty_attrs(arm);
    }

    fn visit_local_mut(&mut self, local: &mut Local) {
        let tmp = self.foreign;
        self.other_attr(local);

        visit_mut::visit_local_mut(self, local);
        self.find_remove_empty_attrs(local);
        self.foreign = tmp;
    }

    fn visit_stmt_mut(&mut self, stmt: &mut Stmt) {
        visit_mut::visit_stmt_mut(self, stmt);
        visit_stmt_mut(stmt);
    }

    // Stop at item bounds
    fn visit_item_mut(&mut self, _item: &mut Item) {}
}

#[inline(never)]
#[cold]
fn parse_failed(ident: &str) -> ! {
    panic!(
        "`{}` invalid tokens: the arguments of `{}!` macro must be an expression",
        NAME, ident
    )
}

/// Find `?` operator.
pub(super) fn find_try<F>(marker: &Marker, f: F) -> bool
where
    F: FnOnce(&mut FindTry<'_>),
{
    let mut find = FindTry::new(marker);
    f(&mut find);
    find.has
}

pub(super) struct FindTry<'a> {
    marker: &'a Marker,
    in_closure: bool,
    foreign: bool,
    has: bool,
}

impl<'a> FindTry<'a> {
    fn new(marker: &'a Marker) -> Self {
        Self {
            marker,
            in_closure: false,
            foreign: false,
            has: false,
        }
    }
}

impl VisitMut for FindTry<'_> {
    fn visit_expr_mut(&mut self, expr: &mut Expr) {
        let tmp_in_closure = self.in_closure;
        let tmp_foreign = self.foreign;

        if let Expr::Closure(_) = &expr {
            self.in_closure = true;
        }

        if !self.in_closure && !expr.any_empty_attr(NEVER) {
            if let Expr::Try(ExprTry { expr, .. }) = expr {
                match &**expr {
                    Expr::Macro(expr) => {
                        if !self.marker.marker_macro(&expr.mac) {
                            self.has = true;
                        }
                    }
                    _ => self.has = true,
                }
            }
        }

        if expr.any_attr(NAME) {
            self.foreign = true;
        }
        if !self.has {
            visit_mut::visit_expr_mut(self, expr);
        }

        self.in_closure = tmp_in_closure;
        self.foreign = tmp_foreign;
    }

    fn visit_local_mut(&mut self, local: &mut Local) {
        let tmp = self.foreign;

        if local.any_attr(NAME) {
            self.foreign = true;
        }

        visit_mut::visit_local_mut(self, local);
        self.foreign = tmp;
    }

    // Stop at item bounds
    fn visit_item_mut(&mut self, _item: &mut Item) {}
}

pub(super) struct Dummy;

impl VisitMut for Dummy {
    fn visit_stmt_mut(&mut self, stmt: &mut Stmt) {
        visit_mut::visit_stmt_mut(self, stmt);
        visit_stmt_mut(stmt);
    }

    // Stop at item bounds
    fn visit_item_mut(&mut self, _item: &mut Item) {}
}

fn visit_stmt_mut(stmt: &mut Stmt) {
    // Stop at item bounds
    if let Stmt::Item(_) = stmt {
        return;
    }

    if let Some(Attribute { tts, .. }) = stmt.find_remove_attr(NAME) {
        syn::parse2(tts)
            .map_err(|e| invalid_args!(e))
            .and_then(|group: Group| parse_args(group.stream()))
            .and_then(|params| stmt.visit_parent(params))
            .unwrap_or_else(|e| panic!("`{}` {}", NAME, e));
    }
}
