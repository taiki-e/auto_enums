use std::cell::RefCell;

use proc_macro2::Ident;
use quote::quote;
use rand::{rngs::ThreadRng, Rng};
use smallvec::{smallvec, SmallVec};
use syn::*;

use crate::utils::{Result, *};

use super::*;

const DERIVE_ATTR: &str = "enum_derive";

thread_local! {
    static RNG: RefCell<ThreadRng> = RefCell::new(rand::thread_rng());
}

pub(super) type Builder = EnumBuilder;

struct EnumVariant(String);

pub(super) struct EnumBuilder {
    ident: String,
    variants: Stack<EnumVariant>,
    next: usize,
}

impl EnumVariant {
    fn new(id: usize) -> Self {
        EnumVariant(format!("__T{}", id))
    }

    fn ident(&self) -> Ident {
        ident_call_site(&self.0)
    }

    fn path(&self, ident: &str) -> Path {
        let segments: SmallVec<[_; 2]> =
            smallvec![ident_call_site(ident).into(), self.ident().into()];
        path(segments.into_iter())
    }

    fn expr_call(&self, ident: &str, attrs: Vec<Attribute>, expr: Expr) -> Expr {
        expr_call(attrs, self.path(ident), expr)
    }
}

impl EnumBuilder {
    pub(super) fn new() -> Self {
        EnumBuilder {
            ident: format!("__E{}", RNG.with(|rng| rng.borrow_mut().gen::<u32>())),
            variants: Stack::new(),
            next: 0,
        }
    }

    fn len(&self) -> usize {
        self.variants.len()
    }

    pub(super) fn is_empty(&self) -> bool {
        self.variants.is_empty()
    }

    fn iter(&self) -> impl Iterator<Item = Ident> + '_ {
        self.variants.iter().map(|f| f.ident())
    }

    fn push_variant(&mut self) {
        let field = EnumVariant::new(self.len());
        self.variants.push(field);
    }

    pub(super) fn reserve(&mut self, additional: usize) {
        (0..additional).for_each(|_| self.push_variant());
    }

    fn get_expr_call(&self, index: usize, attrs: Vec<Attribute>, expr: Expr) -> Expr {
        self.variants[index].expr_call(&self.ident, attrs, expr)
    }

    pub(super) fn next_expr_call(&mut self, attrs: Vec<Attribute>, expr: Expr) -> Expr {
        assert!(self.next <= self.len());

        if self.next == self.len() {
            self.push_variant();
        }

        let expr_call = self.get_expr_call(self.next, attrs, expr);
        self.next += 1;
        expr_call
    }

    pub(super) fn build(&self, args: &[Arg]) -> Result<ItemEnum> {
        if self.len() < 2 {
            Err("macro is required two or more branches or marker macros in total")?;
        }

        let attr = ident_call_site(DERIVE_ATTR);
        let ident = ident_call_site(&self.ident);
        let ty_generics = self.iter();
        let variants = self.iter();
        let fields = self.iter();

        syn::parse2(quote! {
            #[#attr(#(#args),*)]
            enum #ident<#(#ty_generics),*> {
                #(#variants(#fields),)*
            }
        })
        .map_err(|_| "failed generate an enum".into())
    }
}
