use std::{fs, io::Read};

use gag::BufferRedirect;

macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);

        // Find and cut the rest of the path
        match &name[..name.len() - 3].rfind(':') {
            Some(pos) => &name[pos + 1..name.len() - 3],
            None => &name[..name.len() - 3],
        }
    }};
}

macro_rules! test {
    ($input: literal) => {
        let function_name = function!();
        let expected =
            fs::read_to_string(format!("src/output/{}.stdout", function_name))
            .unwrap()
            .replace("\\n", "\n")
            .replace("\\u{1b}", "\u{1b}");

        let mut buf = BufferRedirect::stdout().unwrap();
        println!("{}", colored_macro::colored!($input));
        let mut actual = String::new();
        buf.read_to_string(&mut actual).unwrap();
        assert_eq!(expected, actual);
        drop(buf);
    };
}

#[test]
fn test_no_colours() {
    test!("Hello, world!");
}

#[test]
fn test_single_colour() {
    test!("Hello, <red>world</red>!");
}
