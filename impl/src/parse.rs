use proc_macro2::{Ident, TokenStream as TokenStream2, TokenTree};
use quote::ToTokens;
use syn::{Data, DeriveInput, Meta, Variant};

#[derive(Clone)]
pub(crate) enum DeriveTarget {
    Struct(ErrorOfStruct),
    Enum(ErrorOfEnum),
}

#[derive(Clone)]
pub(crate) struct ErrorOfStruct {
    pub(crate) input: DeriveInput,
    pub(crate) i18n_key: TokenStream2,
}

#[derive(Clone)]
pub(crate) struct ErrorOfEnum {
    pub(crate) ident: syn::Ident,
    pub(crate) enum_variants: Vec<EnumVariant>,
}

#[derive(Clone)]
pub(crate) struct EnumVariant {
    pub(crate) variant: Variant,
    pub(crate) i18n_key_delegate: I18nKeyDelegate,
}

#[derive(Clone)]
pub(crate) enum I18nKeyDelegate {
    I18nKey(TokenStream2),
    Delegate,
}

pub(crate) fn get_language_codes(input: &DeriveInput) -> Vec<Ident> {
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

                    if language_codes.is_empty() {
                        return None;
                    }

                    Some(language_codes)
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

pub(crate) fn get_derive_target(input: &DeriveInput) -> DeriveTarget {
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

            let struct_struct = ErrorOfStruct {
                input: input.clone(),
                i18n_key: token_stream.clone(),
            };

            DeriveTarget::Struct(struct_struct)
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

            DeriveTarget::Enum(ErrorOfEnum {
                ident: input.ident.clone(),
                enum_variants,
            })
        }
        Data::Union(_) => panic!("I18nError is only supported for Enum."),
    }
}
