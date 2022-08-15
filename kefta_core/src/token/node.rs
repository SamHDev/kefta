use proc_macro2::{Delimiter, Group, TokenStream, TokenTree};
use crate::error::KeftaTokenError;
use crate::node::{AttrNode, AttrTree};
use crate::token::{AttrTokenParse, AttrTokenStream};

const NODE_EXPECTED: &'static str = "`=`, `,`, `(...)`";

// parse node
impl AttrTokenParse for AttrNode {
    fn parse(stream: &mut AttrTokenStream) -> Result<Self, KeftaTokenError> {
        Ok(AttrNode {
            // node ident
            ident: stream.parse()?,

            data: match stream.next() {
                // marker - no data
                None => AttrTree::Marker,

                // when punct
                Some(TokenTree::Punct(punct)) => match punct.as_char() {
                    // marker - found punct
                    ',' => AttrTree::Marker,

                    // valued - equality
                    '=' => AttrTree::Valued {
                        equal: punct,
                        value: stream.parse()?,
                    },

                    // container - tailfish
                    ':' => {
                        // consume second colon (optional)
                        if let Some(TokenTree::Punct(colon)) = stream.peek() {
                            if colon.as_char() == ':' {
                                // skip over colon
                                stream.skip()
                            }
                        }

                        // build tailfish container
                        AttrTree::Container {
                            group: Delimiter::None,
                            nodes: vec![stream.parse()?], // parse next,
                            tailfish: true
                        }
                    },

                    // error - invalid punct
                    _ => return Err(KeftaTokenError::Expected {
                        expected: NODE_EXPECTED,
                        description: Some("invalid punct token"),
                        found: TokenTree::Punct(punct)
                    }),
                },

                Some(TokenTree::Group(group)) => match group.delimiter() {
                    // container - grouped
                    Delimiter::Parenthesis => AttrTree::Container {
                        group: group.delimiter(),
                        nodes: AttrTokenStream::new(group.stream()).parse()?,
                        tailfish: false,
                    },

                    // error - invalid delimiter
                    _ => return Err(KeftaTokenError::Expected {
                        expected: NODE_EXPECTED,
                        description: Some("invalid group delimiter"),
                        found: TokenTree::Group(group)
                    })
                },

                Some(token_tree) => return Err(KeftaTokenError::Expected {
                    expected: NODE_EXPECTED,
                    description: None,
                    found: token_tree
                })
            }

        })
    }
}

// parse array of nodes
impl AttrTokenParse for Vec<AttrNode> {
    fn parse(stream: &mut AttrTokenStream) -> Result<Self, KeftaTokenError> {
        let mut nodes = Vec::new();

        // iterate over streams
        while stream.has_tokens() {
            nodes.push(stream.parse::<AttrNode>()?);

            // skip over separators
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

impl AttrNode {
    pub fn parse_root(stream: &mut AttrTokenStream) -> Result<Vec<AttrNode>, KeftaTokenError> {
        let group = stream.parse::<Group>()?;
        AttrTokenParse::parse(&mut AttrTokenStream::new(group.stream()))
    }
}