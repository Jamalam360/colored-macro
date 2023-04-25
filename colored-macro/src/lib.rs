use colored_macro_impl::{process, Colored};
use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro]
pub fn colored(input: TokenStream) -> TokenStream {
    process(parse_macro_input!(input as Colored)).into()
}
