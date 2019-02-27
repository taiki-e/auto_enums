use syn::{Arm, Attribute, Expr, Local};

use crate::utils::*;

pub(super) trait Attrs {
    fn attrs(&self) -> &[Attribute];

    fn any_attr(&self, ident: &str) -> bool {
        self.attrs().iter().any(|attr| attr.path.is_ident(ident))
    }

    fn any_empty_attr(&self, ident: &str) -> bool {
        self.attrs()
            .iter()
            .any(|Attribute { path, tts, .. }| path.is_ident(ident) && tts.is_empty())
    }
}

pub(super) trait AttrsMut: Attrs {
    fn attrs_mut<T, F: FnOnce(&mut Vec<Attribute>) -> T>(&mut self, f: F) -> T;

    fn find_remove_attr(&mut self, ident: &str) -> Option<Attribute> {
        self.attrs_mut(|attrs| attrs.find_remove(|attr| attr.path.is_ident(ident)))
    }

    fn find_remove_empty_attr(&mut self, ident: &str) -> bool {
        fn find_remove(attrs: &mut Vec<Attribute>, ident: &str) -> Option<Attribute> {
            attrs.find_remove(|Attribute { path, tts, .. }| path.is_ident(ident) && tts.is_empty())
        }

        self.attrs_mut(|attrs| find_remove(attrs, ident).is_some())
    }
}

impl<A: Attrs> Attrs for &'_ A {
    fn attrs(&self) -> &[Attribute] {
        (**self).attrs()
    }
}
impl<A: Attrs> Attrs for &'_ mut A {
    fn attrs(&self) -> &[Attribute] {
        (**self).attrs()
    }
}
impl<A: AttrsMut> AttrsMut for &'_ mut A {
    fn attrs_mut<T, F: FnOnce(&mut Vec<Attribute>) -> T>(&mut self, f: F) -> T {
        (**self).attrs_mut(f)
    }
}

impl Attrs for [Attribute] {
    fn attrs(&self) -> &[Attribute] {
        self
    }
}
impl Attrs for Vec<Attribute> {
    fn attrs(&self) -> &[Attribute] {
        self
    }
}
impl AttrsMut for Vec<Attribute> {
    fn attrs_mut<T, F: FnOnce(&mut Vec<Attribute>) -> T>(&mut self, f: F) -> T {
        f(self)
    }
}

impl Attrs for Local {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}
impl AttrsMut for Local {
    fn attrs_mut<T, F: FnOnce(&mut Vec<Attribute>) -> T>(&mut self, f: F) -> T {
        f(&mut self.attrs)
    }
}
impl Attrs for Arm {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}
impl AttrsMut for Arm {
    fn attrs_mut<T, F: FnOnce(&mut Vec<Attribute>) -> T>(&mut self, f: F) -> T {
        f(&mut self.attrs)
    }
}

macro_rules! attrs_impl {
    ($($Expr:ident),*) => {
        impl Attrs for Expr {
            fn attrs(&self) -> &[Attribute] {
                match self {
                    $(Expr::$Expr(expr) => &expr.attrs,)*
                    Expr::Verbatim(_) => &[],
                }
            }
        }
        impl AttrsMut for Expr {
            fn attrs_mut<T, F: FnOnce(&mut Vec<Attribute>) -> T>(&mut self, f: F) -> T {
                match self {
                    $(Expr::$Expr(expr) => f(&mut expr.attrs),)*
                    Expr::Verbatim(_) => f(&mut Vec::with_capacity(0)),
                }
            }
        }
    };
}

attrs_impl! {
    Box,
    InPlace,
    Array,
    Call,
    MethodCall,
    Tuple,
    Binary,
    Unary,
    Lit,
    Cast,
    Type,
    Let,
    If,
    While,
    ForLoop,
    Loop,
    Match,
    Closure,
    Unsafe,
    Block,
    Assign,
    AssignOp,
    Field,
    Index,
    Range,
    Path,
    Reference,
    Break,
    Continue,
    Return,
    Macro,
    Struct,
    Repeat,
    Paren,
    Group,
    Try,
    Async,
    TryBlock,
    Yield
}
