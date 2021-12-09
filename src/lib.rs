//! This macro library is currently unstable, it works for it's role in Nostromo
//! Engine, but is not in the state for general use.
//! It may fail in many ways & it currently requires linkage to Nostromo for
//! the error type.
//! Use at your own risk.
//! Actually, your probably better off just not using it at all.
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

/// !!Unstable!! Generates TryFrom for each variant of enum.
///
/// The type needs to provide a as_ref() -> impl Str for for this derivation to
/// succeed.
/// I recommend using strum_macro::AsRefStr.
/// # Warning
/// Note that this only works for enums composed solely of 1 unnamed variant.
/// If it finds a single one that does not follow these requirements, it fails.
/// Furthermore, the return type is nostromo_engine::StackException.
#[proc_macro_derive(TryFromVariants)]
pub fn try_from_variants(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let output = tryfrom_variants(input);

    output.into()
}

fn tryfrom_variants(input: DeriveInput) -> TokenStream {
    let input_name = input.ident;

    let stream = match input.data {
        Data::Enum(enum_data) => {
            let mut stream = TokenStream::new();
            for variant in enum_data.variants {
                stream.extend(generate_variant_tryfrom(&input_name, &variant));
            }
            stream
        }
        _ => panic!(
            "Derive TryFromVariants for {} failed. Must be an enum.",
            input_name
        ),
    };

    stream
}

fn generate_variant_tryfrom(enum_name: &syn::Ident, variant: &syn::Variant) -> TokenStream {
    let member_data = match &variant.fields {
        Fields::Unnamed(fields) => &fields.unnamed,
        _ => panic!(
            "TryFrom requires only unamed members, failed {}::{}",
            enum_name, variant.ident
        ),
    };
    let variant = variant.ident.clone();
    let wrapped_type = member_data.first();

    let stream = quote! {
    impl TryFrom<#enum_name> for #wrapped_type {
        type Error = StackException;

        fn try_from(value: #enum_name) -> std::result::Result<Self, Self::Error> {
            match value {
                #enum_name::#variant(n) => Ok(n),
                _ => Err(StackException::InvalidStackValue {
                    exp_type: concat!(stringify!(#enum_name), "::", stringify!(#variant)).to_string(),
                    act_type: format!(concat!(stringify!(#enum_name), "::{}"), value.as_ref()),
                }),
            }
        }
    }
        };

    stream
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
        tryfrom_variants(struct_tokens);
    }

    #[test]
    #[should_panic(expected = "Derive TryFromVariants for NotEnum failed. Must be an enum.")]
    fn fails_for_union() {
        let union_tokens = TokenStream::from_str("union NotEnum { a: u32, b: f32, }").unwrap();
        let union_tokens: DeriveInput = parse2(union_tokens).unwrap();
        tryfrom_variants(union_tokens);
    }

    #[test]
    #[should_panic(expected = "TryFrom requires only unamed members, failed Dewey::Frank")]
    fn fails_for_non_unnamed_enums() {
        let enum_tokens = TokenStream::from_str("enum Dewey { Frank, Ernest(bool), }").unwrap();
        let enum_tokens: DeriveInput = parse2(enum_tokens).unwrap();
        tryfrom_variants(enum_tokens);
    }
}
