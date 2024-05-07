use proc_macro2::{TokenStream, TokenTree};
use quote::{quote, quote_spanned};
use syn::{
    parse::Parse, parse2, parse_str, spanned::Spanned, Attribute, Data, DataEnum, DeriveInput,
    Error, Ident, Meta, Result, TypePath, Variant, Visibility,
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
    let variant_span = &variant.span();
    let variant_ident = &variant.ident;
    let attrs = variant.attrs.first();

    let formatted_ident: Ident = parse_str(&format!(
        "{}{}",
        quote! {#variant_ident},
        quote! {#input_ident}
    ))?;
    let fields = match attrs {
        Some(attr) => build_fields(attr),
        None => Err(Error::new(
            variant_span.to_owned(),
            "Unable to find `production` attribute",
        )),
    }?;
    Ok(quote! {
        #[derive(std::fmt::Debug)]
        #input_vis struct #formatted_ident {
            #fields
        }
    })
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
    let attr_ident = &list_meta.path;
    let meta_tokens = &list_meta.tokens;
    let mut tokens_iter = meta_tokens.clone().into_iter().peekable();
    let mut token_tuples: Vec<(Ident, TypePath)> = Vec::new();

    while tokens_iter.peek().is_some() {
        let ident = parse_token_tree::<Ident>(&mut tokens_iter)
            .map_err(|_| Error::new(attr_ident.span(), "Expected an identifier"))?;

        match_token_tree(&mut tokens_iter, ":").map_err(|_| {
            Error::new(
                attr_ident.span(),
                "Expected a `:` after the property identifier",
            )
        })?;

        let type_path = parse_token_tree::<TypePath>(&mut tokens_iter)
            .map_err(|_| Error::new(attr_ident.span(), "Expected a type path after `:`"))?;

        if tokens_iter.peek().is_some() {
            match_token_tree(&mut tokens_iter, ",")
                .map_err(|_| Error::new(attr_ident.span(), "Expected a `,`"))?;
        }

        token_tuples.push((ident, type_path));
    }

    let field = token_tuples
        .iter()
        .map(|tuple| {
            let (field_ident, field_type) = tuple;
            quote! {
                pub #field_ident : #field_type,
            }
        })
        .collect::<Vec<TokenStream>>();

    Ok(quote! { #(#field)* })
}

fn match_token_tree(
    iterator: &mut impl Iterator<Item = TokenTree>,
    str_compare: &str,
) -> std::result::Result<(), ()> {
    if let Some(comma_token) = iterator.next() {
        if comma_token.to_string().as_str() != str_compare {
            return Err(());
        };
    }
    Ok(())
}

fn parse_token_tree<T: Parse>(
    iterator: &mut impl Iterator<Item = TokenTree>,
) -> std::result::Result<T, ()> {
    if let Some(token) = iterator.next() {
        if let Ok(parsed_syntax) = parse2::<T>(quote_spanned! { token.span() => #token }) {
            return Ok(parsed_syntax);
        }
    };
    Err(())
}
