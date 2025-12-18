use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Error};

mod ast;

#[cfg(feature = "debug")]
fn prettify(token_stream: proc_macro2::TokenStream) -> String {
    use syn::File;

    let file: File = syn::parse2::<File>(token_stream).unwrap();
    prettyplease::unparse(&file)
}

#[proc_macro_derive(SyntaxTree, attributes(production))]
pub fn impl_syn_tree(input: TokenStream) -> TokenStream {
    let parsed_input = parse_macro_input!(input as DeriveInput);
    let result = ast::generate_syn_tree(parsed_input).unwrap_or_else(Error::into_compile_error);

    #[cfg(feature = "debug")]
    if std::env::var_os("FABULIST_DEBUG").is_some_and(|env| env == "true") {
        println!("{}", prettify(result.clone()));
    }

    result.into()
}
