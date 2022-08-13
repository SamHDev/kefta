use proc_macro2::Span;
use syn::{Expr, Lit, Token};
use syn::spanned::Spanned;

pub fn require_expr(key: &Span, expr: Option<(Token![=], Expr)>) -> syn::Result<Expr> {
    match expr {
        None => Err(syn::Error::new(key.clone(), "expected a value")),
        Some((_equal, expr)) => Ok(expr),
    }
}

pub fn disallow_expr(expr: Option<(Token![=], Expr)>) -> syn::Result<()> {
    match expr {
        None => Ok(()),
        Some((equal, _expr)) => Err(syn::Error::new(equal.span(), "did not expect a value")),
    }
}

pub fn string_literal(expr: Expr) -> syn::Result<String> {
    match expr {
        Expr::Lit(ref literal) => match &literal.lit {
            Lit::Str(expr) => Ok(expr.value()),
            _ => Err(syn::Error::new(expr.span(), "expected a literal string"))
        },
        _ => Err(syn::Error::new(expr.span(), "expected a literal value"))
    }
}