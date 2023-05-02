//! This crate implements the macro for `colored_macro` and should not be used directly.

use proc_macro2::TokenStream;
use ansi::RESET;
use parser::Element;
use syn::{
    parse::{Parse, ParseStream, Result},
    token::Comma,
    Expr, LitStr,
    parse2
};

use crate::{ansi::style_to_ansi_code, parser::parse_tags};
mod ansi;
mod parser;
#[cfg(test)]
mod tests;

#[doc(hidden)]
pub fn colored_macro(item: TokenStream) -> Result<TokenStream> {
    Ok(process(parse2(item)?))
}

/// A segment is like a token, it can either be optionally styled text or a style end tag.
#[derive(Debug, PartialEq)]
pub(crate) enum Segment {
    /// (Style, Text)
    Text(Option<String>, String),
    /// (Style)
    StyleEnd(String),
}

#[derive(Debug, PartialEq)]
pub(crate) struct Colored {
    /// The segments of the string literal.
    /// A segment is like a token, it can either be optionally styled text or a style end tag.
    pub segments: Vec<Segment>,
    /// Arguments passed after the string literal (e.g. `colored!("Hello, {}!", "world")`, `"world"` is a format argument).
    /// This only includes arguments that literally appear after the comma, not inline arguments like `{name}`.
    pub format_args: Vec<String>,
}

impl Parse for Colored {
    /// Parse the macro input into a `Colored` struct.
    /// The input is in the form `"String <red>literal</red>", format_args*`.
    fn parse(input: ParseStream) -> Result<Self> {
        let mut segments = Vec::new();
        let mut format_args = Vec::new();
        let mut style = None;

        for element in parse_tags(input.parse::<LitStr>()?.value()) {
            match element {
                Element::Start(tag_name) => {
                    style = Some(tag_name.to_string());
                }
                Element::End(tag_name) => {
                    segments.push(Segment::StyleEnd(tag_name.to_string()));
                }
                Element::Text(text) => {
                    segments.push(Segment::Text(style.take(), text));
                }
            }
        }

        while !input.is_empty() {
            input.parse::<Comma>()?;
            let expr = input.parse::<Expr>()?;
            format_args.push(quote::quote!(#expr).to_string());
        }

        Ok(Self {
            segments,
            format_args,
        })
    }
}

pub(crate) fn process(colored: Colored) -> TokenStream {
    let mut fmt_string = String::new();
    let mut style_stack = Vec::new();

    for segment in colored.segments {
        match segment {
            Segment::Text(style, text) => {
                if let Some(style) = style {
                    let ansi_style = style_to_ansi_code(style);
                    fmt_string.push_str(&format!("{}{}", ansi_style, text));
                    style_stack.push(ansi_style);
                } else {
                    fmt_string.push_str(&text);
                }
            }
            Segment::StyleEnd(style) => {
                let ansi_style = style_to_ansi_code(style);

                if let Some(prev_ansi_style) = style_stack.pop() {
                    if prev_ansi_style != ansi_style {
                        panic!(
                            "Mismatched style end tag: expected {}, got {}",
                            prev_ansi_style.escape_default(),
                            ansi_style.escape_default()
                        );
                    }
                }

                let reset = if let Some(prev_ansi_style) = style_stack.last() {
                    format!("\x1b[0;00m{}", prev_ansi_style)
                } else {
                    RESET.to_string()
                };

                fmt_string.push_str(&reset);
            }
        }
    }

    fmt_string.push_str(RESET);

    let mut output = String::new();

    // Open a `format!("` call.
    output.push_str("format!(\"");
    output.push_str(&fmt_string);
    // Close the `format!("` call with `"`.
    output.push_str("\", ");

    // Push the format arguments.
    for arg in colored.format_args {
        output.push_str(&arg);
        output.push_str(", ");
    }

    // Close the `format!` call
    output.push(')');
    output.parse().unwrap()
}
