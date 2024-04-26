use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    parse_str, spanned::Spanned, Data, DeriveInput, Error, Expr, Field, Fields, GenericArgument,
    Ident, Index, Member, PathArguments, Result, Type, TypePath,
};

pub fn generate_element(fab_ident: &str, input: DeriveInput) -> Result<TokenStream> {
    let data_ident = &input.ident;
    let fab_ident: Expr = parse_str(fab_ident).map_err(|_e| {
        Error::new(
            data_ident.span(),
            "Failed to parse `fabulist_core` identifier.",
        )
    })?;
    let data_struct = match &input.data {
        Data::Struct(data_struct) => data_struct,
        _ => {
            return Err(Error::new(
                data_ident.span(),
                "`Element` can only be derived from structs.",
            ))
        }
    };
    let valid_fields = filtered_fields(&data_struct.fields)?;
    let inset_field_impl = build_inset_getter_setters(data_ident, &valid_fields)?;
    let inset_interp_assignments = build_inset_interp_assignments(&valid_fields)?;

    Ok(quote! {
        impl #fab_ident ::story::part::Element for #data_ident {}
        #inset_field_impl
        impl #fab_ident ::story::resource::InterpInset for #data_ident {
            fn interp_inset(
                &mut self,
                resources: &mut #fab_ident ::story::resource::Resources
            ) { #inset_interp_assignments }
        }
    })
}

fn build_inset_getter_setters(data_ident: &Ident, fields: &Vec<&Field>) -> Result<TokenStream> {
    let getters_setters: Vec<TokenStream> = fields
        .iter()
        .enumerate()
        .filter_map(|(_, field)| {
            let ident: Ident = match &field.ident {
                Some(ident) => ident.clone(),
                _ => return None,
            };

            let field_ty = &field.ty;
            let setter_ident: Ident = parse_str(&format!("set_{}", quote! {#ident}.to_string()))
                .expect("Failed to create setter identifier.");

            Some(quote_spanned! { field.span() =>
                pub fn #ident(&self) -> &#field_ty {
                    &self.#ident
                }
                pub fn #setter_ident(&mut self, id: impl Into<String>) {
                    self.#ident.set_id(id);
                }
            })
        })
        .collect();

    if !getters_setters.is_empty() {
        Ok(quote! { impl #data_ident { #(#getters_setters)* } })
    } else {
        Ok(quote! {})
    }
}

fn build_inset_interp_assignments(fields: &Vec<&Field>) -> Result<TokenStream> {
    let assignments: Vec<TokenStream> = fields
        .iter()
        .enumerate()
        .filter_map(|(idx, field)| {
            let ident: Member = match &field.ident {
                Some(ident) => Member::Named(ident.clone()),
                _ => Member::Unnamed(Index {
                    index: idx as u32,
                    span: field.span(),
                }),
            };

            let generic_ty = match type_generic(field.span(), &field.ty) {
                Ok(generic_ty) => generic_ty,
                _ => return None,
            };

            Some(quote_spanned! { field.span() =>
                let resource = resources.get::<#generic_ty>(self.#ident.id());
                self.#ident.set_value(resource.clone());
            })
        })
        .collect();

    Ok(quote! { #(#assignments)* })
}

fn filtered_fields(fields: &Fields) -> Result<Vec<&Field>> {
    let filter_closure = |field: &&Field| -> bool {
        let field_ty: &TypePath = match &field.ty {
            Type::Path(type_path) => type_path,
            _ => return false,
        };
        let valid_segment = &field_ty
            .path
            .segments
            .iter()
            .find(|segment| segment.ident == "Inset");
        valid_segment.is_some()
    };

    match fields {
        Fields::Named(named_fields) => {
            Ok(named_fields.named.iter().filter(filter_closure).collect())
        }
        Fields::Unnamed(unnamed_fields) => Ok(unnamed_fields
            .unnamed
            .iter()
            .filter(filter_closure)
            .collect()),
        Fields::Unit => Ok(vec![]),
    }
}

fn type_generic(span: Span, ty: &Type) -> Result<&GenericArgument> {
    if let Type::Path(type_path) = ty {
        if let Some(path_segement) = type_path.path.segments.last() {
            if let PathArguments::AngleBracketed(angle_bracketed_path) = &path_segement.arguments {
                if let Some(generic_ty) = &angle_bracketed_path.args.first() {
                    return Ok(generic_ty);
                }
            }
        }
    }
    Err(Error::new(
        span,
        format!(
            "Failed to parse generic arguments of type `{}`.",
            ty.to_token_stream()
        ),
    ))
}
