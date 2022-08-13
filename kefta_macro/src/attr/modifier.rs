// name modifiers for renaming all.

pub enum NameModifier {
    Lower,
    Upper,
    Pascal,
    Camel,
    Snake,
    UpperSnake,
}

impl NameModifier {
    pub fn parse_string(name: &str) -> Option<Self> {
        match name {
            "lowercase" => Some(Self::Lower),
            "uppercase" | "Uppercase" => Some(Self::Upper),
            "pascalcase" | "PascalCase" => Some(Self::Pascal),
            "camelcase" | "camelCase" => Some(Self::Camel),
            "snakecase" | "snake_case" => Some(Self::Snake),
            "screaming_snake_case" | "SCREAMING_SNAKE_CASE"
                | "upper_snake_cake" | "UPPER_SNAKE_CASE" => Some(Self::UpperSnake),
            _ => None,
        }
    }

    pub fn execute(&self, name: &str) -> String {
        let mut segments = Vec::new();

        let mut buffer = String::new();
        for c in name.chars() {
            if c.is_uppercase() || c == '_' {
                segments.push(buffer);
                buffer = String::new();
            }

            buffer.push(c);
        }
        segments.push(buffer);

        // clear empty
        segments.retain(|x| !x.is_empty());


        match self {
            NameModifier::Lower => segments.join("").to_ascii_lowercase(),
            NameModifier::Upper => segments.join("").to_ascii_uppercase(),
            NameModifier::Pascal => {
                for mut seg in segments.iter_mut() {
                    seg
                        .to_ascii_lowercase()
                        .replace_range(
                            0..1,
                            &seg.chars().nth(0).unwrap().to_ascii_uppercase().to_string()
                        )
                }
                segments.join("")
            }

            NameModifier::Camel => {
                let mut first = false;
                for mut seg in segments.iter_mut() {
                    if !first { first = true; continue; }
                    seg
                        .to_ascii_lowercase()
                        .replace_range(
                            0..1,
                            &seg.chars().nth(0).unwrap().to_ascii_uppercase().to_string()
                        )
                }
                segments.join("")
            },

            NameModifier::Snake => {
                segments.join("_").to_ascii_lowercase()
            },

            NameModifier::UpperSnake => {
                segments.join("_").to_ascii_uppercase()
            }
        }
    }
}