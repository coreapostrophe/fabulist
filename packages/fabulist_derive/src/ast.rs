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
    let attrs = variant.attrs.first();

    let formatted_ident: Ident = parse_str(&format!(
        "{}{}",
        quote! {#variant_ident},
        quote! {#input_ident}
    ))?;
    let fields = match attrs {
        Some(attr) => build_fields(attr),
        None => Ok(TokenStream::new()),
    }?;
    Ok(quote! {
        #[derive(std::fmt::Debug, core::clone::Clone)]
        #input_vis struct #formatted_ident {
            #fields
        }
    })
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
    let Meta::List(list_meta) = meta else {
        return Err(Error::new(
            attr_span.to_owned(),
            "Only list meta type is supported.",
        ));
    };
    let meta_tokens = &list_meta.tokens;

    let field_parser = Punctuated::<ParsableField, Token![,]>::parse_separated_nonempty;
    let tokens_vec = field_parser.parse2(meta_tokens.clone())?;

    let field = tokens_vec
        .iter()
        .map(|field| {
            let field_value = &field.value;
            quote! { pub #field_value, }
        })
        .collect::<Vec<TokenStream>>();

    Ok(quote! { #(#field)* })
}
