use syn::{Arm, Attribute, Expr, Local, Stmt};

use super::VecExt;

pub(crate) trait Attrs {
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

pub(crate) trait AttrsMut: Attrs {
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

impl Attrs for Stmt {
    fn attrs(&self) -> &[Attribute] {
        match self {
            Stmt::Expr(expr) | Stmt::Semi(expr, _) => expr.attrs(),
            Stmt::Local(local) => local.attrs(),
            // Stop at item bounds
            Stmt::Item(_) => &[],
        }
    }
}

impl AttrsMut for Stmt {
    fn attrs_mut<T, F: FnOnce(&mut Vec<Attribute>) -> T>(&mut self, f: F) -> T {
        match self {
            Stmt::Expr(expr) | Stmt::Semi(expr, _) => expr.attrs_mut(f),
            Stmt::Local(local) => local.attrs_mut(f),
            // Stop at item bounds
            Stmt::Item(_) => f(&mut Vec::new()),
        }
    }
}

macro_rules! attrs_impl {
    ($($Expr:ident,)*) => {
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
                    Expr::Verbatim(_) => f(&mut Vec::new()),
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
    Yield,
}
