//! Proc macros for derive_try_from
//!
//! Do not link to this.
//! This library is solely to provide proc macros.
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

/// Generates TryFrom for each variant of enum.
#[proc_macro_derive(TryFromVariants)]
pub fn try_from_variants(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let output = from_variants_proc(input, "TryFromVariants", &try_from_quote);

    output.into()
}

/// Generates From for each variant of enum.
#[proc_macro_derive(FromVariants)]
pub fn from_variants(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let output = from_variants_proc(input, "FromVariants", &variant_from_quote);

    output.into()
}

fn from_variants_proc<F>(input: DeriveInput, macro_name: &str, quote_fn: F) -> TokenStream
where
    F: Fn(&syn::Ident, &syn::Ident, &syn::Field) -> TokenStream,
{
    let input_name = input.ident;

    let stream = match input.data {
        Data::Enum(enum_data) => {
            let mut stream = TokenStream::new();
            for variant in enum_data.variants {
                stream.extend(generate_variant_froms(
                    &input_name,
                    &variant,
                    macro_name,
                    &quote_fn,
                ));
            }
            stream
        }
        _ => panic!(
            "Derive {} for {} failed. Must be an enum.",
            macro_name, input_name
        ),
    };

    stream
}

fn generate_variant_froms<F>(
    enum_name: &syn::Ident,
    variant: &syn::Variant,
    macro_name: &str,
    quote_fn: F,
) -> TokenStream
where
    F: Fn(&syn::Ident, &syn::Ident, &syn::Field) -> TokenStream,
{
    let member_data = match &variant.fields {
        Fields::Unnamed(fields) => &fields.unnamed,
        _ => panic!(
            "{} requires only unamed members, failed {}::{}",
            macro_name, enum_name, variant.ident
        ),
    };
    let variant = &variant.ident;
    let wrapped_type = member_data.first().unwrap();

    quote_fn(enum_name, variant, wrapped_type)
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

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::TokenStream;
    use std::str::FromStr;
    use syn::{parse2, DeriveInput};

    #[test]
    #[should_panic(expected = "Derive TryFromVariants for NotEnum failed. Must be an enum.")]
    fn fails_for_struct() {
        let struct_tokens: TokenStream = TokenStream::from_str("struct NotEnum;").unwrap();
        let struct_tokens: DeriveInput = parse2(struct_tokens).unwrap();
        from_variants_proc(struct_tokens, "TryFromVariants", &try_from_quote);
    }

    #[test]
    #[should_panic(expected = "Derive TryFromVariants for NotEnum failed. Must be an enum.")]
    fn fails_for_union_proc() {
        let union_tokens = TokenStream::from_str("union NotEnum { a: u32, b: f32, }").unwrap();
        let union_tokens: DeriveInput = parse2(union_tokens).unwrap();
        from_variants_proc(union_tokens, "TryFromVariants", &try_from_quote);
    }

    #[test]
    #[should_panic(expected = "TryFromVariants requires only unamed members, failed Dewey::Frank")]
    fn fails_for_non_unnamed_enums() {
        let enum_tokens = TokenStream::from_str("enum Dewey { Frank, Ernest(bool), }").unwrap();
        let enum_tokens: DeriveInput = parse2(enum_tokens).unwrap();
        from_variants_proc(enum_tokens, "TryFromVariants", &try_from_quote);
    }
}
