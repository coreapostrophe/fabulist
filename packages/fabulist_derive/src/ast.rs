use proc_macro2::{TokenStream, TokenTree};
use quote::{quote, quote_spanned};
use syn::{
    parse2, parse_str, spanned::Spanned, Attribute, Data, DataEnum, DeriveInput, Error, Ident,
    Meta, Result, TypePath, Variant, Visibility,
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
    let tokens_vec: Vec<TokenTree> = meta_tokens.clone().into_iter().collect();

    let token_pairs = tokens_vec
        .split(|token_tree| token_tree.to_string().as_str() == ",")
        .peekable();

    let mut token_tuples: Vec<(Ident, TypePath)> = Vec::new();

    for token_pair in token_pairs {
        let mut pair_iter = token_pair.split(|token| token.to_string().as_str() == ":");
        let Some(ident_token_slice) = pair_iter.next() else {
            return Err(Error::new(
                attr_ident.span(),
                "Expected a syntax property definition. (i.e. #[production(left: Expr)] )",
            ));
        };
        let Some(type_token_slice) = pair_iter.next() else {
            return Err(Error::new(
                attr_ident.span(),
                "Expected a syntax property definition. (i.e. #[production(left: Expr)] )",
            ));
        };
        let ident_token_stream = TokenStream::from_iter(ident_token_slice.to_vec());
        let type_token_stream = TokenStream::from_iter(type_token_slice.to_vec());

        let ident =
            parse2::<Ident>(quote_spanned! { ident_token_stream.span() => #ident_token_stream })
                .map_err(|_| Error::new(ident_token_stream.span(), "Expected an identifier"))?;

        let type_path =
            parse2::<TypePath>(quote_spanned! { type_token_stream.span() => #type_token_stream })
                .map_err(|_| Error::new(type_token_stream.span(), "Expected a type path"))?;

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
