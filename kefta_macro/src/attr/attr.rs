use crate::expr::{disallow_expr, require_expr, string_literal};
use crate::nodes::AttrNode;

pub struct StructItemAttr {
    pub name: Option<String>,
    pub require: bool,
    pub flag: bool,
}

impl Default for StructItemAttr {
    fn default() -> Self {
        StructItemAttr {
            name: None,
            require: false,
            flag: false,
        }
    }
}

impl StructItemAttr {
    pub fn parse_node(&mut self, node: AttrNode) -> syn::Result<()> {
        match node.ident.to_string().as_str() {
            "name" => {
                self.name = Some(string_literal(require_expr(&node.ident.span(), node.expr)?)?);
            },
            "require" => {
                disallow_expr(node.expr)?;
                self.require = true;
            },
            "flag" => {
                disallow_expr(node.expr)?;
                self.flag = true;
            },

            _ => return Err(syn::Error::new(
                node.ident.span(),
                "unknown item attribute",
            )),
        }

        Ok(())
    }
}

pub struct EnumItemAttr {
    pub name: Option<String>,
}