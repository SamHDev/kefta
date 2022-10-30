use proc_macro::{Delimiter, Ident, Spacing, TokenTree};
use crate::node::{AttrContents, AttrNode};
use crate::node::error::{ParseError, ParseErrorKind};
use crate::node::node::ContainerType;
use crate::node::stream::ParseTokenStream;

pub fn parse_body(stream: &mut ParseTokenStream) -> Result<Vec<AttrNode>, ParseError> {
    let mut nodes = Vec::new();

    // iter/parse over nodes
    loop {
        let node = parse_node(stream)?;
        nodes.push(node);

        // check for delimiter
        if let Some(token) = stream.next() {
            match token {
                TokenTree::Punct(punct) if punct.as_char() == ',' => {
                    continue
                },
                _ => {
                    // unexpected token
                    return Err((ParseErrorKind::ExpectedDelimiter, token.span()).into())
                }
            }
        } else {
            // break, we've hit the end of the body
            break
        }
    }

    Ok(nodes)
}

pub fn parse_node(stream: &mut ParseTokenStream) -> Result<AttrNode, ParseError> {
    if let Some(token) = stream.next() {
        match token {
            // parse literal values
            value@ TokenTree::Literal(_) => Ok(AttrNode::Literal { value }),

            // parse other values
            TokenTree::Ident(ident) => parse_content(ident, stream),

            // expected literal or ident
            _ => Err((ParseErrorKind::ExpectedAttribute, token.span()).into())
        }
    } else {
        // no data in stream
        Err(ParseErrorKind::UnexpectedEnd.into())
    }
}

pub fn parse_content(ident: Ident, stream: &mut ParseTokenStream) -> Result<AttrNode, ParseError> {
    if let Some(token) = stream.peek().cloned() {
        match token {
            // punct
            TokenTree::Punct(ref punct) => match punct.as_char() {
                // delimiter - return marker
                ',' => Ok(AttrNode::Marker { ident }),

                // equality `=`
                '=' => Ok(AttrNode::Value {
                    ident,
                    equal: punct.clone(),
                    value: {
                        stream.skip();
                        if let Some(value) = stream.next() {
                            value
                        } else {
                            return Err((ParseErrorKind::ExpectedValue, punct.span()).into())
                        }
                    }
                }),

                // tailfish `::`
                ':' => {
                    stream.skip();
                    if let Some(token) = stream.next() {
                        if let TokenTree::Punct(punct) = token {
                            if punct.spacing() == Spacing::Joint && punct.as_char() == ':' {

                                // parse next ident
                                let next = match stream.next() {
                                    None => return Err((ParseErrorKind::ExpectedTailfishIdent, punct.span()).into()),
                                    Some(TokenTree::Ident(ident)) => ident,
                                    Some(token) => return Err((ParseErrorKind::ExpectedTailfishIdent, token.span()).into()),
                                };

                                // parse inline nodes
                                let contents = parse_content(next, stream)?;

                                // container
                                Ok(AttrNode::Container {
                                    ident,
                                    // todo join if feature enabled
                                    group: punct.span(),
                                    container_type: ContainerType::Tailfish,
                                    contents: AttrContents::Node(Box::new(contents))
                                })

                            } else {
                                // expected punct tailfish
                                return Err((ParseErrorKind::ExpectedTailfishPunct, punct.span()).into())
                            }
                        } else {
                            // expected punct
                            return Err((ParseErrorKind::ExpectedTailfishPunct, token.span()).into())
                        }
                    } else {
                        // expected token
                        return Err((ParseErrorKind::ExpectedTailfishPunct, token.span()).into())
                    }
                }

               _ => return Err((ParseErrorKind::InvalidContent, token.span()).into())
            },

            // group `()`
            TokenTree::Group(group) => {
                stream.skip();
                if group.delimiter() == Delimiter::Parenthesis {
                    // parse contents and return container
                    Ok(AttrNode::Container {
                        ident,
                        group: group.span(),
                        container_type: ContainerType::Grouped,
                        contents: AttrContents::Stream(group.stream())
                    })
                } else {
                    // invalid group type
                    return Err((ParseErrorKind::InvalidContainerGroup, group.span()).into())
                }
            },

            // invalid token - expecting above
            token @ _ => {
                return Err((ParseErrorKind::InvalidContent, token.span()).into())
            }
        }
    } else {
        // no more tokens - return marker
        Ok(AttrNode::Marker { ident })
    }
}

