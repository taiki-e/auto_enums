use syn::{Arm, Attribute, Expr, Local};

use crate::utils::*;

pub(super) trait Attrs {
    fn attrs(&self) -> &[Attribute];

    fn any_attr<S: AsRef<str>>(&self, ident: S) -> bool {
        any_attr(self.attrs(), ident.as_ref(), false)
    }

    fn any_empty_attr<S: AsRef<str>>(&self, ident: S) -> bool {
        any_attr(self.attrs(), ident.as_ref(), true)
    }
}

fn any_attr(attrs: &[Attribute], ident: &str, require_empty: bool) -> bool {
    attrs
        .iter()
        .any(|attr| attr.path.is_ident(ident) && (!require_empty || attr.tts.is_empty()))
}

pub(super) trait AttrsMut: Attrs {
    fn attrs_mut<T, F: FnOnce(&mut Vec<Attribute>) -> T>(&mut self, f: F) -> T;

    fn find_remove_attr<S: AsRef<str>>(&mut self, ident: S) -> Option<Attribute> {
        self.attrs_mut(|attrs| find_remove_attr(attrs, ident.as_ref(), false))
    }

    fn find_remove_empty_attr<S: AsRef<str>>(&mut self, ident: S) -> bool {
        self.attrs_mut(|attrs| find_remove_attr(attrs, ident.as_ref(), true).is_some())
    }
}

fn find_remove_attr(
    attrs: &mut Vec<Attribute>,
    ident: &str,
    require_empty: bool,
) -> Option<Attribute> {
    attrs.find_remove(|attr| attr.path.is_ident(ident) && (!require_empty || attr.tts.is_empty()))
}

impl<'a, A: Attrs> Attrs for &'a A {
    fn attrs(&self) -> &[Attribute] {
        (**self).attrs()
    }
}
impl<'a, A: Attrs> Attrs for &'a mut A {
    fn attrs(&self) -> &[Attribute] {
        (**self).attrs()
    }
}
impl<'a, A: AttrsMut> AttrsMut for &'a mut A {
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
    ($($Expr:ident($Self:ident)),*) => {
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
    Box(ExprBox),
    InPlace(ExprInPlace),
    Array(ExprArray),
    Call(ExprCall),
    MethodCall(ExprMethodCall),
    Tuple(ExprTuple),
    Binary(ExprBinary),
    Unary(ExprUnary),
    Lit(ExprLit),
    Cast(ExprCast),
    Type(ExprType),
    Let(ExprLet),
    If(ExprIf),
    While(ExprWhile),
    ForLoop(ExprForLoop),
    Loop(ExprLoop),
    Match(ExprMatch),
    Closure(ExprClosure),
    Unsafe(ExprUnsafe),
    Block(ExprBlock),
    Assign(ExprAssign),
    AssignOp(ExprAssignOp),
    Field(ExprField),
    Index(ExprIndex),
    Range(ExprRange),
    Path(ExprPath),
    Reference(ExprReference),
    Break(ExprBreak),
    Continue(ExprContinue),
    Return(ExprReturn),
    Macro(ExprMacro),
    Struct(ExprStruct),
    Repeat(ExprRepeat),
    Paren(ExprParen),
    Group(ExprGroup),
    Try(ExprTry),
    Async(ExprAsync),
    TryBlock(ExprTryBlock),
    Yield(ExprYield)
}
