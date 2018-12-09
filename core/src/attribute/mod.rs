use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{fold::Fold, visit::Visit, *};

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
        Ok(stmt) => parent_stmt(stmt, params).map(ToTokens::into_token_stream),
        Err(_) => syn::parse(input)
            .map_err(|_| "can only be used on expression, statement, or function".into())
            .and_then(|expr| parent_expr(expr, params))
            .map(ToTokens::into_token_stream),
    }
}

fn parent_stmt(stmt: Stmt, params: Params) -> Result<Stmt> {
    match stmt {
        Stmt::Expr(expr) => parent_expr(expr, params).map(Stmt::Expr),
        Stmt::Semi(expr, semi) => stmt_semi(expr, params).map(|expr| Stmt::Semi(expr, semi)),
        Stmt::Local(local) => stmt_let(local, params).map(Stmt::Local),
        Stmt::Item(Item::Fn(item)) => item_fn(item, params).map(|item| Stmt::Item(Item::Fn(item))),
        Stmt::Item(_) => Err(unsupported_item(
            "items other than function are not supported",
        )),
    }
}

fn parent_expr(mut expr: Expr, mut params: Params) -> Result<Expr> {
    let mut builder = Builder::new();

    if params.args().is_empty() {
        return Ok(Replacer::dummy(&mut builder).fold_expr(expr));
    }

    params.count_marker(|c| c.visit_expr(&expr));

    if !params.never() {
        child_expr(&mut expr, &mut builder, &params)?;
    }

    if builder.is_empty() && !params.marker() && params.attr() {
        return Ok(Replacer::dummy(&mut builder).fold_expr(expr));
    }

    params
        .build(&mut builder)
        .map(|item| expr_block(block(vec![Stmt::Item(item.into()), Stmt::Expr(expr)])))
        .map(|expr| params.replacer(&mut builder).fold_expr(expr))
}

fn stmt_semi(expr: Expr, mut params: Params) -> Result<Expr> {
    let mut builder = Builder::new();

    if params.args().is_empty() {
        return Ok(Replacer::dummy(&mut builder).fold_expr(expr));
    }

    params.count_marker(|c| c.visit_expr(&expr));

    if !params.marker() {
        if !params.attr() {
            Err(unsupported_stmt(
                "expression with trailing semicolon is required two or more marker macros",
            ))?;
        }

        return Ok(Replacer::dummy(&mut builder).fold_expr(expr));
    }

    params
        .build(&mut builder)
        .map(|item| expr_block(block(vec![Stmt::Item(item.into()), Stmt::Expr(expr)])))
        .map(|expr| params.replacer(&mut builder).fold_expr(expr))
}

fn stmt_let(mut local: Local, mut params: Params) -> Result<Local> {
    let mut builder = Builder::new();

    #[cfg(feature = "type_analysis")]
    {
        if let Some((_, ty)) = &local.ty {
            params.impl_traits(&*ty);
        }
    }

    if params.args().is_empty() {
        return Ok(Replacer::dummy(&mut builder).fold_local(local));
    }

    params.count_marker(|c| c.visit_local(&local));

    let mut expr = (local.init)
        .take()
        .map(|(_, expr)| expr)
        .ok_or_else(|| unsupported_stmt("uninitialized let statement"))?;

    if !params.never() {
        child_expr(&mut *expr, &mut builder, &params)?;
    }

    if builder.is_empty() && !params.marker() && params.attr() {
        local.init = Some((default(), expr));
        return Ok(Replacer::dummy(&mut builder).fold_local(local));
    }

    params.build(&mut builder).map(|item| {
        let expr = expr_block(block(vec![Stmt::Item(item.into()), Stmt::Expr(*expr)]));
        local.init = Some((default(), Box::new(expr)));

        params.replacer(&mut builder).fold_local(local)
    })
}

fn item_fn(mut item: ItemFn, mut params: Params) -> Result<ItemFn> {
    let mut builder = Builder::new();

    #[cfg(feature = "type_analysis")]
    {
        if let ReturnType::Type(_, ty) = &item.decl.output {
            params.impl_traits(&*ty);
        }
    }

    if params.args().is_empty() {
        return Ok(Replacer::dummy(&mut builder).fold_item_fn(item));
    }

    params.count_marker(|c| c.visit_item_fn(&item));

    match (*item.block).stmts.last_mut() {
        Some(Stmt::Expr(expr)) if !params.never() => child_expr(expr, &mut builder, &params)?,
        Some(_) => {}
        None => Err(unsupported_item("empty function"))?,
    }

    if builder.is_empty() && !params.marker() {
        if !params.attr() {
            Err(unsupported_item("for function that returns a non-expression statement, you need to specify `manual` option"))?;
        }

        return Ok(Replacer::dummy(&mut builder).fold_item_fn(item));
    }

    params.build(&mut builder).map(|i| {
        let mut stmts = Vec::with_capacity((*item.block).stmts.len() + 1);
        stmts.push(Stmt::Item(i.into()));
        stmts.append(&mut (*item.block).stmts);
        (*item.block).stmts = stmts;

        params.replacer(&mut builder).fold_item_fn(item)
    })
}
