use syn::{Attribute, Expr, Ident, Token};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

pub fn parse_nodes(attrs: Vec<Attribute>) -> syn::Result<Vec<AttrNode>> {
    let mut build = Vec::new();

    for attr in attrs {
        build.extend(parse_node(attr)?.into_iter())
    }

    Ok(build)
}

pub fn parse_node(attr: Attribute) -> syn::Result<Punctuated<AttrNode, Token![,]>> {
    struct Wrapped(Punctuated<AttrNode, Token![,]>);
    impl Parse for Wrapped {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(Self(Punctuated::parse_terminated(input)?))
        }
    }
    Ok(syn::parse2::<Wrapped>(attr.tokens)?.0)
}


pub struct AttrNode {
    pub ident: Ident,
    pub expr: Option<(Token![=], Expr)>,
}

impl Parse for AttrNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(AttrNode {
            ident: input.parse()?,
            expr: if input.peek(Token![=]) {
                Some((input.parse()?, input.parse()?))
            } else {
                None
            }
        })
    }
}