use proc_macro2::{Delimiter, Ident, Punct, TokenTree};
use crate::{KeftaError, KeftaExpected, KeftaResult};
use crate::node::{AttrData, AttrNode};
use crate::stream::AttrStream;

pub trait AttrStreamParse: Sized {
    fn parse(stream: &mut AttrStream) -> KeftaResult<Self>;
}

impl AttrStreamParse for TokenTree {
    fn parse(stream: &mut AttrStream) -> KeftaResult<Self> {
        match stream.next() {
            None => Err(KeftaError::ExpectedToken),
            Some(value) => Ok(value)
        }
    }
}

impl AttrStreamParse for Ident {
    fn parse(stream: &mut AttrStream) -> KeftaResult<Self> {
        match stream.parse::<TokenTree>()? {
            TokenTree::Ident(ident) => Ok(ident),
            token @ _ => Err(KeftaError::Expected {
                expected: KeftaExpected::Ident,
                found: token
            })
        }
    }
}

impl AttrStreamParse for Punct {
    fn parse(stream: &mut AttrStream) -> KeftaResult<Self> {
        match stream.parse::<TokenTree>()? {
            TokenTree::Punct(punct) => Ok(punct),
            token @ _ => Err(KeftaError::Expected {
                expected: KeftaExpected::Punct,
                found: token
            })
        }
    }
}


impl AttrStreamParse for AttrNode {
    fn parse(stream: &mut AttrStream) -> KeftaResult<Self> {

        Ok(AttrNode {
            ident: stream.parse()?,
            data: match stream.next() {
                None => AttrData::Marker,

                Some(TokenTree::Punct(punct)) => match punct.as_char() {
                    // markers
                    ',' => AttrData::Marker,

                    // values
                    '=' => AttrData::Valued {
                        equal: punct,
                        value: stream.parse()?,
                    },

                    // tailfish containers
                    ':' => {
                        if let Some(TokenTree::Punct(punct)) = stream.peek() {
                            if punct.as_char() == ':' {
                                stream.skip()
                            }
                        }
                        AttrData::Container {
                            group: Delimiter::None,
                            nodes: vec![stream.parse()?]
                        }
                    }

                    _ => return Err(KeftaError::Expected {
                        expected: KeftaExpected::Message("`=`, `,` or `(...)`"),
                        found: TokenTree::Punct(punct)
                    })
                },
                Some(TokenTree::Group(group)) => match group.delimiter() {
                    // grouped
                    Delimiter::Parenthesis => AttrData::Container {
                        group: group.delimiter(),
                        nodes: AttrStream::new(group.stream()).parse()?
                    },

                    // error
                    _ => return Err(KeftaError::Expected {
                        expected: KeftaExpected::Message("`=`, `,` or `(...)`"),
                        found: TokenTree::Group(group)
                    })
                },

                found @ _ => return Err(KeftaError::Expected {
                    expected: KeftaExpected::Message("`=`, `,` or `(...)`"),
                    found: found.unwrap()
                })
            }
        })
    }
}

impl AttrStreamParse for Vec<AttrNode> {
    fn parse(stream: &mut AttrStream) -> KeftaResult<Self> {
        let mut nodes = Vec::new();

        while stream.has_tokens() {
            nodes.push(stream.parse::<AttrNode>()?);

            if let Some(peek) = stream.peek() {
                match peek {
                    TokenTree::Punct(punct) => if punct.as_char() == ',' {
                        let _ = stream.skip();
                    },
                    _ => continue
                }
            } else {
                break;
            }
        }

        Ok(nodes)
    }
}