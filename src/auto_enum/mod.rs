mod context;
mod expr;
#[cfg(feature = "type_analysis")]
mod type_analysis;
mod visitor;

use proc_macro2::TokenStream;
use quote::ToTokens;
#[cfg(feature = "type_analysis")]
use syn::Pat;
use syn::{
    AngleBracketedGenericArguments, Expr, ExprClosure, GenericArgument, Item, ItemEnum, ItemFn,
    Local, PathArguments, Result, ReturnType, Stmt, Type, TypePath,
};

use self::{
    context::{Context, VisitLastMode, VisitMode, DEFAULT_MARKER},
    expr::child_expr,
};
use crate::utils::{block, expr_block, replace_expr};

/// The attribute name.
const NAME: &str = "auto_enum";
/// The annotation for recursively parsing.
const NESTED: &str = "nested";
/// The annotation for skipping branch.
const NEVER: &str = "never";

pub(crate) fn attribute(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut cx = match Context::root(input.clone(), args) {
        Err(e) => return e.to_compile_error(),
        Ok(cx) => cx,
    };

    let res = match syn::parse2::<Stmt>(input.clone()) {
        Ok(mut stmt) => expand_parent_stmt(&mut cx, &mut stmt).map(|()| stmt.into_token_stream()),
        Err(e) => syn::parse2::<Expr>(input)
            .map_err(|_e| {
                cx.error(e);
                format_err!(cx.span, "may only be used on expression, statement, or function")
            })
            .and_then(|mut expr| {
                expand_parent_expr(&mut cx, &mut expr, false).map(|()| expr.into_token_stream())
            }),
    };

    match res {
        Err(e) => cx.error(e),
        Ok(_) if cx.has_error() => {}
        Ok(tokens) => return tokens,
    }
    cx.compile_error().unwrap()
}

fn expand_expr(cx: &mut Context, expr: &mut Expr) -> Result<()> {
    let expr = match expr {
        Expr::Closure(ExprClosure { body, .. }) if cx.visit_last() => {
            let count = visitor::visit_fn(cx, &mut **body);
            if count.try_ >= 2 {
                cx.visit_mode = VisitMode::Try;
            } else {
                cx.visit_mode = VisitMode::Return(count.return_);
            }
            &mut **body
        }
        _ => expr,
    };

    child_expr(cx, expr)?;

    #[cfg(feature = "type_analysis")]
    {
        if let VisitMode::Return(count) = cx.visit_mode {
            if cx.args.is_empty() && cx.variant_is_empty() && count < 2 {
                cx.dummy(expr);
                return Ok(());
            }
        }
    }

    cx.visitor(expr);

    Ok(())
}

fn build_expr(expr: &mut Expr, item: ItemEnum) {
    replace_expr(expr, |expr| expr_block(block(vec![Stmt::Item(item.into()), Stmt::Expr(expr)])));
}

// =================================================================================================
// Expand statement or expression in which `#[auto_enum]` was directly used.

fn expand_parent_stmt(cx: &mut Context, stmt: &mut Stmt) -> Result<()> {
    match stmt {
        Stmt::Expr(expr) => expand_parent_expr(cx, expr, false),
        Stmt::Semi(expr, _) => expand_parent_expr(cx, expr, true),
        Stmt::Local(local) => expand_parent_local(cx, local),
        Stmt::Item(Item::Fn(item)) => expand_parent_item_fn(cx, item),
        Stmt::Item(item) => {
            bail!(item, "may only be used on expression, statement, or function");
        }
    }
}

fn expand_parent_expr(cx: &mut Context, expr: &mut Expr, has_semi: bool) -> Result<()> {
    if has_semi {
        cx.visit_last_mode = VisitLastMode::Never;
    }

    if cx.is_dummy() {
        cx.dummy(expr);
        return Ok(());
    }

    expand_expr(cx, expr)?;

    cx.build(|item| build_expr(expr, item))
}

fn expand_parent_local(cx: &mut Context, local: &mut Local) -> Result<()> {
    #[cfg(feature = "type_analysis")]
    {
        if let Pat::Type(pat) = &mut local.pat {
            if cx.collect_impl_trait(&mut pat.ty) {
                local.pat = (*pat.pat).clone()
            }
        }
    }

    if cx.is_dummy() {
        cx.dummy(local);
        return Ok(());
    }

    let expr = if let Some((_, expr)) = &mut local.init {
        &mut **expr
    } else {
        bail!(local, "the `#[auto_enum]` attribute is not supported uninitialized let statement");
    };

    expand_expr(cx, expr)?;

    cx.build(|item| build_expr(expr, item))
}

fn expand_parent_item_fn(cx: &mut Context, item: &mut ItemFn) -> Result<()> {
    let ItemFn { sig, block, .. } = item;
    if let ReturnType::Type(_, ty) = &mut sig.output {
        match &**ty {
            // `return`
            Type::ImplTrait(_) if cx.visit_last_mode != VisitLastMode::Never => {
                let count = visitor::visit_fn(cx, &mut **block);
                cx.visit_mode = VisitMode::Return(count.return_);
            }

            // `?` operator
            Type::Path(TypePath { qself: None, path })
                if cx.visit_last_mode != VisitLastMode::Never =>
            {
                let ty = path.segments.last().unwrap();
                match &ty.arguments {
                    // `Result<T, impl Trait>`
                    PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                        colon2_token: None,
                        args,
                        ..
                    }) if args.len() == 2 && ty.ident == "Result" => {
                        if let (
                            GenericArgument::Type(_),
                            GenericArgument::Type(Type::ImplTrait(_)),
                        ) = (&args[0], &args[1])
                        {
                            let count = visitor::visit_fn(cx, &mut **block);
                            if count.try_ >= 2 {
                                cx.visit_mode = VisitMode::Try;
                            }
                        }
                    }
                    _ => {}
                }
            }

            _ => {}
        }

        #[cfg(feature = "type_analysis")]
        cx.collect_impl_trait(&mut *ty);
    }

    if cx.is_dummy() {
        cx.dummy(item);
        return Ok(());
    }

    match item.block.stmts.last_mut() {
        Some(Stmt::Expr(expr)) => child_expr(cx, expr)?,
        Some(_) => {}
        None => {
            bail!(item.block, "the `#[auto_enum]` attribute is not supported empty functions");
        }
    }

    #[cfg(feature = "type_analysis")]
    {
        if let VisitMode::Return(count) = cx.visit_mode {
            if cx.args.is_empty() && cx.variant_is_empty() && count < 2 {
                cx.dummy(item);
                return Ok(());
            }
        }
    }

    cx.visitor(item);

    cx.build(|i| item.block.stmts.insert(0, Stmt::Item(i.into())))
}
