use syn::*;

use super::VecExt;

pub(crate) trait Attrs {
    fn attrs(&self) -> &[Attribute];

    fn any_attr(&self, ident: &str) -> bool {
        self.attrs().iter().any(|attr| attr.path.is_ident(ident))
    }

    fn any_empty_attr(&self, ident: &str) -> bool {
        self.attrs().iter().any(|attr| attr.path.is_ident(ident) && attr.tokens.is_empty())
    }
}

pub(crate) trait AttrsMut: Attrs {
    fn attrs_mut<T, F: FnOnce(&mut Vec<Attribute>) -> T>(&mut self, f: F) -> T;

    fn find_remove_attr(&mut self, ident: &str) -> Option<Attribute> {
        self.attrs_mut(|attrs| attrs.find_remove(|attr| attr.path.is_ident(ident)))
    }

    fn find_remove_empty_attr(&mut self, ident: &str) -> bool {
        fn find_remove(attrs: &mut Vec<Attribute>, ident: &str) -> Option<Attribute> {
            attrs.find_remove(|attr| attr.path.is_ident(ident) && attr.tokens.is_empty())
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
    ($($Expr:ident($Struct:ident),)*) => {
        impl Attrs for Expr {
            fn attrs(&self) -> &[Attribute] {
                match self {
                    $(Expr::$Expr($Struct { attrs, .. }))|* => &attrs,
                    _ => &[],
                }
            }
        }

        impl AttrsMut for Expr {
            fn attrs_mut<T, F: FnOnce(&mut Vec<Attribute>) -> T>(&mut self, f: F) -> T {
                match self {
                    $(Expr::$Expr($Struct { attrs, .. }))|* => f(attrs),
                    _ => f(&mut Vec::new()),
                }
            }
        }
    };
}

attrs_impl! {
    Array(ExprArray),
    Assign(ExprAssign),
    AssignOp(ExprAssignOp),
    Async(ExprAsync),
    Await(ExprAwait),
    Binary(ExprBinary),
    Block(ExprBlock),
    Box(ExprBox),
    Break(ExprBreak),
    Call(ExprCall),
    Cast(ExprCast),
    Closure(ExprClosure),
    Continue(ExprContinue),
    Field(ExprField),
    ForLoop(ExprForLoop),
    Group(ExprGroup),
    If(ExprIf),
    Index(ExprIndex),
    Let(ExprLet),
    Lit(ExprLit),
    Loop(ExprLoop),
    Macro(ExprMacro),
    Match(ExprMatch),
    MethodCall(ExprMethodCall),
    Paren(ExprParen),
    Path(ExprPath),
    Range(ExprRange),
    Reference(ExprReference),
    Repeat(ExprRepeat),
    Return(ExprReturn),
    Struct(ExprStruct),
    Try(ExprTry),
    TryBlock(ExprTryBlock),
    Tuple(ExprTuple),
    Type(ExprType),
    Unary(ExprUnary),
    Unsafe(ExprUnsafe),
    While(ExprWhile),
    Yield(ExprYield),
}
