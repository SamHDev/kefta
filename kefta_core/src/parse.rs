use proc_macro2::{Delimiter, Ident, Punct, Spacing, TokenStream, TokenTree};
use crate::error::{ParseError};
use crate::node::{AttrData, AttrIdent, AttrNode};


type TokenStreamIter = <TokenStream as IntoIterator<Item=TokenTree>>::IntoIter;

fn is_tailfish_open(punct: &Punct) -> bool {
    punct.as_char() == ':' && punct.spacing() == Spacing::Joint
}

fn parse_tailfish(x: Punct, stream: &mut TokenStreamIter) -> Result<[Punct; 2], ParseError> {
    debug_assert!(is_tailfish_open(&x), "`x` is not a valid tailfish");

    match stream.next() {
        None => Err(ParseError::UnexpectedEnd { last: x.span(), }),
        Some(TokenTree::Punct(y)) if y.as_char() == ':' => Ok([x, y]),
        Some(y) => Err(ParseError::UnexpectedToken {
            position: x.span(),
            expected: Some("tailfish"),
            token: y,
        })
    }
}

fn parse_tailing_ident(prefix: &[Punct; 2], stream: &mut TokenStreamIter) -> Result<Ident, ParseError> {
    match stream.next() {
        None => Err(ParseError::UnexpectedEnd {
            last: prefix[1].span(),
        }),

        Some(TokenTree::Ident(ident)) => Ok(ident),

        Some(token) => Err(ParseError::UnexpectedToken {
            position: prefix[1].span(),
            expected: Some("identifier after tailfish"),
            token,
        })
    }
}

fn parse_initial_ident(stream: &mut TokenStreamIter) -> Result<Option<AttrIdent>, ParseError> {
    match stream.next() {
        None => Ok(None),

        Some(TokenTree::Ident(ident)) => Ok(Some(AttrIdent {
            prefix: None,
            path: Vec::new(),
            ident,
        })),

        Some(TokenTree::Punct(x)) if is_tailfish_open(&x) => {
            let prefix = parse_tailfish(x, stream)?;
            let ident = parse_tailing_ident(&prefix, stream)?;

            Ok(Some(AttrIdent {
                prefix: Some(prefix),
                path: Vec::new(),
                ident,
            }))
        }

        Some(token) => Err(ParseError::UnexpectedToken {
            position: token.span(),
            expected: Some("invalid attribute"),
            token,
        })
    }
}

fn parse_node_content(mut ident: AttrIdent, stream: &mut TokenStreamIter) -> Result<AttrNode, ParseError> {
    loop {
        match stream.next() {
            None => return Ok(AttrNode { ident, data: AttrData::None }),

            Some(TokenTree::Punct(equals)) if equals.as_char() == '=' => {

                let Some(value) = stream.next() else {
                    return Err(ParseError::UnexpectedEnd { last: equals.span(), })
                };

                return Ok(AttrNode { ident, data: AttrData::Valued { equals, value }})
            }

            Some(TokenTree::Punct(colon)) if is_tailfish_open(&colon) => {
                let tailfish = parse_tailfish(colon, stream)?;
                let segment = parse_tailing_ident(&tailfish, stream)?;
                ident.path.push((segment, tailfish));
            }

            Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Parenthesis => {
                return Ok(AttrNode {
                    ident,
                    data: AttrData::List { group, contents: group.stream()},
                })
            },

            Some(token) => return Err(ParseError::UnexpectedToken {
                position: token.span(),
                expected: None,
                token,
            })
        }
    }
}

pub fn parse_node(stream: TokenStream) -> Result<AttrNode, ParseError> {
    let mut stream = stream.into_iter();

    let ident = parse_initial_ident(&mut stream)?
        .ok_or_else(ParseError::EmptyTree)?;

    parse_node_content(ident, &mut stream)
}
