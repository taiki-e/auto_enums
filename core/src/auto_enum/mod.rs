use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::*;

use crate::utils::{block, expr_block, replace_expr};

mod context;
mod expr;
#[cfg(feature = "type_analysis")]
mod type_analysis;
mod visitor;

use self::{
    context::{Context, VisitLastMode, VisitMode, DEFAULT_MARKER},
    expr::child_expr,
};

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
        Ok(mut stmt) => expand_parent_stmt(&mut stmt, &mut cx).map(|()| stmt.into_token_stream()),
        Err(e) => syn::parse2::<Expr>(input)
            .map_err(|_e| {
                cx.diagnostic.error(e);
                error!(cx.span, "may only be used on expression, statement, or function")
            })
            .and_then(|mut expr| {
                expand_parent_expr(&mut expr, &mut cx, false).map(|()| expr.into_token_stream())
            }),
    };

    match res {
        Err(e) => cx.diagnostic.error(e),
        Ok(_) if cx.failed() => {}
        Ok(tokens) => return tokens,
    }
    cx.diagnostic.to_compile_error().unwrap()
}

fn expand_expr(expr: &mut Expr, cx: &mut Context) -> Result<()> {
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

    child_expr(expr, cx)?;

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

fn expand_parent_stmt(stmt: &mut Stmt, cx: &mut Context) -> Result<()> {
    match stmt {
        Stmt::Expr(expr) => expand_parent_expr(expr, cx, false),
        Stmt::Semi(expr, _) => expand_parent_expr(expr, cx, true),
        Stmt::Local(local) => expand_parent_local(local, cx),
        Stmt::Item(Item::Fn(item)) => expand_parent_item_fn(item, cx),
        Stmt::Item(item) => {
            Err(error!(item, "may only be used on expression, statement, or function"))
        }
    }
}

fn expand_parent_expr(expr: &mut Expr, cx: &mut Context, has_semi: bool) -> Result<()> {
    if has_semi {
        cx.visit_last_mode = VisitLastMode::Never;
    }

    if cx.is_dummy() {
        cx.dummy(expr);
        return Ok(());
    }

    expand_expr(expr, cx)?;

    cx.build(|item| build_expr(expr, item))
}

fn expand_parent_local(local: &mut Local, cx: &mut Context) -> Result<()> {
    #[cfg(feature = "type_analysis")]
    {
        if let Pat::Type(PatType { ty, .. }) = &mut local.pat {
            cx.collect_trait(&mut *ty);
        }
    }

    if cx.is_dummy() {
        cx.dummy(local);
        return Ok(());
    }

    let expr = if let Some((_, expr)) = &mut local.init {
        &mut **expr
    } else {
        return Err(error!(
            local,
            "the `#[auto_enum]` attribute is not supported uninitialized let statement"
        ));
    };

    expand_expr(expr, cx)?;

    cx.build(|item| build_expr(expr, item))
}

fn expand_parent_item_fn(item: &mut ItemFn, cx: &mut Context) -> Result<()> {
    #[cfg(auto_enums_def_site_enum_ident)]
    cx.update_enum_ident(item);

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
        cx.collect_trait(&mut *ty);
    }

    if cx.is_dummy() {
        cx.dummy(item);
        return Ok(());
    }

    match item.block.stmts.last_mut() {
        Some(Stmt::Expr(expr)) => child_expr(expr, cx)?,
        Some(_) => {}
        None => {
            return Err(error!(
                item.block,
                "the `#[auto_enum]` attribute is not supported empty functions"
            ));
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
