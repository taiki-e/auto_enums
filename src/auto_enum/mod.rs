// SPDX-License-Identifier: Apache-2.0 OR MIT

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
    AngleBracketedGenericArguments, Error, Expr, ExprClosure, GenericArgument, Item, ItemEnum,
    ItemFn, Local, LocalInit, PathArguments, ReturnType, Stmt, Type, TypePath,
};

use self::{
    context::{Context, VisitLastMode, VisitMode, DEFAULT_MARKER},
    expr::child_expr,
};
use crate::utils::{block, expr_block, path_eq, replace_expr};

/// The attribute name.
const NAME: &str = "auto_enum";
/// The annotation for recursively parsing.
const NESTED: &str = "nested";
/// The annotation for skipping branch.
const NEVER: &str = "never";

pub(crate) fn attribute(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut cx = match Context::root(input.clone(), args) {
        Ok(cx) => cx,
        Err(e) => return e.to_compile_error(),
    };

    match syn::parse2::<Stmt>(input.clone()) {
        Ok(mut stmt) => {
            expand_parent_stmt(&mut cx, &mut stmt);
            cx.check().map(|()| stmt.into_token_stream())
        }
        Err(e) => match syn::parse2::<Expr>(input) {
            Err(_e) => {
                cx.error(e);
                cx.error(format_err!(
                    cx.span,
                    "may only be used on expression, statement, or function"
                ));
                cx.check().map(|()| unreachable!())
            }
            Ok(mut expr) => {
                expand_parent_expr(&mut cx, &mut expr, false);
                cx.check().map(|()| expr.into_token_stream())
            }
        },
    }
    .unwrap_or_else(Error::into_compile_error)
}

fn expand_expr(cx: &mut Context, expr: &mut Expr) {
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

    child_expr(cx, expr);

    #[cfg(feature = "type_analysis")]
    {
        if let VisitMode::Return(count) = cx.visit_mode {
            if cx.args.is_empty() && cx.variant_is_empty() && count < 2 {
                cx.dummy(expr);
                return;
            }
        }
    }

    cx.visitor(expr);
}

fn build_expr(expr: &mut Expr, item: ItemEnum) {
    replace_expr(expr, |expr| {
        expr_block(block(vec![Stmt::Item(item.into()), Stmt::Expr(expr, None)]))
    });
}

// =================================================================================================
// Expand statement or expression in which `#[auto_enum]` was directly used.

fn expand_parent_stmt(cx: &mut Context, stmt: &mut Stmt) {
    match stmt {
        Stmt::Expr(expr, semi) => expand_parent_expr(cx, expr, semi.is_some()),
        Stmt::Local(local) => expand_parent_local(cx, local),
        Stmt::Item(Item::Fn(item)) => expand_parent_item_fn(cx, item),
        Stmt::Item(item) => {
            cx.error(format_err!(item, "may only be used on expression, statement, or function"));
        }
        Stmt::Macro(_) => {}
    }
}

fn expand_parent_expr(cx: &mut Context, expr: &mut Expr, has_semi: bool) {
    if has_semi {
        cx.visit_last_mode = VisitLastMode::Never;
    }

    if cx.is_dummy() {
        cx.dummy(expr);
        return;
    }

    expand_expr(cx, expr);

    cx.build(|item| build_expr(expr, item));
}

fn expand_parent_local(cx: &mut Context, local: &mut Local) {
    #[cfg(feature = "type_analysis")]
    {
        if let Pat::Type(pat) = &mut local.pat {
            if cx.collect_impl_trait(&mut pat.ty) {
                local.pat = (*pat.pat).clone();
            }
        }
    }

    if cx.is_dummy() {
        cx.dummy(local);
        return;
    }

    let expr = if let Some(LocalInit { expr, .. }) = &mut local.init {
        &mut **expr
    } else {
        cx.error(format_err!(
            local,
            "the `#[auto_enum]` attribute is not supported uninitialized let statement"
        ));
        return;
    };

    expand_expr(cx, expr);

    cx.build(|item| build_expr(expr, item));
}

fn expand_parent_item_fn(cx: &mut Context, item: &mut ItemFn) {
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
                    }) if args.len() == 2
                        && path_eq(path, &["std", "core"], &["result", "Result"]) =>
                    {
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
        return;
    }

    match item.block.stmts.last_mut() {
        Some(Stmt::Expr(expr, None)) => child_expr(cx, expr),
        Some(_) => {}
        None => cx.error(format_err!(
            item.block,
            "the `#[auto_enum]` attribute is not supported empty functions"
        )),
    }

    #[cfg(feature = "type_analysis")]
    {
        if let VisitMode::Return(count) = cx.visit_mode {
            if cx.args.is_empty() && cx.variant_is_empty() && count < 2 {
                cx.dummy(item);
                return;
            }
        }
    }

    cx.visitor(item);

    cx.build(|i| item.block.stmts.insert(0, Stmt::Item(i.into())));
}
