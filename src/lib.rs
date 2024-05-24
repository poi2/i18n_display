extern crate proc_macro;

mod i18n_display;

use std::borrow::Borrow;

use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, TokenTree};
use quote::ToTokens;
use syn::{parse_macro_input, spanned::Spanned, token::Brace, Data, DeriveInput, Meta, Variant};

#[macro_use]
extern crate quote;

#[proc_macro_derive(I18nError, attributes(i18n_key, i18n_delegate))]
pub fn derive_i18n_error(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    // dbg!(input.clone());
    derive(&input).into()
}

fn derive(input: &DeriveInput) -> TokenStream2 {
    match &input.data {
        Data::Struct(_) => panic!("Union is not supported"),
        Data::Enum(data) => {
            // dbg!(data.variants.clone());
            let enum_variants: Vec<EnumVariant> = data
                .variants
                .iter()
                .map(|variant| {
                    let mut i18n_key = None;
                    let mut i18n_delegate = false;
                    variant.attrs.iter().for_each(|attr| {
                        // dbg!(attr.clone());
                        let meta = &attr.meta.clone();
                        match meta {
                            Meta::List(list) => {
                                if list.path.is_ident("i18n_key") {
                                    i18n_key = Some(list.tokens.clone());
                                }
                                if list.path.is_ident("i18n_delegate") {
                                    i18n_delegate = true;
                                }
                            }
                            _ => {
                                panic!("Unexpected attribute");
                            }
                        };
                    });

                    EnumVariant {
                        variant: variant.clone(),
                        i18n_key_delegate: match (i18n_key, i18n_delegate) {
                            (Some(_), true) => {
                                panic!("Cannot specify both i18n_key and i18n_delegate")
                            }
                            (Some(key), false) => I18nKeyDelegate::I18nKey(key),
                            (None, true) => I18nKeyDelegate::Delegate,
                            (None, false) => {
                                panic!("Either i18n_key or i18n_delegate must be specified")
                            }
                        },
                    }
                })
                .collect();
            // dbg!(enum_variants);
            let enum_struct = EnumStruct {
                ident: input.ident.clone(),
                enum_variants,
            };
            let token_stream = generate_impl(enum_struct);
            dbg!(token_stream.to_string());
            token_stream
        }
        Data::Union(_) => panic!("Union is not supported"),
    }
}

fn generate_impl(enum_struct: EnumStruct) -> TokenStream2 {
    let ident = enum_struct.ident.clone();
    let inner_function = enum_struct
        .enum_variants
        .clone()
        .into_iter()
        .map(|enum_variant| generate_inner_function(enum_variant))
        .collect::<Vec<_>>();

    let function = quote! {
        impl I18nDisplay for #ident {
            fn to_i18n_display(&self, language_code: LanguageCode) -> String {
                match self {
                    #(#inner_function)*
                }
            }
        }
    };
    // dbg!(function.to_string().clone());
    function
}

struct EnumStruct {
    ident: syn::Ident,
    enum_variants: Vec<EnumVariant>,
}

#[derive(Clone)]
struct EnumVariant {
    variant: Variant,
    i18n_key_delegate: I18nKeyDelegate,
}

fn generate_inner_function(enum_variant: EnumVariant) -> TokenStream2 {
    let variant = enum_variant.variant;
    let i18n_key_delegate = enum_variant.i18n_key_delegate;
    let function = match i18n_key_delegate {
        I18nKeyDelegate::I18nKey(key) => generate_i18n_inner_function(variant.clone(), key),
        I18nKeyDelegate::Delegate => generate_delegate_inner_function(variant.clone()),
    };
    function
}

fn generate_i18n_inner_function(variant: Variant, key: TokenStream2) -> TokenStream2 {
    // dbg!(variant.clone());
    let ident = variant.ident;
    let token_stream = match variant.fields {
        syn::Fields::Named(_) => {
            panic!("Named fields are not supported")
        }
        syn::Fields::Unnamed(_) => {
            // FIXME: Unnamed fields は複数がありうる
            quote! {
                Self::#ident(_, ..) => #key.to_string(),
            }
        }
        syn::Fields::Unit => {
            quote! {
                Self::#ident => #key.to_string(),
            }
        }
    };
    dbg!(token_stream.to_string());
    token_stream
}

fn generate_delegate_inner_function(variant: Variant) -> TokenStream2 {
    let ident = variant.ident;
    let token_stream = quote! {
        Self::#ident(inner) => inner.to_i18n_display(language_code),
    };
    dbg!(token_stream.clone());
    token_stream
}

#[derive(Debug, Clone)]
enum I18nKeyDelegate {
    I18nKey(TokenStream2),
    Delegate,
}
