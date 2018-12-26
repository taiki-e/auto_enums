use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{visit::Visit, visit_mut::VisitMut, *};

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
        .unwrap_or_else(|e| compile_err(&format!("`#[{}]` {}", NAME, e)))
        .into()
}

fn expand(args: TokenStream2, input: TokenStream) -> Result<TokenStream2> {
    let params = parse_args(args)?;

    match syn::parse(input.clone()) {
        Ok(mut stmt) => parent_stmt(&mut stmt, params).map(|()| stmt.into_token_stream()),
        Err(_) => syn::parse(input)
            .map_err(|_| "can only be used on expression, statement, or function".into())
            .and_then(|mut expr| parent_expr(&mut expr, params).map(|()| expr.into_token_stream())),
    }
}

fn parent_stmt(stmt: &mut Stmt, params: Params) -> Result<()> {
    match stmt {
        Stmt::Expr(expr) => parent_expr(expr, params),
        Stmt::Semi(expr, _) => stmt_semi(expr, params),
        Stmt::Local(local) => stmt_let(local, params),
        Stmt::Item(Item::Fn(item)) => item_fn(item, params),
        Stmt::Item(_) => Err(unsupported_item(
            "items other than function are not supported",
        )),
    }
}

fn parent_expr(expr: &mut Expr, mut params: Params) -> Result<()> {
    let mut builder = Builder::new();

    if params.args().is_empty() {
        Replacer::dummy(&mut builder).visit_expr_mut(expr);
        return Ok(());
    }

    params.count_marker(|c| c.visit_expr(&expr));

    if !params.never() {
        match expr {
            Expr::Closure(expr) => {
                params.fn_visitor(true, &mut builder, |v| v.visit_expr_mut(&mut *expr.body));
                child_expr(&mut *expr.body, &mut builder, &params)?;
            }
            _ => child_expr(expr, &mut builder, &params)?,
        }
    }

    if builder.len() + params.count() < 2 && params.attr() {
        Replacer::dummy(&mut builder).visit_expr_mut(expr);
        return Ok(());
    }

    params.build(&mut builder).map(|item| {
        replace_expr(expr, |expr| {
            expr_block(block(vec![Stmt::Item(item.into()), Stmt::Expr(expr)]))
        });
        params.replacer(&mut builder).visit_expr_mut(expr);
    })
}

fn stmt_semi(expr: &mut Expr, mut params: Params) -> Result<()> {
    let mut builder = Builder::new();

    if params.args().is_empty() {
        Replacer::dummy(&mut builder).visit_expr_mut(expr);
        return Ok(());
    }

    params.count_marker(|c| c.visit_expr(&expr));

    match params.count() {
        0 | 1 if !params.attr() => Err(unsupported_stmt(
            "expression with trailing semicolon is required two or more marker macros",
        ))?,
        0 => {
            Replacer::dummy(&mut builder).visit_expr_mut(expr);
            return Ok(());
        }
        _ => {}
    }

    params.build(&mut builder).map(|item| {
        replace_expr(expr, |expr| {
            expr_block(block(vec![Stmt::Item(item.into()), Stmt::Expr(expr)]))
        });
        params.replacer(&mut builder).visit_expr_mut(expr);
    })
}

fn stmt_let(local: &mut Local, mut params: Params) -> Result<()> {
    let mut builder = Builder::new();

    #[cfg(feature = "type_analysis")]
    {
        if let Some((_, ty)) = &local.ty {
            params.impl_traits(&*ty);
        }
    }

    if params.args().is_empty() {
        Replacer::dummy(&mut builder).visit_local_mut(local);
        return Ok(());
    }

    params.count_marker(|c| c.visit_local(&local));

    let mut expr = (local.init)
        .take()
        .map(|(_, expr)| expr)
        .ok_or_else(|| unsupported_stmt("uninitialized let statement"))?;

    if !params.never() {
        match &mut *expr {
            Expr::Closure(expr) => {
                params.fn_visitor(true, &mut builder, |v| v.visit_expr_mut(&mut *expr.body));
                child_expr(&mut *expr.body, &mut builder, &params)?;
            }
            expr => child_expr(expr, &mut builder, &params)?,
        }
    }

    if builder.len() + params.count() < 2 && params.attr() {
        local.init = Some((default(), expr));
        Replacer::dummy(&mut builder).visit_local_mut(local);
        return Ok(());
    }

    params.build(&mut builder).map(|item| {
        replace_expr(&mut *expr, |expr| {
            expr_block(block(vec![Stmt::Item(item.into()), Stmt::Expr(expr)]))
        });
        local.init = Some((default(), expr));

        params.replacer(&mut builder).visit_local_mut(local);
    })
}

fn item_fn(item: &mut ItemFn, mut params: Params) -> Result<()> {
    let mut builder = Builder::new();
    let mut return_impl_trait = false;

    if let ReturnType::Type(_, ty) = &item.decl.output {
        if let Type::ImplTrait(_) = &**ty {
            return_impl_trait = true;
        }

        #[cfg(feature = "type_analysis")]
        params.impl_traits(&*ty);
    }

    if params.args().is_empty() {
        Replacer::dummy(&mut builder).visit_item_fn_mut(item);
        return Ok(());
    }

    params.count_marker(|c| c.visit_item_fn(&item));

    if !params.never() && return_impl_trait {
        params.fn_visitor(false, &mut builder, |v| v.visit_item_fn_mut(item));
    }

    match (*item.block).stmts.last_mut() {
        Some(Stmt::Expr(expr)) if !params.never() => child_expr(expr, &mut builder, &params)?,
        Some(_) => {}
        None => Err(unsupported_item("empty function"))?,
    }

    if builder.len() + params.count() < 2 {
        if !params.attr() {
            Err(unsupported_item("for function that returns a non-expression statement, you need to specify `marker` macros"))?;
        }

        Replacer::dummy(&mut builder).visit_item_fn_mut(item);
        return Ok(());
    }

    params.build(&mut builder).map(|i| {
        let mut stmts = Vec::with_capacity((*item.block).stmts.len() + 1);
        stmts.push(Stmt::Item(i.into()));
        stmts.append(&mut (*item.block).stmts);
        (*item.block).stmts = stmts;

        params.replacer(&mut builder).visit_item_fn_mut(item);
    })
}
