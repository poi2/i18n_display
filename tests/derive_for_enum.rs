#[allow(unused_imports)]
#[macro_use]
extern crate rust_i18n;

rust_i18n::i18n!("locales");

use i18n_error::{I18nError, LanguageCode, ToI18nString};
use test_case::test_case;

#[test_case(LanguageCode::En => "Resource not found.".to_string())]
#[test_case(LanguageCode::Fr => "Ressource non trouvée.".to_string())]
#[test_case(LanguageCode::Zu => panics "Unsupported language code: Zu")]

fn test_to_i18n_string_for_unit_like_enum(language_code: LanguageCode) -> String {
    #[derive(I18nError)]
    #[i18n_language_codes(En, Fr)]
    enum DomainError {
        #[i18n_key("error.DomainError.ResourceNotFound")]
        ResourceNotFound,
    }

    let error = DomainError::ResourceNotFound;

    error.to_i18n_string(language_code)
}

#[test_case(LanguageCode::En => "Resource not found.".to_string())]
#[test_case(LanguageCode::Fr => "Ressource non trouvée.".to_string())]
#[test_case(LanguageCode::Zu => panics "Unsupported language code: Zu")]
fn test_to_i18n_string_for_single_tuple_like_enum(language_code: LanguageCode) -> String {
    #[allow(unused)]
    #[derive(I18nError)]
    #[i18n_language_codes(En, Fr)]
    enum DomainError {
        #[i18n_key("error.DomainError.ResourceNotFound")]
        ResourceNotFound(String),
    }

    let error = DomainError::ResourceNotFound("Beer".to_string());

    error.to_i18n_string(language_code)
}

#[test_case(LanguageCode::En => "Resource not found.".to_string())]
#[test_case(LanguageCode::Fr => "Ressource non trouvée.".to_string())]
#[test_case(LanguageCode::Zu => panics "Unsupported language code: Zu")]
fn test_to_i18n_string_for_multiple_tuple_like_enum(language_code: LanguageCode) -> String {
    #[allow(unused)]
    #[derive(I18nError)]
    #[i18n_language_codes(En, Fr)]
    enum DomainError {
        #[i18n_key("error.DomainError.ResourceNotFound")]
        ResourceNotFound(String, String),
    }

    let error = DomainError::ResourceNotFound("Beer".to_string(), "1".to_string());

    error.to_i18n_string(language_code)
}

#[test_case(LanguageCode::En => "Resource not found.".to_string())]
#[test_case(LanguageCode::Fr => "Ressource non trouvée.".to_string())]
#[test_case(LanguageCode::Zu => panics "Unsupported language code: Zu")]
fn test_to_i18n_string_for_field_like_enum(language_code: LanguageCode) -> String {
    #[derive(I18nError)]
    #[i18n_language_codes(En, Fr)]
    enum DomainError {
        #[i18n_key("error.DomainError.ResourceNotFound")]
        ResourceNotFound { _resource: String, _id: String },
    }

    let error = DomainError::ResourceNotFound {
        _resource: "Beer".to_string(),
        _id: "1".to_string(),
    };

    error.to_i18n_string(language_code)
}

#[test_case(LanguageCode::En => "Resource not found.".to_string())]
#[test_case(LanguageCode::Fr => "Ressource non trouvée.".to_string())]
#[test_case(LanguageCode::Zu => panics "Unsupported language code: Zu")]
fn test_delegation_to_inner_enum_error(language_code: LanguageCode) -> String {
    #[derive(I18nError)]
    #[i18n_language_codes(En, Fr)]
    enum DomainError {
        #[i18n_delegate]
        ResourceNotFound(ResourceNotFound),
    }

    #[allow(unused)]
    #[derive(I18nError)]
    #[i18n_language_codes(En, Fr)]
    enum ResourceNotFound {
        #[i18n_key("error.DomainError.ResourceNotFound")]
        Beer,
    }

    let error = DomainError::ResourceNotFound(ResourceNotFound::Beer);

    error.to_i18n_string(language_code)
}

#[test_case(LanguageCode::En => "Resource not found.".to_string())]
#[test_case(LanguageCode::Fr => "Ressource non trouvée.".to_string())]
#[test_case(LanguageCode::Zu => panics "Unsupported language code: Zu")]
fn test_delegation_to_inner_struct_error(language_code: LanguageCode) -> String {
    #[derive(I18nError)]
    #[i18n_language_codes(En, Fr)]
    enum DomainError {
        #[i18n_delegate]
        ResourceNotFound(ResourceNotFound),
    }

    #[allow(unused)]
    #[derive(I18nError)]
    #[i18n_language_codes(En, Fr)]
    #[i18n_key("error.DomainError.ResourceNotFound")]
    struct ResourceNotFound {
        resource: String,
    }

    let error = DomainError::ResourceNotFound(ResourceNotFound {
        resource: "Beer".to_string(),
    });

    error.to_i18n_string(language_code)
}
