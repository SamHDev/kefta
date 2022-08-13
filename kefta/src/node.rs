pub struct AttrNode {
    pub ident: proc_macro2::Ident,
    pub data: AttrData,
}

pub enum AttrData {
    Marker,
    Valued {
        equal: proc_macro2::Punct,
        value: proc_macro2::TokenTree,
    },
    Container {
        group: proc_macro2::Delimiter,
        nodes: Vec<AttrNode>,
    }
}

impl AttrNode {
    pub fn as_string(&self) -> String {
        self.ident.to_string()
    }
}