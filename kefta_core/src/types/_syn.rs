use std::any::type_name;
use std::fmt::Formatter;
use std::marker::PhantomData;
use proc_macro2::Span;
use quote::__private::ext::RepToTokensExt;
use syn;
use syn::{Expr, ExprPath};
use crate::model::{FromMeta, MetaDomain, MetaError, MetaSource, MetaVisitor};

impl FromMeta<Expr> for Expr {
    fn from_meta<S>(source: S) -> Result<Self, S::Error> where S: MetaSource<Expr> {
        struct _Visitor;

        impl MetaVisitor for _Visitor {
            type Output = Expr;
            type Domain = Expr;

            fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
                f.write_str("valid expression")
            }

            fn visit_value<E>(self, _span: Option<Span>, value: Self::Domain) -> Result<Self::Output, E> where E: MetaError {
                Ok(value)
            }
        }

        source.visit(_Visitor)
    }
}


macro_rules! impl_syn_pat {
    (
        $(
            for $typ:ty : $exp:literal $pat:pat => $expr:expr
        ),*
    ) => {
        $(
            impl FromMeta<Expr> for $typ {
                fn from_meta<S>(source: S) -> Result<Self, S::Error> where S: MetaSource<Expr> {

                    struct _Visitor;

                    impl MetaVisitor for _Visitor {
                        type Output = $typ;
                        type Domain = Expr;

                        fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
                            f.write_str($exp)
                        }

                        fn visit_value<E>(self, span: Option<Span>, value: Self::Domain) -> Result<Self::Output, E> where E: MetaError {
                            match value {
                                $pat => Ok($expr),
                                expr => Err(E::expecting(span, self, format_args!("{:?}", expr)))
                            }
                        }
                    }

                    source.visit(_Visitor)
                }
            }
        )*
    };
}

impl_syn_pat! {
    for syn::ExprArray: "a ExprArray" Expr::Array(x) => x,
    for syn::ExprAssign: "a ExprAssign" Expr::Assign(x) => x,
    for syn::ExprAsync: "a ExprAsync" Expr::Async(x) => x,
    for syn::ExprAwait: "a ExprAwait" Expr::Await(x) => x,
    for syn::ExprBinary: "a ExprBinary" Expr::Binary(x) => x,
    for syn::ExprBlock: "a ExprBlock" Expr::Block(x) => x,
    for syn::ExprBreak: "a ExprBreak" Expr::Break(x) => x,
    for syn::ExprCall: "a ExprCall" Expr::Call(x) => x,
    for syn::ExprCast: "a ExprCast" Expr::Cast(x) => x,
    for syn::ExprClosure: "a ExprClosure" Expr::Closure(x) => x,
    for syn::ExprConst: "a ExprConst" Expr::Const(x) => x,
    for syn::ExprContinue: "a ExprContinue" Expr::Continue(x) => x,
    for syn::ExprField: "a ExprField" Expr::Field(x) => x,
    for syn::ExprForLoop: "a ExprForLoop" Expr::ForLoop(x) => x,
    for syn::ExprGroup: "a ExprGroup" Expr::Group(x) => x,
    for syn::ExprIf: "a ExprIf" Expr::If(x) => x,
    for syn::ExprIndex: "a ExprIndex" Expr::Index(x) => x,
    for syn::ExprInfer: "a ExprInfer" Expr::Infer(x) => x,
    for syn::ExprLet: "a ExprLet" Expr::Let(x) => x,
    for syn::ExprLit: "a ExprLit" Expr::Lit(x) => x,
    for syn::ExprLoop: "a ExprLoop" Expr::Loop(x) => x,
    for syn::ExprMacro: "a ExprMacro" Expr::Macro(x) => x,
    for syn::ExprMatch: "a ExprMatch" Expr::Match(x) => x,
    for syn::ExprMethodCall: "a ExprMethodCall" Expr::MethodCall(x) => x,
    for syn::ExprParen: "a ExprParen" Expr::Paren(x) => x,
    for syn::ExprPath: "a ExprPath" Expr::Path(x) => x,
    for syn::ExprRange: "a ExprRange" Expr::Range(x) => x,
    for syn::ExprReference: "a ExprReference" Expr::Reference(x) => x,
    for syn::ExprRepeat: "a ExprRepeat" Expr::Repeat(x) => x,
    for syn::ExprReturn: "a ExprReturn" Expr::Return(x) => x,
    for syn::ExprStruct: "a ExprStruct" Expr::Struct(x) => x,
    for syn::ExprTry: "a ExprTry" Expr::Try(x) => x,
    for syn::ExprTryBlock: "a ExprTryBlock" Expr::TryBlock(x) => x,
    for syn::ExprTuple: "a ExprTuple" Expr::Tuple(x) => x,
    for syn::ExprUnary: "a ExprUnary" Expr::Unary(x) => x,
    for syn::ExprUnsafe: "a ExprUnsafe" Expr::Unsafe(x) => x,
    for syn::ExprWhile: "a ExprWhile" Expr::While(x) => x,
    for syn::ExprYield: "a ExprYield" Expr::Yield(x) => x,

    for syn::Path: "a Path" Expr::Path(ExprPath { path, .. }) => path,
    for syn::Lit: "a Literal" Expr::Lit(syn::ExprLit { lit, .. }) => lit,

    for syn::LitStr: "a string literal" Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(expr), ..}) => expr,
	for syn::LitByteStr: "a byte-string literal" Expr::Lit(syn::ExprLit { lit: syn::Lit::ByteStr(expr), ..}) => expr,
	for syn::LitByte: "a byte literal" Expr::Lit(syn::ExprLit { lit: syn::Lit::Byte(expr), ..}) => expr,
	for syn::LitChar: "a char literal" Expr::Lit(syn::ExprLit { lit: syn::Lit::Char(expr), ..}) => expr,
	for syn::LitInt: "a int literal" Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(expr), ..}) => expr,
	for syn::LitFloat: "a float literal" Expr::Lit(syn::ExprLit { lit: syn::Lit::Float(expr), ..}) => expr,
	for syn::LitBool: "a bool literal" Expr::Lit(syn::ExprLit { lit: syn::Lit::Bool(expr), ..}) => expr,

    for String: "a string" Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(expr), ..}) => expr.value(),
    for char: "a char" Expr::Lit(syn::ExprLit { lit: syn::Lit::Char(expr), ..}) => expr.value()
}

macro_rules! impl_syn_pat_num {
    (
        $(
            for $(-$x:tt)? $typ:ty : $pat:pat => $expr:expr
        ),*
    ) => {
        $(
            impl FromMeta<Expr> for $typ {
                fn from_meta<S>(source: S) -> Result<Self, S::Error> where S: MetaSource<Expr> {

                    struct _Visitor;

                    impl MetaVisitor for _Visitor {
                        type Output = $typ;
                        type Domain = Expr;

                        fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
                            f.write_str(concat!("a ", stringify!($typ)))
                        }

                        fn visit_value<E>(self, span: Option<Span>, value: Self::Domain) -> Result<Self::Output, E> where E: MetaError {
                            match value {
                                 Expr::Lit(syn::ExprLit { lit: $pat, .. }) => match $expr.base10_parse::<$typ>() {
                                    Ok(x) => Ok(x),
                                    Err(err) => Err(E::invalid_value(
                                        Some(err.span()),
                                        self,
                                        &err.to_string()
                                    ))
                                 },

                                 $(Expr::Unary(syn::ExprUnary {
                                    op: syn::UnOp::Neg(_),
                                    expr,
                                    ..
                                }) => {
                                    let _shim = stringify!($x);
                                    match *expr {
                                        Expr::Lit(syn::ExprLit { lit: $pat, .. }) =>
                                        match $expr.base10_parse::<$typ>() {
                                            Ok(x) => Ok(-x),
                                            Err(err) => Err(E::invalid_value(
                                                Some(err.span()),
                                                self,
                                                &err.to_string()
                                            ))
                                        },
                                        _ => Err(E::expecting(span, self, concat!("a valid ", stringify!($typ))))
                                    }
                                },)?

                                expr => Err(E::expecting(span, self, format_args!("{:?}", expr)))
                            }
                        }
                    }

                    source.visit(_Visitor)
                }
            }
        )*
    };
}

impl_syn_pat_num! {
    for u8: syn::Lit::Int(expr) => expr,
    for u16: syn::Lit::Int(expr) => expr,
    for u32: syn::Lit::Int(expr) => expr,
    for u64: syn::Lit::Int(expr) => expr,
    for u128: syn::Lit::Int(expr) => expr,

    for -- i8: syn::Lit::Int(expr) => expr,
    for -- i16: syn::Lit::Int(expr) => expr,
    for -- i32: syn::Lit::Int(expr) => expr,
    for -- i64: syn::Lit::Int(expr) => expr,
    for -- i128: syn::Lit::Int(expr) => expr,

    for -- f32: syn::Lit::Float(expr) => expr,
    for -- f64: syn::Lit::Float(expr) => expr
}


macro_rules! impl_syn_tuple {
    ($(
        (
            $len: literal:
            $($x: ident),*
        )
    )*) => {
        $(

        impl<$($x),*> FromMeta<Expr> for ($($x),*) where $($x: FromMeta<Expr>),* {
            fn from_meta<S>(source: S) -> Result<Self, S::Error> where S: MetaSource<Expr> {
                struct _Visitor<$($x),*>(PhantomData<($($x),*)>);

                impl<$($x),*> MetaVisitor for _Visitor<$($x),*>
                    where $($x: FromMeta<Expr>),*
                {
                    type Output = ($($x),*);
                    type Domain = Expr;

                    fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
                        f.write_fmt(format_args!("a tuple {}", type_name::<Self::Output>()))
                    }

                    fn visit_value<E>(self, span: Option<Span>, value: Self::Domain) -> Result<Self::Output, E> where E: MetaError {
                        match value {
                            Expr::Tuple(syn::ExprTuple { elems , ..}) => {
                                Ok((
                                   $(match elems.next().map($x::from_meta) {
                                        Some(Ok(x)) => x,
                                        Some(Err(e)) => Err(e),
                                        None => return Err(E::invalid_value(span, self, concat!("expected tuple length ", stringify!($len)))),
                                    },)*
                                ))
                            },
                            expr => Err(E::expecting(span, self, format_args!("expr ({})", expr.as_error_display())))
                        }
                    }
                }

                source.visit()
            }
        }

        )*
    };
}

impl_syn_tuple! {
    (2: T0, T1)
}