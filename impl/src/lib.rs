extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenStream as TokenStream2, TokenTree};
use quote::ToTokens;
use syn::{parse_macro_input, Data, DeriveInput, Meta, Variant};

#[macro_use]
extern crate quote;

#[proc_macro_derive(I18nError, attributes(i18n_key, i18n_delegate, i18n_language_codes))]
pub fn derive_i18n_error(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    derive(&input).into()
}

fn derive(input: &DeriveInput) -> TokenStream2 {
    let language_codes = get_language_codes(input);

    let i18n_error_type = get_enum_struct(input);

    match i18n_error_type {
        I18nErrorType::Struct(struct_struct) => {
            generate_impl_for_struct(struct_struct, language_codes)
        }
        I18nErrorType::Enum(enum_struct) => generate_impl_for_enum(enum_struct, language_codes),
    }
}

fn get_language_codes(input: &DeriveInput) -> Vec<Ident> {
    let language_codes = input
        .attrs
        .iter()
        .filter_map(|attr| match attr.meta.clone() {
            Meta::List(list) => {
                if list.path.is_ident("i18n_language_codes") {
                    let language_codes = list
                        .to_token_stream()
                        .into_iter()
                        .filter_map(|tk| match tk {
                            TokenTree::Group(ref group) => {
                                let stream = group.stream();
                                let language_codes = stream
                                    .into_iter()
                                    .filter_map(|s| match s {
                                        TokenTree::Ident(ref ident) => Some(ident.clone()),
                                        _ => None,
                                    })
                                    .collect::<Vec<_>>();
                                Some(language_codes)
                            }
                            _ => None,
                        })
                        .flatten()
                        .collect::<Vec<_>>();
                    if !language_codes.is_empty() {
                        Some(language_codes)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => {
                panic!("i18n_language_codes must be a list.");
            }
        })
        .flatten()
        .collect::<Vec<_>>();

    if language_codes.is_empty() {
        panic!("i18n_language_codes must be specified.");
    }

    language_codes
}

enum I18nErrorType {
    Struct(StructStruct),
    Enum(EnumStruct),
}

fn get_enum_struct(input: &DeriveInput) -> I18nErrorType {
    match &input.data {
        Data::Struct(_) => {
            let token_streams = input
                .attrs
                .iter()
                .filter_map(|attr| match attr.meta.clone() {
                    Meta::List(list) => {
                        if list.path.is_ident("i18n_key") {
                            Some(list.tokens.clone())
                        } else {
                            None
                        }
                    }
                    _ => {
                        panic!("i18n_error must be a list.");
                    }
                })
                .collect::<Vec<TokenStream2>>();

            if token_streams.len() != 1 {
                panic!("i18n_error must be specified only once as Struct attribute.");
            }
            let token_stream = token_streams.first().unwrap();

            let struct_struct = StructStruct {
                ident: input.ident.clone(),
                i18n_key: token_stream.clone(),
            };

            I18nErrorType::Struct(struct_struct)
        }
        Data::Enum(data) => {
            let enum_variants: Vec<EnumVariant> = data
                .variants
                .iter()
                .map(|variant| {
                    let mut i18n_key = None;
                    let mut i18n_delegate = false;
                    variant.attrs.iter().for_each(|attr| {
                        let meta = &attr.meta.clone();
                        match meta {
                            Meta::List(list) => {
                                if list.path.is_ident("i18n_key") {
                                    i18n_key = Some(list.tokens.clone());
                                }
                            }
                            Meta::Path(path) => {
                                if path.is_ident("i18n_delegate") {
                                    i18n_delegate = true;
                                }
                            }
                            _ => {
                                panic!("i18n_key must be a list. i18n_delegate must a path.");
                            }
                        };
                    });
                    EnumVariant {
                        variant: variant.clone(),
                        i18n_key_delegate: match (i18n_key, i18n_delegate) {
                            (Some(_), true) => {
                                panic!("Cannot specify both i18n_key and i18n_delegate.")
                            }
                            (Some(key), false) => I18nKeyDelegate::I18nKey(key),
                            (None, true) => I18nKeyDelegate::Delegate,
                            (None, false) => {
                                panic!("Either i18n_key or i18n_delegate must be specified.")
                            }
                        },
                    }
                })
                .collect();

            I18nErrorType::Enum(EnumStruct {
                ident: input.ident.clone(),
                enum_variants,
            })
        }
        Data::Union(_) => panic!("I18nError is only supported for Enum."),
    }
}

fn generate_impl_for_enum(enum_struct: EnumStruct, language_codes: Vec<Ident>) -> TokenStream2 {
    let ident = enum_struct.ident.clone();
    let inner_function = enum_struct
        .enum_variants
        .clone()
        .into_iter()
        .map(|enum_variant| generate_inner_function(enum_variant, language_codes.clone()))
        .collect::<Vec<_>>();

    // FIXME: language_code: Into<LanguageCode> とすることで、わざわざユーザーに変換を意識せずに使えるようにする
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

// FIXME: struct に対応する場合、generic を対応する必要がある
fn generate_impl_for_struct(
    struct_struct: StructStruct,
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

#[derive(Debug, Clone)]
struct StructStruct {
    ident: syn::Ident,
    i18n_key: TokenStream2,
}

#[derive(Debug, Clone)]
struct EnumStruct {
    ident: syn::Ident,
    enum_variants: Vec<EnumVariant>,
}

#[derive(Debug, Clone)]
struct EnumVariant {
    variant: Variant,
    i18n_key_delegate: I18nKeyDelegate,
}

#[derive(Debug, Clone)]
enum I18nKeyDelegate {
    I18nKey(TokenStream2),
    Delegate,
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
