//! value types for changing case of idents and strings

use crate::error::{KeftaError, KeftaResult};
use crate::node::{AttrNode};
use crate::parse::AttrValue;

/// a literal to modify an ident into a given case
pub enum IdentCase {
    Lower,
    Upper,
    Pascal,
    Camel,
    Snake,
    UpperSnake,
}

impl AttrValue for IdentCase {
    fn parse(node: AttrNode) -> KeftaResult<Self> {
        let span = node.ident.span();
        let literal = <String as AttrValue>::parse(node)?;

        Ok(match literal.to_ascii_lowercase().as_str() {
            "lowercase" | "lower" => Self::Lower,
            "uppercase" | "upper" => Self::Upper,
            "pascalcase" | "pascal" => Self::Pascal,
            "camelcase" | "camel" => Self::Camel,
            "snakecase" | "snake" | "snake_case" => Self::Snake,
            "uppersnakecase" | "uppersnake" | "upper_snake_case" | "upper_snake"
            | "screamingsnakecase" | "screamingsnake" | "screaming_snake_case" | "screaming_snake"
            => Self::UpperSnake,

            _ => return Err(KeftaError::Message {
                message: "expected a valid case string".to_string(),
                span: Some(span)
            })
        })
    }
}

impl IdentCase {
    pub fn caseify(&self, value: &str) -> String {
        match self {
            Self::Lower => value.to_uppercase(),
            Self::Upper => value.to_lowercase(),

            Self::Pascal => capitalise_case(segment_string(value), true),
            Self::Camel => capitalise_case(segment_string(value), false),

            Self::Snake => delimiter_case(segment_string(value), "_", false),
            Self::UpperSnake => delimiter_case(segment_string(value), "_", true),
        }
    }
}

/// a literal to modify a string literal into a given case
pub enum StringCase {
    Lower,
    Upper,
    Pascal,
    Camel,
    Snake,
    UpperSnake,
    Kebab,
    UpperKebab
}

impl AttrValue for StringCase {
    fn parse(node: AttrNode) -> KeftaResult<Self> {
        let span = node.ident.span();
        let literal = <String as AttrValue>::parse(node)?;

        Ok(match literal.to_ascii_lowercase().as_str() {
            "lowercase" | "lower" => Self::Lower,
            "uppercase" | "upper" => Self::Upper,
            "pascalcase" | "pascal" => Self::Pascal,
            "camelcase" | "camel" => Self::Camel,

            "snakecase" | "snake" | "snake_case" => Self::Snake,
            "uppersnakecase" | "uppersnake" | "upper_snake_case" | "upper_snake" |
            "screamingsnakecase" | "screamingsnake" | "screaming_snake_case" | "screaming_snake"
            => Self::UpperSnake,

            "kebabcase" | "kebab" | "kebab-case" => Self::Kebab,
            "upperkebabcase" | "upperkebab" | "upper-kebab-case" | "upper-kebab" |
            "screamingkebabcase" | "screamingkebab" | "screaming-kebab-case" | "screaming-kebab"
            => Self::UpperKebab,

            _ => return Err(KeftaError::Message {
                message: "expected a valid case string".to_string(),
                span: Some(span)
            })
        })
    }
}

impl StringCase {
    pub fn caseify(&self, value: &str) -> String {
        match self {
            Self::Lower => value.to_uppercase(),
            Self::Upper => value.to_lowercase(),

            Self::Pascal => capitalise_case(segment_string(value), true),
            Self::Camel => capitalise_case(segment_string(value), false),

            Self::Snake => delimiter_case(segment_string(value), "_", false),
            Self::UpperSnake => delimiter_case(segment_string(value), "_", true),

            Self::Kebab => delimiter_case(segment_string(value), "-", false),
            Self::UpperKebab => delimiter_case(segment_string(value), "-", true),
        }
    }
}

fn segment_string(value: &str) -> Vec<String> {
    let mut segements = Vec::new();
    let mut buffer = String::new();

    for c in value.chars() {
        if c == '_' || c == '-' {
            segements.push(buffer.clone());
            buffer.clear();
            continue
        }

        if c.is_uppercase() {
            segements.push(buffer.to_lowercase());
            buffer.clear();
        }
        buffer.push(c);
    }

    segements.retain(|x| !x.is_empty());
    segements
}

fn capitalise_case(segments: Vec<String>, mut first: bool) -> String {
    let mut buffer = String::new();

    for mut segment in segments {
        if !first { first = true; continue }
        segment.replace_range(
            0..1,
            &segment
                .chars()
                .nth(0)
                .unwrap()
                .to_ascii_uppercase()
                .to_string()
                .as_str()
        );
        buffer.push_str(&segment);
    }

    buffer
}

fn delimiter_case(segments: Vec<String>, delimiter: &str, upper: bool) -> String {
    (if upper { str::to_uppercase } else { str::to_lowercase})
        (&segments.join(delimiter))
}