//! This crate implements the macro for `colored_macro` and should not be used directly.

extern crate proc_macro;

use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro]
/// colored_macro is a macro that allows you to write colored text in your terminal using XML-like syntax.
/// ```no_run no_test
/// use colored_macro::colored_macro;
/// 
/// fn main() {
///    println!("{}", colored_macro!("<red>Hello, <green>world</green>!</red>"));
/// }
/// ```
pub fn colored_macro(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as proc_macro2::TokenStream);

    match colored_macro_impl::colored_macro(item) {
        Ok(tokens) => tokens.into(),
        Err(err) => TokenStream::from(err.to_compile_error()),
    }
}
