#[derive(Debug)]
/// An element in the string literal.
pub enum Element {
    /// A tag that starts a style.
    /// <red>
    Start(String),
    /// A tag that ends a style.
    /// </red>
    End(String),
    /// Any other text in the string literal.
    Text(String),
}

/// Parses a string literal into a vector of `Element`s.
pub fn parse_tags(input: String) -> Vec<Element> {
    let c = input.chars();
    let mut elements = Vec::new();
    let mut current = String::new();
    let mut in_tag = false;

    for c in c {
        if c == '<' {
            if !current.is_empty() {
                elements.push(Element::Text(current));
                current = String::new();
            }

            in_tag = true;
        } else if c == '>' {
            if in_tag {
                if let Some(stripped) = current.strip_prefix('/') {
                    elements.push(Element::End(stripped.to_string()));
                } else {
                    elements.push(Element::Start(current));
                }

                current = String::new();
                in_tag = false;
            } else {
                current.push(c);
            }
        } else {
            current.push(c);
        }
    }

    if !current.is_empty() {
        elements.push(Element::Text(current));
    }

    elements
}
