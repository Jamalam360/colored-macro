# Colored Macro

Colored macro is a Rust crate for creating colored and formatted strings using ANSI codes in a readable way. It uses XML-like syntax:

`<red>red text</red> <bold>bold text</bold> <green>{fmt_expr}</green>`

## The Problem

With current crates, messages quickly become hard to quickly understand at a glance. This is best explained with an example:

```rust
println!("[{}] ({}): {}", level.green(), timestamp.blue(), message.bold());
```

You can see that even with a simple message, the format string becomes hard to quickly understand, compared to the equivalent using this crate:

```rust
println!("{}", colored!("[<green>{level}</green>] (<blue>{timestamp}</blue>): <bold>{message}</bold>"));
```

## Documentation

The following styles are available:

* Any [CSS named color](https://www.w3.org/wiki/CSS/Properties/color/keywords), as long as their alpha is 1.
* Any RGB color in the regular CSS format, as long as its alpha is 1 (e.g. `rgb(10, 12, 200)`, `#0012G4`).
* `reset` (resets current styles)
* `bold`
* `dim`
* `italic`
* `underline`
* `blink`
* `reverse`
* `hidden`
* `strikethrough`

Styles can be nested:

```rust
colored_macro::colored!("<red>all red <yellow>yellow <blue>now to blue</blue> back to yellow</yellow> back to red</red>")
```

![Output of above code](https://user-images.githubusercontent.com/56727311/233061615-998b3d66-2457-4f8b-8075-6c71489b0edd.png)
