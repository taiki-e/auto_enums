use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{visit_mut::VisitMut, *};

use crate::utils::*;

mod context;
mod expr;
#[cfg(feature = "type_analysis")]
mod type_analysis;
mod visitor;

use self::context::{Context, VisitLastMode, VisitMode, DEFAULT_MARKER};
use self::expr::child_expr;

/// The attribute name.
const NAME: &str = "auto_enum";

/// The annotation for recursively parsing.
const NESTED: &str = "nested";
/// The annotation for skipping branch.
const NEVER: &str = "never";
/// The annotations used by `#[auto_enum]`.
const EMPTY_ATTRS: &[&str] = &[NEVER, NESTED];

/// The old annotation replaced by `#[nested]`.
const NESTED_OLD: &str = "rec";

pub(crate) fn attribute(args: TokenStream, input: TokenStream) -> TokenStream {
    expand(args, input).unwrap_or_else(|e| e.to_compile_error())
}

fn expand(args: TokenStream, input: TokenStream) -> Result<TokenStream> {
    let mut cx = Context::root(input.clone(), args)?;

    let res = match syn::parse2::<Stmt>(input.clone()) {
        Ok(mut stmt) => expand_parent_stmt(&mut stmt, &mut cx).map(|()| stmt.into_token_stream()),
        Err(e) => syn::parse2::<Expr>(input)
            .map_err(|_e| {
                cx.diagnostic.error(e);
                error!(cx.span, "may only be used on expression, statement, or function")
            })
            .and_then(|mut expr| {
                expand_parent_expr(&mut expr, &mut cx).map(|()| expr.into_token_stream())
            }),
    };

    match res {
        Err(e) => cx.diagnostic.error(e),
        Ok(_) if cx.failed() => {}
        Ok(tokens) => return Ok(tokens),
    }
    Err(cx.diagnostic.get_inner().unwrap())
}

fn visit_expr(expr: &mut Expr, cx: &mut Context) -> Result<()> {
    let expr = match expr {
        Expr::Closure(ExprClosure { body, .. }) if cx.visit_last() => {
            cx.visit_mode = VisitMode::Return;
            cx.visit_last_mode = VisitLastMode::Closure;
            cx.find_try(|v| v.visit_expr_mut(body));
            &mut **body
        }
        _ => expr,
    };

    child_expr(expr, cx).map(|()| cx.visitor(|v| v.visit_expr_mut(expr)))
}

fn build_expr(expr: &mut Expr, item: ItemEnum) {
    replace_expr(expr, |expr| expr_block(block(vec![Stmt::Item(item.into()), Stmt::Expr(expr)])));
}

// =================================================================================================
// Expand statement or expression in which `#[auto_enum]` was directly used.

fn expand_parent_stmt(stmt: &mut Stmt, cx: &mut Context) -> Result<()> {
    if let Stmt::Semi(..) = &stmt {
        cx.visit_last_mode = VisitLastMode::Never;
    }

    match stmt {
        Stmt::Expr(expr) | Stmt::Semi(expr, _) => expand_parent_expr(expr, cx),
        Stmt::Local(local) => expand_parent_local(local, cx),
        Stmt::Item(Item::Fn(item)) => expand_parent_item_fn(item, cx),
        Stmt::Item(item) => {
            Err(error!(item, "may only be used on expression, statement, or function"))
        }
    }
}

fn expand_parent_expr(expr: &mut Expr, cx: &mut Context) -> Result<()> {
    if cx.is_dummy() {
        cx.dummy(|v| v.visit_expr_mut(expr));
        return Ok(());
    }

    visit_expr(expr, cx)?;

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
        cx.dummy(|v| v.visit_local_mut(local));
        return Ok(());
    }

    let expr = match local.init.as_mut().map(|(_, expr)| &mut **expr) {
        Some(expr) => expr,
        None => {
            return Err(error!(
                local,
                "the `#[auto_enum]` attribute is not supported uninitialized let statement"
            ))
        }
    };

    visit_expr(expr, cx)?;

    cx.build(|item| build_expr(expr, item))
}

fn expand_parent_item_fn(item: &mut ItemFn, cx: &mut Context) -> Result<()> {
    let ItemFn { sig, block, .. } = item;
    if let ReturnType::Type(_, ty) = &mut sig.output {
        match &**ty {
            // `return`
            Type::ImplTrait(_) if cx.visit_last() => cx.visit_mode = VisitMode::Return,

            // `?` operator
            Type::Path(TypePath { qself: None, path }) if cx.visit_last() => {
                let PathSegment { arguments, ident } = &path.segments[path.segments.len() - 1];
                match arguments {
                    // `Result<T, impl Trait>`
                    PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                        colon2_token: None,
                        args,
                        ..
                    }) if args.len() == 2 && ident == "Result" => {
                        if let (
                            GenericArgument::Type(_),
                            GenericArgument::Type(Type::ImplTrait(_)),
                        ) = (&args[0], &args[1])
                        {
                            cx.find_try(|v| v.visit_block_mut(&mut **block));
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
        cx.dummy(|v| v.visit_item_fn_mut(item));
        return Ok(());
    }

    match item.block.stmts.last_mut() {
        Some(Stmt::Expr(expr)) => child_expr(expr, cx)?,
        Some(_) => {}
        None => {
            return Err(error!(
                item.block,
                "the `#[auto_enum]` attribute is not supported empty functions"
            ))
        }
    }

    cx.visitor(|v| v.visit_item_fn_mut(item));

    cx.build(|i| item.block.stmts.insert(0, Stmt::Item(i.into())))
}
