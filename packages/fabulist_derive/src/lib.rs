use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Error, File};

mod ast;
mod element;

const CORE_CRATE_NAME: &str = "fabulist_core";
const CORE_CRATE_NAME_INTERNAL: &str = "crate";

#[allow(dead_code)]
#[cfg(debug_assertions)]
fn prettify(token_stream: proc_macro2::TokenStream) -> String {
    let file: File = syn::parse2::<File>(token_stream).unwrap();
    prettyplease::unparse(&file)
}

#[proc_macro_derive(Element)]
pub fn impl_element(input: TokenStream) -> TokenStream {
    let parsed_input = parse_macro_input!(input as DeriveInput);
    let result = element::generate_element(CORE_CRATE_NAME, parsed_input)
        .unwrap_or_else(Error::into_compile_error);

    #[cfg(debug_assertions)]
    #[cfg(feature = "debug")]
    println!("{}", prettify(result.clone()));

    result.into()
}

#[proc_macro_derive(ElementInternal)]
pub fn impl_element_internal(input: TokenStream) -> TokenStream {
    let parsed_input = parse_macro_input!(input as DeriveInput);
    let result = element::generate_element(CORE_CRATE_NAME_INTERNAL, parsed_input)
        .unwrap_or_else(Error::into_compile_error);

    #[cfg(debug_assertions)]
    #[cfg(feature = "debug")]
    println!("{}", prettify(result.clone()));

    result.into()
}

#[proc_macro_derive(SyntaxTree, attributes(production))]
pub fn impl_syn_tree(input: TokenStream) -> TokenStream {
    let parsed_input = parse_macro_input!(input as DeriveInput);
    let result = ast::generate_syn_tree(parsed_input).unwrap_or_else(Error::into_compile_error);

    #[cfg(debug_assertions)]
    #[cfg(feature = "debug")]
    println!("{}", prettify(result.clone()));

    result.into()
}
