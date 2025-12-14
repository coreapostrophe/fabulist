use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, Parser},
    parse_str,
    punctuated::Punctuated,
    spanned::Spanned,
    Attribute, Data, DataEnum, DeriveInput, Error, Field, Ident, Meta, Result, Token, Variant,
    Visibility,
};

pub fn generate_syn_tree(input: DeriveInput) -> Result<TokenStream> {
    let span = &input.span();
    let input_ident = &input.ident;
    let input_vis = &input.vis;
    let data = &input.data;
    match data {
        Data::Enum(data_enum) => build_structs(input_ident, input_vis, data_enum),
        _ => Err(Error::new(
            span.to_owned(),
            "Only the `Enum` data type is supported",
        )),
    }
}

fn build_structs(
    input_ident: &Ident,
    input_vis: &Visibility,
    data_enum: &DataEnum,
) -> Result<TokenStream> {
    let structs = data_enum
        .variants
        .iter()
        .map(|variant| build_struct(input_ident, input_vis, variant))
        .collect::<Result<Vec<TokenStream>>>()?;
    Ok(quote! { #(#structs)* })
}

fn build_struct(
    input_ident: &Ident,
    input_vis: &Visibility,
    variant: &Variant,
) -> Result<TokenStream> {
    let variant_ident = &variant.ident;
    let production_attr = variant
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("production"));

    let passthrough_attrs: Vec<&Attribute> = variant
        .attrs
        .iter()
        .filter(|attr| !attr.path().is_ident("production"))
        .collect();

    let formatted_ident: Ident = parse_str(&format!(
        "{}{}",
        quote! {#variant_ident},
        quote! {#input_ident}
    ))?;

    match production_attr {
        Some(attr) => {
            let fields = build_fields(attr)?;
            let struct_doc = format!(
                "SyntaxTree-generated AST node for `{}`::`{}`.",
                input_ident, variant_ident
            );
            Ok(quote! {
                #(#passthrough_attrs)*
                #[doc = #struct_doc]
                #[derive(std::fmt::Debug, core::clone::Clone)]
                #input_vis struct #formatted_ident {
                    #fields
                }
            })
        }
        None => Ok(TokenStream::new()),
    }
}

struct ParsableField {
    pub value: Field,
}

impl Parse for ParsableField {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let field = Field::parse_named(input)?;
        Ok(Self { value: field })
    }
}

fn build_fields(attr: &Attribute) -> Result<TokenStream> {
    let attr_span = &attr.span();
    let meta = &attr.meta;
    match meta {
        Meta::List(list_meta) => {
            let meta_tokens = &list_meta.tokens;

            let field_parser = Punctuated::<ParsableField, Token![,]>::parse_separated_nonempty;
            let tokens_vec = field_parser.parse2(meta_tokens.clone())?;

            let field = tokens_vec
                .iter()
                .map(|field| {
                    let field_value = &field.value;
                    let name = field_value
                        .ident
                        .as_ref()
                        .map(|ident| ident.to_string())
                        .unwrap_or_else(|| "field".to_string());
                    let field_doc =
                        format!("Generated field `{}` from a SyntaxTree production.", name);
                    Ok(quote! {
                        #[doc = #field_doc]
                        pub #field_value,
                    })
                })
                .collect::<Result<Vec<TokenStream>>>()?;

            Ok(quote! { #(#field)* })
        }
        Meta::Path(_) => Ok(TokenStream::new()),
        Meta::NameValue(_) => Err(Error::new(
            attr_span.to_owned(),
            "named value meta-type is not supported.",
        )),
    }
}
