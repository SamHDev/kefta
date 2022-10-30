use proc_macro::{Literal, Span, TokenStream, TokenTree};
use quote::{format_ident, quote};
use syn::{DataStruct, Field, Generics};
use syn::__private::quote::quote_spanned;
use syn::__private::TokenStream2;
use syn::spanned::Spanned;
use kefta_core::node::AttrSource;
use kefta_core::parse::AttrModel;
use syn::Ident;
use crate::attrs::{ItemAttr, ModelAttr};

pub fn process(model: ModelAttr, data: DataStruct, ident: Ident, generics: Generics) -> Result<TokenStream2, syn::Error> {
    let mut namespaces = vec![None];

    let mut fields = TokenStream2::new();
    for field in data.fields {
        fields.extend(process_field(field, &model.namespace, &mut namespaces)?);
    }

    let mut constructor = TokenStream2::new();

    for (i, name) in namespaces.into_iter().enumerate() {
        let ident = format_ident!("_k_ns{}", i);

        if let Some(name) = name {
            let lit = Literal::string(&name);
            let lit = TokenStream2::from(TokenStream::from(TokenTree::Literal(lit)));

            constructor.extend(quote!(let mut #ident = kefta::AttrMap::new_named( _k_ns0.get(Some(#lit)) )?; ));
        } else {
            constructor.extend(quote!(let mut #ident = kefta::AttrMap::new(_k_nodes); ));
        }
    }


    let func = quote!(impl #generics kefta::AttrModel for #ident #generics  {
         fn parse(_k_nodes: Vec<kefta::AttrNode>) -> kefta::error::KeftaResult<Self> {
            #constructor;

            Ok(Self {
                #fields
            })
        }
    });

    Ok(func)
}

fn process_field(
    field: Field,
    namespace: &Option<String>,
    namespaces: &mut Vec<Option<String>>
) -> Result<TokenStream2, syn::Error> {
    // get span
    let span = field.span();

    // parse attrs
    let nodes = AttrSource::parse(field.attrs).unwrap();
    let attrs = ItemAttr::parse(nodes).unwrap();


    // calc namespace
    let namespace = if attrs.root_namespace {
        None
    } else {
        match attrs.namespace {
            None => namespace.clone(),
            Some(x) => Some(x),
        }
    };

    // get namespace index
    let ns = if let Some(pos) = namespaces.iter().position(|x| x == &namespace) {
        pos
    } else {
        let index = namespaces.len();
        namespaces.push(namespace);
        index
    };

    // get field ident
    let ident = match field.ident {
        None => return Err(syn::Error::new(span, "expected a named field")),
        Some(ident) => ident,
    };

    // build names
    let mut names = Vec::new();

    // extract ident or rename
    let (name, name_span) = match attrs.rename {
        None => (ident.to_string(), ident.span().unwrap()),
        Some((name, span)) => (name, span)
    };
    names.push((Some(name), name_span));

    // root value
    if let (true, Some(span)) = attrs.value {
        names.push((None, span));
    }

    // aliases
    for (alias, span) in attrs.alias {
        names.push((Some(alias), span))
    }

    // todo: check if duplicates

    // build parse source
    let source = if let (true, Some(span)) = attrs.rest {
        quote_spanned!(span.into() => .rest() )
    } else {
        if names.len() == 1 {
            let (name, span) = names.into_iter().next().unwrap();
            let x = get_name(name, span);
            quote!( .get( #x ) )
        } else {
            let mut tokens = TokenStream2::new();

            for (name, span) in names {
                if !tokens.is_empty() {
                    tokens.extend(quote!(,));
                }

                tokens.extend(get_name(name, span) );
            }

            quote!( .gather( &[ #tokens ] ) )
        }
    };

    let ns_ident = format_ident!("_k_ns{}", ns);
    let parse_type = field.ty;

    let field = if attrs.default {
        quote!( #ident: < Option<#parse_type> as kefta::AttrModel >::parse( #ns_ident #source )?.unwrap_or_default() , )
    } else if let Some(expr) = attrs.default_value {
        let expr = TokenStream2::from(TokenStream::from(expr));
        quote!( #ident: < Option<#parse_type> as kefta::AttrModel >::parse( #ns_ident #source )?.unwrap_or(#expr) , )
    } else {
        quote!( #ident: < #parse_type as kefta::AttrModel >::parse( #ns_ident #source )? , )
    };

    Ok(field)
}

fn get_name(name: Option<String>, span: Span) -> TokenStream2 {
    if let Some(name) = name {
        let mut lit = Literal::string(&name);
        lit.set_span(span);
        let lit = TokenStream2::from(TokenStream::from(TokenTree::Literal(lit)));

        quote_spanned!(span.into() => Some(#lit))
    } else {
        quote_spanned!(span.into() => None)
    }.into()
}