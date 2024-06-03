#[allow(unused_imports)]
#[macro_use]
extern crate rust_i18n;

rust_i18n::i18n!("locales");

use i18n_error::{I18nError, LanguageCode, ToI18nString};
use test_case::test_case;

#[test_case(LanguageCode::En => "Resource not found.".to_string())]
#[test_case(LanguageCode::Fr => "Ressource non trouvée.".to_string())]
#[test_case(LanguageCode::Zu => panics "Unsupported language code: Zu")]

fn test_to_i18n_string_for_simple_struct(language_code: LanguageCode) -> String {
    #[allow(unused)]
    #[derive(I18nError)]
    #[i18n_language_codes(En, Fr)]
    #[i18n_key("error.DomainError.ResourceNotFound")]
    struct ResourceNotFound {
        resource: String,
    }

    let error = ResourceNotFound {
        resource: "Beer".to_string(),
    };

    error.to_i18n_string(language_code)
}

#[test_case(LanguageCode::En => "Resource not found.".to_string())]
#[test_case(LanguageCode::Fr => "Ressource non trouvée.".to_string())]
#[test_case(LanguageCode::Zu => panics "Unsupported language code: Zu")]
fn test_to_i18n_string_for_generics_struct(language_code: LanguageCode) -> String {
    #[allow(unused)]
    #[derive(I18nError)]
    #[i18n_language_codes(En, Fr)]
    #[i18n_key("error.DomainError.ResourceNotFound")]
    struct ResourceNotFound<T> {
        resource: T,
    }

    let error = ResourceNotFound {
        resource: "Beer".to_string(),
    };

    error.to_i18n_string(language_code)
}

#[test_case(LanguageCode::En => "Resource not found.".to_string())]
#[test_case(LanguageCode::Fr => "Ressource non trouvée.".to_string())]
#[test_case(LanguageCode::Zu => panics "Unsupported language code: Zu")]
fn test_to_i18n_string_for_generics_with_where_struct(language_code: LanguageCode) -> String {
    #[allow(unused)]
    #[derive(I18nError)]
    #[i18n_language_codes(En, Fr)]
    #[i18n_key("error.DomainError.ResourceNotFound")]
    struct ResourceNotFound<T>
    where
        T: std::fmt::Display,
    {
        resource: T,
    }

    let error = ResourceNotFound {
        resource: "Beer".to_string(),
    };

    error.to_i18n_string(language_code)
}
