use colored_macro_impl::{Segment, Colored};

macro_rules! colored {
    ($segments: expr) => {
        Colored {
            format_args: vec![],
            segments: $segments,
        }
    };

    ($segments: expr, $format_args: expr) => {
        Colored {
            format_args: $format_args,
            segments: $segments,
        }
    };
}

macro_rules! text {
    ($text: literal) => {
        Segment::Text(None, $text.to_owned())
    };

    ($text: literal, $tag: literal) => {
        Segment::Text(Some($tag.to_owned()), $text.to_owned())
    };
}

macro_rules! end {
    ($tag: literal) => {
        Segment::StyleEnd($tag.to_owned())
    };
}

macro_rules! parses_to {
    ($input: literal, $expected: expr) => {
        let tokens = format!("\"{}\"", $input).parse::<proc_macro2::TokenStream>().unwrap();
        assert_eq!(syn::parse2::<Colored>(tokens).unwrap(), $expected)
    };
}

#[test]
fn parse_no_tags() {
    parses_to!("test", colored!(vec![text!("test")]));
}

#[test]
fn parse_empty_string() {
    parses_to!("", colored!(vec![]));
}

#[test]
fn parse_simple() {
    parses_to!("<red>test</red>", colored!(vec![text!("test", "red"), end!("red")]));
}

#[test]
fn parse_nested() {
    parses_to!("<red><green>test</green></red>", colored!(vec![text!("test", "green"), end!("green"), end!("red")]));
}

#[test]
fn parse_nested_more() {
    parses_to!("<red>red <green>green</green></red>", colored!(vec![text!("red ", "red"), text!("green", "green"), end!("green"), end!("red")]));
}

#[test]
fn parse_nested_then_unnested() {
    parses_to!("<red>red</red> nothing", colored!(vec![text!("red", "red"), end!("red"), text!(" nothing")]));
}
