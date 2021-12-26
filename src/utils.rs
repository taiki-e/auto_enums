use std::{iter, mem};

use proc_macro2::TokenStream;
use syn::{
    punctuated::Punctuated, token, visit_mut::VisitMut, Arm, Attribute, Block, Expr, ExprBlock,
    ExprCall, ExprPath, ExprTuple, ItemFn, Local, Path, PathSegment, Result, Stmt,
};

macro_rules! format_err {
    ($span:expr, $msg:expr $(,)*) => {
        syn::Error::new_spanned(&$span as &dyn quote::ToTokens, &$msg as &dyn ::std::fmt::Display)
    };
    ($span:expr, $($tt:tt)*) => {
        format_err!($span, format!($($tt)*))
    };
}

macro_rules! bail {
    ($($tt:tt)*) => {
        return Err(format_err!($($tt)*))
    };
}

pub(crate) fn path(segments: impl IntoIterator<Item = PathSegment>) -> Path {
    Path { leading_colon: None, segments: segments.into_iter().collect() }
}

pub(crate) fn block(stmts: Vec<Stmt>) -> Block {
    Block { brace_token: token::Brace::default(), stmts }
}

pub(crate) fn expr_block(block: Block) -> Expr {
    Expr::Block(ExprBlock { attrs: Vec::new(), label: None, block })
}

pub(crate) fn expr_call(attrs: Vec<Attribute>, path: Path, arg: Expr) -> Expr {
    Expr::Call(ExprCall {
        attrs,
        func: Box::new(Expr::Path(ExprPath { attrs: Vec::new(), qself: None, path })),
        paren_token: token::Paren::default(),
        args: iter::once(arg).collect(),
    })
}

pub(crate) fn unit() -> Expr {
    Expr::Tuple(ExprTuple {
        attrs: Vec::new(),
        paren_token: token::Paren::default(),
        elems: Punctuated::new(),
    })
}

pub(crate) fn replace_expr(this: &mut Expr, f: impl FnOnce(Expr) -> Expr) {
    *this = f(mem::replace(this, Expr::Verbatim(TokenStream::new())));
}

pub(crate) fn replace_block(this: &mut Block, f: impl FnOnce(Block) -> Expr) {
    // `brace_token` of the block that passed to `f` should have `call_site` span.
    // If `f` generates unused braces containing the span of `this.brace_token`,
    // this will cause confusing warnings: https://github.com/rust-lang/rust/issues/71080
    let stmts = mem::replace(&mut this.stmts, Vec::new());
    this.stmts = vec![Stmt::Expr(f(block(stmts)))];
}

/// Checks if `tokens` is an empty `TokenStream`.
///
/// This is almost equivalent to `syn::parse2::<Nothing>()`, but produces
/// a better error message and does not require ownership of `tokens`.
pub(crate) fn parse_as_empty(tokens: &TokenStream) -> Result<()> {
    if tokens.is_empty() {
        Ok(())
    } else {
        bail!(tokens, "unexpected token: `{}`", tokens)
    }
}

// =================================================================================================
// extension traits

pub(crate) trait VecExt<T> {
    fn find_remove(&mut self, predicate: impl FnMut(&T) -> bool) -> Option<T>;
}

impl<T> VecExt<T> for Vec<T> {
    fn find_remove(&mut self, predicate: impl FnMut(&T) -> bool) -> Option<T> {
        self.iter().position(predicate).map(|i| self.remove(i))
    }
}

// =================================================================================================
// visited node

pub(crate) trait Node {
    fn visited(&mut self, visitor: &mut impl VisitMut);
}

impl Node for Stmt {
    fn visited(&mut self, visitor: &mut impl VisitMut) {
        visitor.visit_stmt_mut(self);
    }
}

impl Node for Local {
    fn visited(&mut self, visitor: &mut impl VisitMut) {
        visitor.visit_local_mut(self);
    }
}

impl Node for Expr {
    fn visited(&mut self, visitor: &mut impl VisitMut) {
        visitor.visit_expr_mut(self);
    }
}

impl Node for Arm {
    fn visited(&mut self, visitor: &mut impl VisitMut) {
        visitor.visit_arm_mut(self);
    }
}

impl Node for Block {
    fn visited(&mut self, visitor: &mut impl VisitMut) {
        visitor.visit_block_mut(self);
    }
}

impl Node for ItemFn {
    fn visited(&mut self, visitor: &mut impl VisitMut) {
        visitor.visit_item_fn_mut(self);
    }
}

// =================================================================================================
// helper for handling attributes

pub(crate) trait Attrs {
    fn attrs(&self) -> &[Attribute];

    fn any_attr(&self, ident: &str) -> bool {
        self.attrs().iter().any(|attr| attr.path.is_ident(ident))
    }

    fn any_empty_attr(&self, ident: &str) -> bool {
        self.attrs().iter().any(|attr| attr.path.is_ident(ident) && attr.tokens.is_empty())
    }

    fn attrs_mut(&mut self) -> Option<&mut Vec<Attribute>>;

    fn find_remove_attr(&mut self, ident: &str) -> Option<Attribute> {
        self.attrs_mut()?.find_remove(|attr| attr.path.is_ident(ident))
    }
}

impl Attrs for Arm {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }

    fn attrs_mut(&mut self) -> Option<&mut Vec<Attribute>> {
        Some(&mut self.attrs)
    }
}

impl Attrs for Local {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }

    fn attrs_mut(&mut self) -> Option<&mut Vec<Attribute>> {
        Some(&mut self.attrs)
    }
}

impl Attrs for Stmt {
    fn attrs(&self) -> &[Attribute] {
        match self {
            Stmt::Expr(expr) | Stmt::Semi(expr, _) => expr.attrs(),
            Stmt::Local(local) => local.attrs(),
            // Ignore nested items.
            Stmt::Item(_) => &[],
        }
    }

    fn attrs_mut(&mut self) -> Option<&mut Vec<Attribute>> {
        match self {
            Stmt::Expr(expr) | Stmt::Semi(expr, _) => expr.attrs_mut(),
            Stmt::Local(local) => local.attrs_mut(),
            // Ignore nested items.
            Stmt::Item(_) => None,
        }
    }
}

macro_rules! attrs_impl {
    ($($Expr:ident($Struct:ident),)*) => {
        impl Attrs for Expr {
            fn attrs(&self) -> &[Attribute] {
                match self {
                    $(Expr::$Expr(syn::$Struct { attrs, .. }))|* => &attrs,
                    _ => &[],
                }
            }

            fn attrs_mut(&mut self) -> Option<&mut Vec<Attribute>> {
                match self {
                    $(Expr::$Expr(syn::$Struct { attrs, .. }))|* => Some(attrs),
                    _ => None,
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
