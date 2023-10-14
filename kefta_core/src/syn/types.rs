use std::fmt::Formatter;
use proc_macro2::Span;
use syn::{Expr, ExprLit, Lit, LitBool};
use crate::model::{FromMeta, MetaError, MetaFound, MetaReceiver, MetaSource};

impl FromMeta<Expr> for Expr {
    fn from_meta<S>(source: S) -> Result<Self, S::Error> where S: MetaSource<Expr> {
        struct _Recv;

        impl MetaReceiver for _Recv {
            type Domain = Expr;
            type Output = Expr;

            fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
                f.write_str("a valid expression")
            }

            fn visit_value<E>(self, _span: Option<Span>, value: Self::Domain) -> Result<Self::Output, E> where E: MetaError {
                Ok(value)
            }
        }

        source.visit(_Recv)
    }
}

impl FromMeta<Expr> for bool {
    fn from_meta<S>(source: S) -> Result<Self, S::Error> where S: MetaSource<Expr> {
        struct _Recv;

        impl MetaReceiver for _Recv {
            type Domain = Expr;
            type Output = bool;

            fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
                f.write_str("a marker or boolean value")
            }

            fn visit_marker<E>(self, _span: Option<Span>) -> Result<Self::Output, E> where E: MetaError {
                Ok(true)
            }

            fn visit_value<E>(self, span: Option<Span>, value: Self::Domain) -> Result<Self::Output, E> where E: MetaError {
                match value {
                    Expr::Lit(ExprLit { lit: Lit::Bool(LitBool { value, .. }), .. }) => Ok(value),
                    _ => Err(E::expecting(span, self, MetaFound::Custom("a non boolean expr")))
                }
            }
        }

        source.visit(_Recv)
    }
}

macro_rules! syn_expr_impl {
    ($(for $typ: ty: $pat: pat => $expr: expr),*) => {
        $(impl FromMeta<Expr> for $typ {
            fn from_meta<S>(source: S) -> Result<Self, S::Error> where S: MetaSource<Expr> {
                struct _Recv;

                impl MetaReceiver for _Recv {
                    type Domain = Expr;
                    type Output = $typ;

                    fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
                        f.write_str(concat!("a valid ", stringify!($typ)))
                    }

                    fn visit_value<E>(self, span: Option<Span>, value: Self::Domain) -> Result<Self::Output, E> where E: MetaError {
                        match value {
                            $pat => Ok($expr),
                            _ => Err(E::expecting(span, self, MetaFound::Custom(
                                concat!("a valid ", stringify!($typ))
                            )))
                        }
                    }
                }

                source.visit(_Recv)
            }
        })*
    };
}

syn_expr_impl! {
	for syn::ExprArray: Expr::Array(expr) => expr,
	for syn::ExprAssign: Expr::Assign(expr) => expr,
	for syn::ExprAsync: Expr::Async(expr) => expr,
	for syn::ExprAwait: Expr::Await(expr) => expr,
	for syn::ExprBinary: Expr::Binary(expr) => expr,
	for syn::ExprBlock: Expr::Block(expr) => expr,
	for syn::ExprBreak: Expr::Break(expr) => expr,
	for syn::ExprCall: Expr::Call(expr) => expr,
	for syn::ExprCast: Expr::Cast(expr) => expr,
	for syn::ExprClosure: Expr::Closure(expr) => expr,
	for syn::ExprConst: Expr::Const(expr) => expr,
	for syn::ExprContinue: Expr::Continue(expr) => expr,
	for syn::ExprField: Expr::Field(expr) => expr,
	for syn::ExprForLoop: Expr::ForLoop(expr) => expr,
	for syn::ExprGroup: Expr::Group(expr) => expr,
	for syn::ExprIf: Expr::If(expr) => expr,
	for syn::ExprIndex: Expr::Index(expr) => expr,
	for syn::ExprInfer: Expr::Infer(expr) => expr,
	for syn::ExprLet: Expr::Let(expr) => expr,
	for syn::ExprLit: Expr::Lit(expr) => expr,
	for syn::ExprLoop: Expr::Loop(expr) => expr,
	for syn::ExprMacro: Expr::Macro(expr) => expr,
	for syn::ExprMatch: Expr::Match(expr) => expr,
	for syn::ExprMethodCall: Expr::MethodCall(expr) => expr,
	for syn::ExprParen: Expr::Paren(expr) => expr,
	for syn::ExprPath: Expr::Path(expr) => expr,
	for syn::ExprRange: Expr::Range(expr) => expr,
	for syn::ExprReference: Expr::Reference(expr) => expr,
	for syn::ExprRepeat: Expr::Repeat(expr) => expr,
	for syn::ExprReturn: Expr::Return(expr) => expr,
	for syn::ExprStruct: Expr::Struct(expr) => expr,
	for syn::ExprTry: Expr::Try(expr) => expr,
	for syn::ExprTryBlock: Expr::TryBlock(expr) => expr,
	for syn::ExprTuple: Expr::Tuple(expr) => expr,
	for syn::ExprUnary: Expr::Unary(expr) => expr,
	for syn::ExprUnsafe: Expr::Unsafe(expr) => expr,
	for syn::ExprWhile: Expr::While(expr) => expr,
	for syn::ExprYield: Expr::Yield(expr) => expr,

    for syn::Path: Expr::Path(syn::ExprPath { path, ..}) => path,
    for syn::Lit: Expr::Lit(syn::ExprLit { lit, .. }) => lit,

    for syn::LitStr: Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(expr), ..}) => expr,
	for syn::LitByteStr: Expr::Lit(syn::ExprLit { lit: syn::Lit::ByteStr(expr), ..}) => expr,
	for syn::LitByte: Expr::Lit(syn::ExprLit { lit: syn::Lit::Byte(expr), ..}) => expr,
	for syn::LitChar: Expr::Lit(syn::ExprLit { lit: syn::Lit::Char(expr), ..}) => expr,
	for syn::LitInt: Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(expr), ..}) => expr,
	for syn::LitFloat: Expr::Lit(syn::ExprLit { lit: syn::Lit::Float(expr), ..}) => expr,
	for syn::LitBool: Expr::Lit(syn::ExprLit { lit: syn::Lit::Bool(expr), ..}) => expr,

    for String: Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(expr), ..}) => expr.value(),
    for char: Expr::Lit(syn::ExprLit { lit: syn::Lit::Char(expr), ..}) => expr.value()
}

macro_rules! syn_num_impl {
    ($(for $(-$tt:tt)? $typ: ty: $pat: pat => $expr: expr),*) => {
        $(impl FromMeta<Expr> for $typ {
            fn from_meta<S>(source: S) -> Result<Self, S::Error> where S: MetaSource<Expr> {
                struct _Recv;

                impl MetaReceiver for _Recv {
                    type Domain = Expr;
                    type Output = $typ;

                    fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
                        f.write_str(concat!("a valid ", stringify!($typ)))
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

                            $(

                                Expr::Unary(syn::ExprUnary {
                                    op: syn::UnOp::Neg(_),
                                    expr,
                                    ..
                                }) => {
                                    let _shim = stringify!($tt);
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
                                        _ => Err(E::expecting(span, self, MetaFound::Custom(
                                            concat!("a valid ", stringify!($typ))
                                        )))
                                    }
                                },

                            )?

                            _ => Err(E::expecting(span, self, MetaFound::Custom(
                                concat!("a valid ", stringify!($typ))
                            )))
                        }
                    }
                }

                source.visit(_Recv)
            }
        })*
    };
}

syn_num_impl! {

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