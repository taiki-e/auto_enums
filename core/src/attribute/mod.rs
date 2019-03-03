use proc_macro2::TokenStream;
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
use self::expr::{child_expr, EMPTY_ATTRS, NEVER};
use self::params::*;
#[cfg(feature = "type_analysis")]
use self::traits::*;
use self::visitor::*;

/// The attribute name.
pub(crate) const NAME: &str = "auto_enum";

pub(crate) fn attribute(args: TokenStream, input: TokenStream) -> TokenStream {
    expand(args, input).unwrap_or_else(|e| e.to_compile_err())
}

fn expand(args: TokenStream, input: TokenStream) -> Result<TokenStream> {
    let mut params = parse_args(args)?;
    params.root();

    match syn::parse2::<Stmt>(input.clone()) {
        Ok(mut stmt) => stmt.visit_parent(params).map(|()| stmt.into_token_stream()),
        Err(_) => syn::parse2::<Expr>(input)
            .map_err(|_| "may only be used on expression, statement, or function".into())
            .and_then(|mut expr| expr.visit_parent(params).map(|()| expr.into_token_stream())),
    }
}

fn visit_expr(expr: &mut Expr, params: &mut Params) -> Result<()> {
    match expr {
        Expr::Closure(ExprClosure { body, .. }) if !params.never() => {
            let visit_try = find_try(params.marker(), |v| v.visit_expr_mut(body));
            if !visit_try {
                child_expr(&mut **body, params)?;
            }

            params._visitor(
                if visit_try {
                    VisitOption::Try
                } else {
                    VisitOption::Return
                },
                |v| v.visit_expr_mut(&mut **body),
            );
        }
        _ => {
            if !params.never() {
                child_expr(expr, params)?;
            }

            params.visitor(|v| v.visit_expr_mut(expr));
        }
    }

    Ok(())
}

fn build_expr(expr: &mut Expr, params: &Params) -> Result<()> {
    params.build().map(|item| {
        replace_expr(expr, |expr| {
            expr_block(block(vec![Stmt::Item(item.into()), Stmt::Expr(expr)]))
        });
    })
}

/// The statement or expression in which `#[auto_enum]` was directly used.
trait Parent {
    fn visit_parent(&mut self, params: Params) -> Result<()>;
}

impl Parent for Stmt {
    fn visit_parent(&mut self, params: Params) -> Result<()> {
        fn stmt_semi(expr: &mut Expr, mut params: Params) -> Result<()> {
            if params.args().is_empty() {
                Dummy.visit_expr_mut(expr);
                return Ok(());
            }

            params.visitor(|v| v.visit_expr_mut(expr));

            match params.builder().len() {
                len @ 0 | len @ 1 if !params.attr() => Err(unsupported_stmt(format!(
                    "expression with trailing semicolon is required two or more marker macros. There is {} marker macro in this statement.",
                    less_than_two(len),
                ))),
                0 => Ok(()),
                _ => build_expr(expr, &params),
            }
        }

        match self {
            Stmt::Expr(expr) => expr.visit_parent(params),
            Stmt::Semi(expr, _) => stmt_semi(expr, params),
            Stmt::Local(local) => local.visit_parent(params),
            Stmt::Item(Item::Fn(item)) => item.visit_parent(params),
            Stmt::Item(_) => Err("may only be used on expression, statement, or function".into()),
        }
    }
}

impl Parent for Expr {
    fn visit_parent(&mut self, mut params: Params) -> Result<()> {
        if params.args().is_empty() {
            Dummy.visit_expr_mut(self);
            return Ok(());
        }

        visit_expr(self, &mut params)?;

        match params.builder().len() {
            len @ 0 | len @ 1 if !params.attr() => Err(unsupported_expr(format!(
                "is required two or more branches or marker macros in total. There is {} branch or marker macro in this expression.",
                less_than_two(len),
            ))),
            0 => Ok(()),
            _ => build_expr(self, &params),
        }
    }
}

impl Parent for Local {
    fn visit_parent(&mut self, mut params: Params) -> Result<()> {
        #[cfg(feature = "type_analysis")]
        {
            if let Some((_, ty)) = &mut self.ty {
                params.impl_traits(&mut *ty);
            }
        }

        if params.args().is_empty() {
            Dummy.visit_local_mut(self);
            return Ok(());
        }

        let mut expr = (self.init)
            .take()
            .map(|(_, expr)| expr)
            .ok_or_else(|| unsupported_stmt("uninitialized let statement"))?;

        visit_expr(&mut *expr, &mut params)?;

        match params.builder().len() {
            len @ 0 | len @ 1 if !params.attr() => Err(unsupported_stmt(format!(
                "is required two or more branches or marker macros in total. There is {} branch or marker macro in this statement.",
                less_than_two(len),
            )))?,
            0 => {}
            _ => build_expr(&mut *expr, &params)?,
        }

        self.init = Some((default(), expr));
        Ok(())
    }
}

impl Parent for ItemFn {
    fn visit_parent(&mut self, mut params: Params) -> Result<()> {
        let mut option = VisitOption::Default;

        let ItemFn { decl, block, .. } = self;
        if let ReturnType::Type(_, ty) = &mut decl.output {
            match &**ty {
                // `return`
                Type::ImplTrait(_) if !params.never() => option = VisitOption::Return,

                // `?` operator
                Type::Path(TypePath { qself: None, path }) if !params.never() => {
                    let PathSegment { arguments, ident } = &path.segments[path.segments.len() - 1];
                    // `Result<T, impl Trait>`
                    match arguments {
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
                                if find_try(params.marker(), |v| v.visit_block_mut(&mut **block)) {
                                    option = VisitOption::Try;
                                }
                            }
                        }
                        _ => {}
                    }
                }

                _ => {}
            }

            #[cfg(feature = "type_analysis")]
            params.impl_traits(&mut *ty);
        }

        if params.args().is_empty() {
            Dummy.visit_item_fn_mut(self);
            return Ok(());
        }

        match self.block.stmts.last_mut() {
            Some(Stmt::Expr(expr)) if !params.never() && !option.is_try() => {
                child_expr(expr, &mut params)?
            }
            Some(_) => {}
            None => Err(unsupported_item("empty function"))?,
        }

        params._visitor(option, |v| v.visit_item_fn_mut(self));

        match params.builder().len() {
            len @ 0 | len @ 1 if !params.attr() => Err(unsupported_stmt(format!(
                "is required two or more branches or marker macros in total. There is {} branch or marker macro in this function.",
                less_than_two(len),
            ))),
            0 => Ok(()),
            _ => params
                .build()
                .map(|i| self.block.stmts.insert(0, Stmt::Item(i.into()))),
        }
    }
}

#[inline(never)]
fn less_than_two(n: usize) -> &'static str {
    assert!(n < 2);
    if n == 0 {
        "no"
    } else {
        "only one"
    }
}
