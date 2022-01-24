//! Proc macros for derive_try_from
//!
//! Do not link to this.
//! This library is solely to provide proc macros.
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use std::collections::HashMap;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

/// Generates TryFrom for each variant of enum.
#[proc_macro_derive(TryFromVariants)]
pub fn try_from_variants(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let output = from_variants_proc(input, "TryFromVariants", &try_from_quote);

    output.unwrap_or_else(syn::Error::into_compile_error).into()
}

/// Generates From for each variant of enum.
#[proc_macro_derive(FromVariants)]
pub fn from_variants(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let output = from_variants_proc(input, "FromVariants", &variant_from_quote);

    output.unwrap_or_else(syn::Error::into_compile_error).into()
}

fn from_variants_proc<F>(
    input: DeriveInput,
    macro_name: &str,
    quote_fn: F,
) -> syn::Result<TokenStream>
where
    F: Fn(&syn::Ident, &syn::Ident, &syn::Field) -> TokenStream,
{
    let input_name = &input.ident;

    match input.data {
        Data::Enum(enum_data) => {
            let mut stream = TokenStream::new();
            let mut implemented_types = HashMap::new();
            for variant in enum_data.variants {
                stream.extend(generate_variant_froms(
                    input_name,
                    &variant,
                    macro_name,
                    &quote_fn,
                    &mut implemented_types,
                )?);
            }
            Ok(stream)
        }
        _ => Err(syn::Error::new(
            input.span(),
            format!("Derive {macro_name} for {input_name} failed. Must be an enum."),
        )),
    }
}

fn generate_variant_froms<F>(
    enum_name: &syn::Ident,
    variant: &syn::Variant,
    macro_name: &str,
    quote_fn: F,
    implemented_types: &mut HashMap<syn::Field, syn::Ident>,
) -> syn::Result<TokenStream>
where
    F: Fn(&syn::Ident, &syn::Ident, &syn::Field) -> TokenStream,
{
    let member_data = match &variant.fields {
        Fields::Unnamed(fields) => &fields.unnamed,
        _ => {
            return Err(syn::Error::new(
                variant.fields.span(),
                format!(
                    "{} can only use unamed members, failed for {}::{}",
                    macro_name, enum_name, variant.ident
                ),
            ))
        }
    };
    // Check only contains 1 element
    if member_data.len() != 1 {
        return Err(syn::Error::new(
            member_data.span(),
            format!(
                "{} failed for {}::{}, does not currently support variants that are larger than 1",
                macro_name, enum_name, variant.ident
            ),
        ));
    }

    let variant = &variant.ident;
    let wrapped_type = member_data.first().unwrap();

    if let Some(_original) = implemented_types.insert(wrapped_type.clone(), variant.clone()) {
        Err(syn::Error::new(
            member_data.span(),
            format!(
                "Repeat of variant for type {} in variant {}::{}. Already defined with {}::{}",
                wrapped_type.ty.to_token_stream(),
                enum_name,
                variant,
                enum_name,
                "blarg"
            ),
        ))
    } else {
        Ok(quote_fn(enum_name, variant, wrapped_type))
    }
}

fn try_from_quote(
    enum_name: &syn::Ident,
    variant: &syn::Ident,
    wrapped_type: &syn::Field,
) -> TokenStream {
    quote! {
    impl TryFrom<#enum_name> for #wrapped_type {
        type Error = ::enum_variant_macros::VariantCastError;

        fn try_from(value: #enum_name) -> ::std::result::Result<Self, Self::Error> {
            match value {
                #enum_name::#variant(n) => Ok(n),
                _ => Err(::enum_variant_macros::VariantCastError {
                    enum_type: stringify!(#enum_name),
                    exp_type: stringify!(#wrapped_type),
                    variant_name: value.into(),
                }),
            }
        }
    }
    }
}

fn variant_from_quote(
    enum_name: &syn::Ident,
    variant: &syn::Ident,
    wrapped_type: &syn::Field,
) -> TokenStream {
    quote! {
        impl From<#wrapped_type> for #enum_name {
            fn from(value: #wrapped_type) -> #enum_name {
                #enum_name::#variant(value)
            }
        }
    }
}
