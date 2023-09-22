use std::iter::Peekable;
use proc_macro2::{Ident, Punct, Spacing, TokenStream, TokenTree};
use crate::error::KeftaError;
use crate::node::{AttrData, AttrIdent, AttrNode};

pub fn parse_path_segment(x: Punct, stream: &mut <TokenStream as IntoIterator>::IntoIter) -> Result<(Ident, [Punct; 2]), KeftaError> {
    match stream.next() {
        None => return Err(KeftaError::UnexpectedEnd { last: x.span() }),
        Some(TokenTree::Punct(y)) if x.as_char() == ':' =>
            match stream.next() {
                Some(TokenTree::Ident(z)) => Ok((z, [x, y])),

                Some(z) => return Err(KeftaError::UnexpectedToken {
                    position: z.span(),
                    expected: Some("ident after tail-fish"),
                    token: z,
                }),

                None => return Err(KeftaError::UnexpectedEnd { last: y.span() }),
            }
        Some(y) => return Err(KeftaError::UnexpectedToken {
            position: y.span(),
            expected: Some("a leading tail-fish punct `::`"),
            token: y,
        }),
    }
}

pub fn parse_attr_node(stream: TokenStream) -> Result<AttrNode, KeftaError> {
    let mut stream = stream.into_iter();

    let mut ident_prefix = None;

    // get the first part of the ident
    let ident = match stream.next() {
        // match an empty tree
        None => return Err(KeftaError::EmptyTree),

        // match an ident
        Some(TokenTree::Ident(x)) => x,

        // match a leading tailfish
        Some(TokenTree::Punct(x)) if x.as_char() == ':' && x.spacing() == Spacing::Joint => {
            let (x, y) = parse_path_segment(x, &mut stream)?;
            ident_prefix = Some(y);
            x
        }

        Some(x) => return Err(KeftaError::UnexpectedToken {
            position: x.span(),
            expected: Some("an attribute path (identifier or tail-fish)"),
            token: x,
        }),
    };

    let mut ident = AttrIdent {
        prefix: ident_prefix,
        path: Vec::new(),
        ident,
    };


    loop {
        match stream.next() {
            None => return Ok(AttrNode { ident, data: AttrData::None }),

            Some(TokenTree::Punct(x)) if x.as_char() == ':' && x.spacing() == Spacing::Joint => {
                let (x, y) = parse_path_segment(x, &mut stream)?;
                ident.path.push((x, y));
            },

            Some(TokenTree::Punct(equals)) if equals.as_char() == '=' => {

            }
        }
    }


    Ok(())
}