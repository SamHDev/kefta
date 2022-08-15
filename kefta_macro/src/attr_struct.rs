use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Field, LitStr};
use syn::spanned::Spanned;
use kefta_core::structs::AttrParse;
use crate::attr::StructAttr;

pub fn attr_struct(input: DeriveInput) -> syn::Result<TokenStream> {
    let data = if let Data::Struct(data) = input.data { data } else { unreachable!() };

    let mut constructor = TokenStream::new();

    for field in data.fields {
        constructor.extend(attr_struct_field(field));
    }

    let (ident, generics) = (input.ident, input.generics);

    Ok(quote! {
        impl #generics kefta::AttrStruct for #ident #generics {
            fn parse(nodes: Vec<kefta::AttrNode>) -> kefta::error::KeftaResult<Self> {
                let mut map = kefta::AttrMap::new(nodes);
                Ok(Self {
                    #constructor
                })
            }
        }
    })
}

fn attr_struct_field(field: Field) -> syn::Result<TokenStream> {
    // parse ident
    let ident = if let Some(ident) = field.ident {
        ident
    } else {
        return Err(syn::Error::new(field.span(), "expected a named field"));
    };

    // parse attrs
    let attrs: StructAttr = match field.attrs.parse_attrs() {
        Ok(attr) => attr,
        Err(e) => return Err(e.into())
    };

    //println!("{:?}", attrs);

    // build key array
    let mut keys = if let Some(rename) = attrs.name {
        vec![LitStr::new(&rename, ident.span())]
    } else {
        vec![LitStr::new(&ident.to_string(), ident.span())]
    };
    for alias in attrs.alias {
        keys.push(LitStr::new(&alias, ident.span()));
    }

    // build key tokens
    let mut key_arr = TokenStream::new();
    let key_len = keys.len() - 1;
    for (i, key) in keys.into_iter().enumerate() {
        if i != key_len {
            key_arr.extend(quote! { #key, })
        } else {
            key_arr.extend(quote! { #key })
        }
    }
    let keys = quote!( &[ #key_arr ] );


    let func = if let Some(call) = attrs.with {
        let call_ident = format_ident!("{}", call);
        quote! { map.parse_with(#keys, #call_ident) }
    } else if attrs.container {
        quote! { map.parse_container(#keys) }
    } else {
        match (attrs.required, attrs.optional) {

            (false, false) => if attrs.multiple {
                quote! { map.parse_array(#keys) }
            } else {
                quote! { map.parse_one(#keys) }
            },

            (true, false) => if attrs.multiple {
                quote! { map.parse_array_required(#keys) }
            } else {
                quote! { map.parse_required(#keys) }
            },

            (false, true) => if attrs.multiple {
                quote! { map.parse_array_optional(#keys) }
            } else {
                quote! { map.parse_optional(#keys) }
            },

            (true, true) => return Err(syn::Error::new(
                ident.span(),
                "attribute cannot be optional and required."
            ))
        }
    };

    Ok(quote!( #ident: #func ?, ))
}
