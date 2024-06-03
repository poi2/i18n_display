#[allow(unused_imports)]
#[macro_use]
extern crate rust_i18n;

rust_i18n::i18n!("locales");

#[cfg(test)]
#[allow(unused_variables)]
mod test {
    use super::*;

    use i18n_error::{I18nError, LanguageCode, ToI18nString};
    use test_case::test_case;

    #[test_case(LanguageCode::En, "Resource not found.")]
    #[test_case(LanguageCode::Fr, "Ressource non trouvée.")]
    fn test_to_i18n_string_for_unit_like_enum(language_code: LanguageCode, expected: &str) {
        #[derive(I18nError)]
        #[i18n_language_codes(En, Fr)]
        enum DomainError {
            #[i18n_key("error.DomainError.ResourceNotFound")]
            ResourceNotFound,
        }

        let error = DomainError::ResourceNotFound;

        assert_eq!(error.to_i18n_string(language_code), expected.to_string());
    }

    #[test_case(LanguageCode::En, "Resource not found.")]
    #[test_case(LanguageCode::Fr, "Ressource non trouvée.")]
    fn test_to_i18n_string_for_single_tuple_like_enum(language_code: LanguageCode, expected: &str) {
        #[allow(unused)]
        #[derive(I18nError)]
        #[i18n_language_codes(En, Fr)]
        enum DomainError {
            #[i18n_key("error.DomainError.ResourceNotFound")]
            ResourceNotFound(String),
        }

        let error = DomainError::ResourceNotFound("Beer".to_string());

        assert_eq!(error.to_i18n_string(language_code), expected.to_string());
    }

    #[test_case(LanguageCode::En, "Resource not found.")]
    #[test_case(LanguageCode::Fr, "Ressource non trouvée.")]
    fn test_to_i18n_string_for_multiple_tuple_like_enum(
        language_code: LanguageCode,
        expected: &str,
    ) {
        #[allow(unused)]
        #[derive(I18nError)]
        #[i18n_language_codes(En, Fr)]
        enum DomainError {
            #[i18n_key("error.DomainError.ResourceNotFound")]
            ResourceNotFound(String, String),
        }

        let error = DomainError::ResourceNotFound("Beer".to_string(), "1".to_string());

        assert_eq!(error.to_i18n_string(language_code), expected.to_string());
    }

    #[test_case(LanguageCode::En, "Resource not found.")]
    #[test_case(LanguageCode::Fr, "Ressource non trouvée.")]
    fn test_to_i18n_string_for_field_like_enum(language_code: LanguageCode, expected: &str) {
        #[derive(I18nError)]
        #[i18n_language_codes(En, Fr)]
        enum DomainError {
            #[i18n_key("error.DomainError.ResourceNotFound")]
            ResourceNotFound { resource: String, id: String },
        }

        let error = DomainError::ResourceNotFound {
            resource: "Beer".to_string(),
            id: "1".to_string(),
        };

        assert_eq!(error.to_i18n_string(language_code), expected.to_string());
    }

    #[test_case(LanguageCode::En, "Resource not found.")]
    #[test_case(LanguageCode::Fr, "Ressource non trouvée.")]
    fn test_delegation_to_enum(language_code: LanguageCode, expected: &str) {
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

        assert_eq!(error.to_i18n_string(language_code), expected.to_string());
    }

    #[test_case(LanguageCode::En, "Resource not found.")]
    #[test_case(LanguageCode::Fr, "Ressource non trouvée.")]
    fn test_delegation_to_struct(language_code: LanguageCode, expected: &str) {
        #[derive(I18nError)]
        #[i18n_language_codes(En, Fr)]
        enum DomainError {
            #[i18n_delegate]
            ResourceNotFound(ResourceNotFound),
        }

        #[allow(unused)]
        #[derive(I18nError)]
        #[i18n_key("error.DomainError.ResourceNotFound")]
        #[i18n_language_codes(En, Fr)]
        struct ResourceNotFound {
            id: String,
        }

        let error = DomainError::ResourceNotFound(ResourceNotFound {
            id: "1".to_string(),
        });

        assert_eq!(error.to_i18n_string(language_code), expected.to_string());
    }

    #[test_case(LanguageCode::En, "Resource not found.")]
    #[test_case(LanguageCode::Fr, "Ressource non trouvée.")]
    fn test_to_i18n_string_for_struct(language_code: LanguageCode, expected: &str) {
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

        assert_eq!(error.to_i18n_string(language_code), expected.to_string());
    }

    #[test_case(LanguageCode::En, "Resource not found.")]
    #[test_case(LanguageCode::Fr, "Ressource non trouvée.")]
    fn test_to_i18n_string_for_generics_struct(language_code: LanguageCode, expected: &str) {
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

        assert_eq!(error.to_i18n_string(language_code), expected.to_string());
    }

    #[test_case(LanguageCode::En, "Resource not found.")]
    #[test_case(LanguageCode::Fr, "Ressource non trouvée.")]
    fn test_to_i18n_string_for_generics_with_where_struct(
        language_code: LanguageCode,
        expected: &str,
    ) {
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

        assert_eq!(error.to_i18n_string(language_code), expected.to_string());
    }

    #[test_case(LanguageCode::En)]
    #[test_case(LanguageCode::Fr => panics "Unsupported language code: Fr")]
    fn test_fail_to_i18n_string_with_unsupported_language_code(language_code: LanguageCode) {
        #[derive(I18nError)]
        #[i18n_language_codes(En)]
        enum DomainError {
            #[i18n_key("error.DomainError.ResourceNotFound")]
            ResourceNotFound,
        }

        let error = DomainError::ResourceNotFound;

        error.to_i18n_string(language_code);
    }
}
