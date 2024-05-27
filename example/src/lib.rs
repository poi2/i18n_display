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

    #[derive(I18nError)]
    #[i18n_language_codes(En, Fr)]
    enum DomainError {
        #[i18n_key("error.DomainError.InsufficientFunds")]
        InsufficientFunds(String),
        #[i18n_key("error.DomainError.OutOfStock")]
        OutOfStock,
        #[i18n_key("error.DomainError.ValidationError")]
        Validation(String, String),
        #[i18n_key("error.DomainError.ResourceNotFound")]
        ResourceNotFound { field: String, id: String },
    }

    #[derive(I18nError)]
    #[i18n_language_codes(En, Fr)]
    enum UseCaseError {
        #[i18n_key("error.UseCaseError.AuthorizationError")]
        Authorization,
        #[i18n_delegate]
        Domain(DomainError),
    }

    #[test_case(LanguageCode::En, "You do not have enough funds.")]
    #[test_case(LanguageCode::Fr, "Vous n'avez pas assez de fonds.")]
    fn test_unnamed_enum(language_code: LanguageCode, expected: &str) {
        let error = DomainError::InsufficientFunds("".to_string());

        assert_eq!(error.to_i18n_string(language_code), expected.to_string());
    }

    #[test_case(LanguageCode::En, "This item is currently out of stock.")]
    #[test_case(LanguageCode::Fr, "Cet article est actuellement en rupture de stock.")]
    fn test_empty_unnamed_enum(language_code: LanguageCode, expected: &str) {
        let error = DomainError::OutOfStock;

        assert_eq!(error.to_i18n_string(language_code), expected.to_string());
    }

    #[test_case(LanguageCode::En, "Invalid input value.")]
    #[test_case(LanguageCode::Fr, "Valeur d'entrée invalide.")]
    fn test_unnamed_enum_with_multiple_value(language_code: LanguageCode, expected: &str) {
        let error = DomainError::Validation("foo".to_string(), "bar".to_string());

        assert_eq!(error.to_i18n_string(language_code), expected.to_string());
    }

    #[test_case(LanguageCode::En, "Resource not found.")]
    #[test_case(LanguageCode::Fr, "Ressource non trouvée.")]
    fn test_field_enum(language_code: LanguageCode, expected: &str) {
        let error = DomainError::ResourceNotFound {
            field: "foo".to_string(),
            id: "foo-1".to_string(),
        };

        assert_eq!(error.to_i18n_string(language_code), expected.to_string());
    }

    #[test_case(LanguageCode::En, "You do not have permission.")]
    #[test_case(LanguageCode::Fr, "Vous n'avez pas la permission.")]
    fn test_outside_enum(language_code: LanguageCode, expected: &str) {
        let error = UseCaseError::Authorization;

        assert_eq!(error.to_i18n_string(language_code), expected.to_string());
    }

    #[test_case(LanguageCode::En, "You do not have enough funds.")]
    #[test_case(LanguageCode::Fr, "Vous n'avez pas assez de fonds.")]
    fn test_delegation(language_code: LanguageCode, expected: &str) {
        let error = UseCaseError::Domain(DomainError::InsufficientFunds("".to_string()));

        assert_eq!(error.to_i18n_string(language_code), expected.to_string());
    }
}
