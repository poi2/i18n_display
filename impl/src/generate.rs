use crate::parse::get_language_codes;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use syn::{DeriveInput, Variant};

use crate::parse::{
    get_derive_target, DeriveTarget, EnumVariant, ErrorOfEnum, ErrorOfStruct, I18nKeyDelegate,
};

pub(crate) fn generate(input: &DeriveInput) -> TokenStream2 {
    let language_codes = get_language_codes(input);

    let derive_target = get_derive_target(input);

    match derive_target {
        DeriveTarget::Struct(struct_struct) => {
            generate_impl_for_struct(struct_struct, language_codes)
        }
        DeriveTarget::Enum(enum_struct) => generate_impl_for_enum(enum_struct, language_codes),
    }
}

fn generate_impl_for_enum(enum_struct: ErrorOfEnum, language_codes: Vec<Ident>) -> TokenStream2 {
    let ident = enum_struct.ident.clone();
    let inner_function = enum_struct
        .enum_variants
        .clone()
        .into_iter()
        .map(|enum_variant| generate_inner_function(enum_variant, language_codes.clone()))
        .collect::<Vec<_>>();

    // FIXME: Change to `language_code: Into<LanguageCode>`.
    quote! {
        impl ToI18nString for #ident {
            fn to_i18n_string(&self, language_code: LanguageCode) -> String {
                use rust_i18n::t;

                match self {
                    #(#inner_function)*
                }
            }
        }
    }
}

// FIXME: Currently, only simple struct is supported. Generic struct is not supported.
fn generate_impl_for_struct(
    struct_struct: ErrorOfStruct,
    language_codes: Vec<Ident>,
) -> TokenStream2 {
    let ident = struct_struct.ident.clone();
    let i18n_key = struct_struct.i18n_key.clone();

    let language_code_match = language_codes
        .iter()
        .map(|language_code| {
            quote! {
                LanguageCode::#language_code => {
                    t!(#i18n_key, locale = LanguageCode::#language_code.to_string().as_str()).to_string()
                },
            }
        })
        .collect::<Vec<TokenStream2>>();

    // FIXME: Change to `language_code: Into<LanguageCode>`.
    quote! {
        impl ToI18nString for #ident {
            fn to_i18n_string(&self, language_code: LanguageCode) -> String {
                use rust_i18n::t;

                match language_code {
                    #(#language_code_match)*
                    _ => {
                        panic!("Unsupported language code: {:?}", language_code);
                    }
                }
            }
        }
    }
}

fn generate_inner_function(enum_variant: EnumVariant, language_codes: Vec<Ident>) -> TokenStream2 {
    match enum_variant.i18n_key_delegate {
        I18nKeyDelegate::I18nKey(key) => {
            generate_i18n_inner_function(enum_variant.variant, key, language_codes)
        }
        I18nKeyDelegate::Delegate => generate_delegate_inner_function(enum_variant.variant),
    }
}

fn generate_i18n_inner_function(
    variant: Variant,
    key: TokenStream2,
    language_codes: Vec<Ident>,
) -> TokenStream2 {
    let ident = variant.ident;
    let language_code_match = language_codes
        .iter()
        .map(|language_code| {
            quote! {
                LanguageCode::#language_code => {
                    t!(#key, locale = LanguageCode::#language_code.to_string().as_str()).to_string()
                },
            }
        })
        .collect::<Vec<TokenStream2>>();

    match variant.fields {
        syn::Fields::Named(_) => {
            let field_token_stream = variant
                .fields
                .iter()
                .map(|field| {
                    let field_ident = field.ident.clone().unwrap();
                    quote! { #field_ident, }
                })
                .collect::<Vec<TokenStream2>>();

            quote! {
                Self::#ident { #(#field_token_stream)* } => match language_code {
                    #(#language_code_match)*
                    _ => {
                        panic!("Unsupported language code: {:?}", language_code);
                    }
                }
            }
        }
        syn::Fields::Unnamed(_) => {
            let field_token_stream: Vec<TokenStream2> =
                variant.fields.iter().map(|_| quote! { _, }).collect();

            quote! {
                Self::#ident(#(#field_token_stream)*) => match language_code {
                    #(#language_code_match)*
                    _ => {
                        panic!("Unsupported language code: {:?}", language_code);
                    }
                }
            }
        }
        syn::Fields::Unit => {
            quote! {
                Self::#ident => match language_code {
                    #(#language_code_match)*
                    _ => {
                        panic!("Unsupported language code: {:?}", language_code);
                    }
                }
            }
        }
    }
}

fn generate_delegate_inner_function(variant: Variant) -> TokenStream2 {
    let ident = variant.ident;
    match variant.fields {
        syn::Fields::Named(_) => {
            panic!("Struct variant is not supported for i18n_delegate");
        }
        syn::Fields::Unnamed(_) => {
            if variant.fields.len() > 1 {
                panic!("Tuple variant with multiple fields is not supported for i18n_delegate");
            }
            quote! {
                Self::#ident(inner) => inner.to_i18n_string(language_code),
            }
        }
        syn::Fields::Unit => {
            panic!("Unit variant is not supported for i18n_delegate");
        }
    }
}
