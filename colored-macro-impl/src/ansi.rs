use css_color::Srgb;

pub const RESET: &str = "\x1b[0m";

/// Converts a style like `red`, `bold`, or `#ff0000` to an ANSI code.
/// Non-text styles like `bold` and `underline` are handled manually, while colors are handled according to the CSS spec.
pub fn style_to_ansi_code(style: String) -> String {
    match style.parse::<Srgb>() {
        Ok(color) => {
            if color.alpha != 1.0 {
                panic!("Color must have alpha value of 1, found {}", color.alpha);
            }

            format!(
                "\x1b[38;2;{};{};{}m",
                (color.red * 255.0) as u8,
                (color.green * 255.0) as u8,
                (color.blue * 255.0) as u8
            )
        }
        Err(_) => match style.as_str() {
            "reset" => RESET,
            "bold" => "\x1b[1m",
            "dim" => "\x1b[2m",
            "italic" => "\x1b[3m",
            "underline" => "\x1b[4m",
            "blink" => "\x1b[5m",
            "reverse" => "\x1b[7m",
            "hidden" => "\x1b[8m",
            "strikethrough" => "\x1b[9m",
            _ => panic!("Unknown style: {}", style),
        }
        .to_string(),
    }
}
