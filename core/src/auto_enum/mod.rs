use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    visit_mut::VisitMut, AngleBracketedGenericArguments, Expr, ExprClosure, GenericArgument, Item,
    ItemFn, Local, PathArguments, PathSegment, ReturnType, Stmt, Type, TypePath,
};

use crate::utils::{Result, *};

mod args;
mod attrs;
mod context;
mod expr;
mod visitor;

use self::args::{parse_args, Arg};
use self::attrs::{Attrs, AttrsMut};
use self::context::*;
use self::expr::child_expr;
use self::visitor::{Dummy, FindTry, Visitor};

#[cfg(feature = "type_analysis")]
mod traits;
#[cfg(feature = "type_analysis")]
use self::traits::*;

/// The attribute name.
const NAME: &str = "auto_enum";
/// The annotation for recursively parsing.
const REC: &str = "rec";
/// The annotation for skipping branch.
const NEVER: &str = "never";
/// The annotations used by `#[auto_enum]`.
const EMPTY_ATTRS: &[&str] = &[NEVER, REC];

pub(crate) fn attribute(args: TokenStream, input: TokenStream) -> TokenStream {
    expand(args, input).unwrap_or_else(|e| e.to_compile_error())
}

fn expand(args: TokenStream, input: TokenStream) -> Result<TokenStream> {
    let cx = parse_args(args).map(|(args, marker, never)| Context::root(args, marker, never))?;

    let span = span!(input);
    match syn::parse2::<Stmt>(input.clone()) {
        Ok(mut stmt) => stmt.visit_parent(cx).map(|()| stmt.into_token_stream()),
        Err(_) => syn::parse2::<Expr>(input)
            .map_err(|_| "may only be used on expression, statement, or function".into())
            .and_then(|mut expr| expr.visit_parent(cx).map(|()| expr.into_token_stream())),
    }
    .map_err(|e| e.set_span(span))
}

fn visit_expr(expr: &mut Expr, cx: &mut Context) -> Result<()> {
    let expr = match expr {
        Expr::Closure(ExprClosure { body, .. }) if cx.visit_last() => {
            cx.visit_mode(VisitMode::Return);
            cx.visit_last_mode(VisitLastMode::Closure);
            cx.find_try(|v| v.visit_expr_mut(body));
            &mut **body
        }
        _ => expr,
    };

    child_expr(expr, cx).map(|()| cx.visitor(|v| v.visit_expr_mut(expr)))
}

fn build_expr(expr: &mut Expr, cx: &Context) {
    replace_expr(expr, |expr| {
        expr_block(block(vec![Stmt::Item(cx.build().into()), Stmt::Expr(expr)]))
    });
}

/// The statement or expression in which `#[auto_enum]` was directly used.
trait Parent {
    fn visit_parent(&mut self, cx: Context) -> Result<()>;
}

impl Parent for Stmt {
    fn visit_parent(&mut self, mut cx: Context) -> Result<()> {
        if let Stmt::Semi(_, _) = &self {
            cx.visit_last_mode(VisitLastMode::Never);
        }

        match self {
            Stmt::Expr(expr) | Stmt::Semi(expr, _) => expr.visit_parent(cx),
            Stmt::Local(local) => local.visit_parent(cx),
            Stmt::Item(Item::Fn(item)) => item.visit_parent(cx),
            Stmt::Item(_) => Err("may only be used on expression, statement, or function".into()),
        }
    }
}

impl Parent for Expr {
    fn visit_parent(&mut self, mut cx: Context) -> Result<()> {
        if cx.args.is_empty() {
            cx.dummy(|v| v.visit_expr_mut(self));
            return Ok(());
        }

        visit_expr(self, &mut cx)?;

        if cx.buildable()? {
            build_expr(self, &cx);
        }
        Ok(())
    }
}

impl Parent for Local {
    fn visit_parent(&mut self, mut cx: Context) -> Result<()> {
        #[cfg(feature = "type_analysis")]
        {
            if let Some((_, ty)) = &mut self.ty {
                cx.impl_traits(&mut *ty);
            }
        }

        if cx.args.is_empty() {
            cx.dummy(|v| v.visit_local_mut(self));
            return Ok(());
        }

        let mut expr = self.init.take().map(|(_, expr)| expr).ok_or_else(|| {
            err!(
                self,
                "the `#[auto_enum]` attribute is not supported uninitialized let statement"
            )
        })?;

        visit_expr(&mut *expr, &mut cx)?;

        if cx.buildable()? {
            build_expr(&mut *expr, &cx);
        }

        self.init = Some((default(), expr));
        Ok(())
    }
}

impl Parent for ItemFn {
    fn visit_parent(&mut self, mut cx: Context) -> Result<()> {
        let Self { decl, block, .. } = self;
        if let ReturnType::Type(_, ty) = &mut decl.output {
            match &**ty {
                // `return`
                Type::ImplTrait(_) if cx.visit_last() => cx.visit_mode(VisitMode::Return),

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
            cx.impl_traits(&mut *ty);
        }

        if cx.args.is_empty() {
            cx.dummy(|v| v.visit_item_fn_mut(self));
            return Ok(());
        }

        match self.block.stmts.last_mut() {
            Some(Stmt::Expr(expr)) => child_expr(expr, &mut cx)?,
            Some(_) => {}
            None => Err(err!(
                self.block,
                "the `#[auto_enum]` attribute is not supported empty functions"
            ))?,
        }

        cx.visitor(|v| v.visit_item_fn_mut(self));

        if cx.buildable()? {
            self.block.stmts.insert(0, Stmt::Item(cx.build().into()));
        }
        Ok(())
    }
}
