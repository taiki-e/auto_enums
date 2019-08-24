use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{visit_mut::VisitMut, *};

use crate::utils::*;

mod context;
mod expr;
#[cfg(feature = "type_analysis")]
mod traits;
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
        Ok(mut stmt) => stmt.expand_parent(&mut cx).map(|()| stmt.into_token_stream()),
        Err(e) => syn::parse2::<Expr>(input)
            .map_err(|_e| {
                cx.diagnostic.error(e);
                error!(cx.span, "the `#[auto_enum]` attribute may only be used on expression, statement, or function")
            })
            .and_then(|mut expr| expr.expand_parent(&mut cx).map(|()| expr.into_token_stream())),
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

/// The statement or expression in which `#[auto_enum]` was directly used.
trait Parent {
    fn expand_parent(&mut self, cx: &mut Context) -> Result<()>;
}

impl Parent for Stmt {
    fn expand_parent(&mut self, cx: &mut Context) -> Result<()> {
        if let Stmt::Semi(..) = &self {
            cx.visit_last_mode = VisitLastMode::Never;
        }

        match self {
            Stmt::Expr(expr) | Stmt::Semi(expr, _) => expr.expand_parent(cx),
            Stmt::Local(local) => local.expand_parent(cx),
            Stmt::Item(Item::Fn(item)) => item.expand_parent(cx),
            Stmt::Item(item) => {
                Err(error!(item, "may only be used on expression, statement, or function"))
            }
        }
    }
}

impl Parent for Expr {
    fn expand_parent(&mut self, cx: &mut Context) -> Result<()> {
        if cx.is_dummy() {
            cx.dummy(|v| v.visit_expr_mut(self));
            return Ok(());
        }

        visit_expr(self, cx)?;

        cx.build(|item| build_expr(self, item))
    }
}

impl Parent for Local {
    fn expand_parent(&mut self, cx: &mut Context) -> Result<()> {
        #[cfg(feature = "type_analysis")]
        {
            if let Pat::Type(PatType { ty, .. }) = &mut self.pat {
                cx.collect_trait(&mut *ty);
            }
        }

        if cx.is_dummy() {
            cx.dummy(|v| v.visit_local_mut(self));
            return Ok(());
        }

        let expr = match self.init.as_mut().map(|(_, expr)| &mut **expr) {
            Some(expr) => expr,
            None => {
                return Err(error!(
                    self,
                    "the `#[auto_enum]` attribute is not supported uninitialized let statement"
                ))
            }
        };

        visit_expr(expr, cx)?;

        cx.build(|item| build_expr(expr, item))
    }
}

impl Parent for ItemFn {
    fn expand_parent(&mut self, cx: &mut Context) -> Result<()> {
        let Self { sig, block, .. } = self;
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
            cx.dummy(|v| v.visit_item_fn_mut(self));
            return Ok(());
        }

        match self.block.stmts.last_mut() {
            Some(Stmt::Expr(expr)) => child_expr(expr, cx)?,
            Some(_) => {}
            None => {
                return Err(error!(
                    self.block,
                    "the `#[auto_enum]` attribute is not supported empty functions"
                ))
            }
        }

        cx.visitor(|v| v.visit_item_fn_mut(self));

        cx.build(|item| self.block.stmts.insert(0, Stmt::Item(item.into())))
    }
}
