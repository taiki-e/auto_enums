use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{visit_mut::VisitMut, *};

use crate::utils::{Result, *};

mod attrs;
mod builder;
mod expr;
mod params;
#[cfg(feature = "type_analysis")]
mod traits;
mod visitor;

use self::attrs::*;
use self::builder::Builder;
use self::expr::*;
use self::params::*;
#[cfg(feature = "type_analysis")]
use self::traits::*;
use self::visitor::*;

const NAME: &str = "auto_enum";

pub(crate) fn attribute(args: TokenStream, input: TokenStream) -> TokenStream {
    expand(args.into(), input)
        .unwrap_or_else(|e| compile_err(&format!("`{}` {}", NAME, e)))
        .into()
}

fn expand(args: TokenStream2, input: TokenStream) -> Result<TokenStream2> {
    let mut params = parse_args(args)?;
    params.root();

    match syn::parse(input.clone()) {
        Ok(mut stmt) => parent_stmt(&mut stmt, params).map(|()| stmt.into_token_stream()),
        Err(_) => syn::parse(input)
            .map_err(|_| "may only be used on expression, statement, or function".into())
            .and_then(|mut expr| parent_expr(&mut expr, params).map(|()| expr.into_token_stream())),
    }
}

fn parent_stmt(stmt: &mut Stmt, params: Params) -> Result<()> {
    match stmt {
        Stmt::Expr(expr) => parent_expr(expr, params),
        Stmt::Semi(expr, _) => stmt_semi(expr, params),
        Stmt::Local(local) => stmt_let(local, params),
        Stmt::Item(Item::Fn(item)) => item_fn(item, params),
        Stmt::Item(_) => Err("may only be used on expression, statement, or function".into()),
    }
}

fn build_expr(expr: &mut Expr, builder: &mut Builder, params: &Params) -> Result<()> {
    builder.build(params.args()).map(|item| {
        replace_expr(expr, |expr| {
            expr_block(block(vec![Stmt::Item(item.into()), Stmt::Expr(expr)]))
        });
    })
}

fn parent_expr(expr: &mut Expr, mut params: Params) -> Result<()> {
    let mut builder = Builder::new();

    if params.args().is_empty() {
        Dummy.visit_expr_mut(expr);
        return Ok(());
    }

    match expr {
        Expr::Closure(ExprClosure { body, .. }) if !params.never() => {
            child_expr(&mut **body, &mut builder, &params)?;

            params.fn_visitor(&mut builder, |v| v.visit_expr_mut(&mut **body));
        }
        _ => {
            if !params.never() {
                child_expr(expr, &mut builder, &params)?;
            }

            params.visitor(&mut builder, |v| v.visit_expr_mut(expr));
        }
    }

    match builder.len() {
        0 | 1 if !params.attr() => Err(unsupported_expr(
            "is required two or more branches or marker macros in total",
        )),
        0 => Ok(()),
        _ => build_expr(expr, &mut builder, &params),
    }
}

fn stmt_semi(expr: &mut Expr, mut params: Params) -> Result<()> {
    let mut builder = Builder::new();

    if params.args().is_empty() {
        Dummy.visit_expr_mut(expr);
        return Ok(());
    }

    params.visitor(&mut builder, |c| c.visit_expr_mut(expr));

    match builder.len() {
        0 | 1 if !params.attr() => Err(unsupported_stmt(
            "expression with trailing semicolon is required two or more marker macros",
        )),
        0 => Ok(()),
        _ => build_expr(expr, &mut builder, &params),
    }
}

fn stmt_let(local: &mut Local, mut params: Params) -> Result<()> {
    let mut builder = Builder::new();

    #[cfg(feature = "type_analysis")]
    {
        if let Some((_, ty)) = &mut local.ty {
            params.impl_traits(&mut *ty);
        }
    }

    if params.args().is_empty() {
        Dummy.visit_local_mut(local);
        return Ok(());
    }

    let mut expr = (local.init)
        .take()
        .map(|(_, expr)| expr)
        .ok_or_else(|| unsupported_stmt("uninitialized let statement"))?;

    match &mut *expr {
        Expr::Closure(ExprClosure { body, .. }) if !params.never() => {
            child_expr(&mut **body, &mut builder, &params)?;

            params.fn_visitor(&mut builder, |v| v.visit_expr_mut(&mut **body));
        }
        expr => {
            if !params.never() {
                child_expr(expr, &mut builder, &params)?;
            }

            params.visitor(&mut builder, |c| c.visit_expr_mut(expr));
        }
    }

    match builder.len() {
        0 | 1 if !params.attr() => Err(unsupported_stmt(
            "is required two or more branches or marker macros in total",
        ))?,
        0 => {}
        _ => build_expr(&mut *expr, &mut builder, &params)?,
    }

    local.init = Some((default(), expr));
    Ok(())
}

fn item_fn(item: &mut ItemFn, mut params: Params) -> Result<()> {
    let mut builder = Builder::new();
    let mut return_impl_trait = false;

    if let ReturnType::Type(_, ty) = &mut item.decl.output {
        if let Type::ImplTrait(_) = &**ty {
            return_impl_trait = true;
        }

        #[cfg(feature = "type_analysis")]
        params.impl_traits(&mut *ty);
    }

    if params.args().is_empty() {
        Dummy.visit_item_fn_mut(item);
        return Ok(());
    }

    match (*item.block).stmts.last_mut() {
        Some(Stmt::Expr(expr)) if !params.never() => child_expr(expr, &mut builder, &params)?,
        Some(_) => {}
        None => Err(unsupported_item("empty function"))?,
    }

    if !params.never() && return_impl_trait {
        params.fn_visitor(&mut builder, |v| v.visit_item_fn_mut(item));
    } else {
        params.visitor(&mut builder, |v| v.visit_item_fn_mut(item));
    }

    match builder.len() {
        0 | 1 if !params.attr() => Err(unsupported_item(
            "is required two or more branches or marker macros in total",
        )),
        0 => Ok(()),
        _ => builder.build(params.args()).map(|i| {
            let mut stmts = Vec::with_capacity((*item.block).stmts.len() + 1);
            stmts.push(Stmt::Item(i.into()));
            stmts.append(&mut (*item.block).stmts);
            (*item.block).stmts = stmts;
        }),
    }
}
