use std::cell::RefCell;

use proc_macro2::Ident;
use quote::quote;
use rand::{rngs::ThreadRng, Rng};
use smallvec::{smallvec, SmallVec};
use syn::{Attribute, Expr, ExprCall, ExprPath, ItemEnum, Path};

use crate::utils::{Result, *};

use super::Arg;

thread_local! {
    static RNG: RefCell<ThreadRng> = RefCell::new(rand::thread_rng());
}

pub(super) type Builder = EnumBuilder;

struct EnumVariant {
    ident: String,
}

pub(super) struct EnumBuilder {
    ident: String,
    variants: Stack<EnumVariant>,
}

impl EnumVariant {
    fn new(id: usize) -> Self {
        Self {
            ident: format!("__T{}", id),
        }
    }

    fn ident(&self) -> Ident {
        ident_call_site(&self.ident)
    }

    fn path(&self, ident: &str) -> Path {
        let segments: SmallVec<[_; 2]> =
            smallvec![ident_call_site(ident).into(), self.ident().into()];
        path(segments)
    }

    fn expr(&self, ident: &str, attrs: Vec<Attribute>, expr: Expr) -> Expr {
        Expr::Call(ExprCall {
            attrs,
            func: Box::new(Expr::Path(ExprPath {
                attrs: Vec::with_capacity(0),
                qself: None,
                path: self.path(ident),
            })),
            paren_token: default(),
            args: Some(expr).into_iter().collect(),
        })
    }
}

impl EnumBuilder {
    pub(super) fn new() -> Self {
        Self {
            ident: format!("__Enum{}", RNG.with(|rng| rng.borrow_mut().gen::<u32>())),
            variants: Stack::new(),
        }
    }

    pub(super) fn ident(&self) -> &str {
        &self.ident
    }

    pub(super) fn len(&self) -> usize {
        self.variants.len()
    }

    fn iter(&self) -> impl Iterator<Item = Ident> + '_ {
        self.variants.iter().map(|v| v.ident())
    }

    fn push_variant(&mut self) {
        let field = EnumVariant::new(self.len());
        self.variants.push(field);
    }

    fn get_expr(&self, index: usize, attrs: Vec<Attribute>, expr: Expr) -> Expr {
        self.variants[index].expr(self.ident(), attrs, expr)
    }

    pub(super) fn next_expr(&mut self, expr: Expr) -> Expr {
        self.next_expr_with_attrs(Vec::with_capacity(0), expr)
    }

    pub(super) fn next_expr_with_attrs(&mut self, attrs: Vec<Attribute>, expr: Expr) -> Expr {
        self.push_variant();
        self.get_expr(self.len() - 1, attrs, expr)
    }

    pub(super) fn build(&self, args: &[Arg]) -> Result<ItemEnum> {
        assert!(self.len() >= 2);

        let ident = ident_call_site(self.ident());
        let ty_generics = self.iter();
        let variants = self.iter();
        let fields = self.iter();

        syn::parse2(quote! {
            #[enum_derive(#(#args),*)]
            enum #ident<#(#ty_generics),*> {
                #(#variants(#fields),)*
            }
        })
        .map_err(|_| "failed generate an enum".into())
    }
}
