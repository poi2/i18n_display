use std::str::FromStr;

use i18n_error::I18nError;
use thiserror::Error;

#[derive(I18nError, Debug, Error)]
pub enum MyError {
    #[error("Insufficient funds")]
    #[i18n_key("InsufficientFunds")]
    InsufficientFunds(String),
    #[error("Out of stock")]
    #[i18n_key("OutOfStock")]
    OutOfStock,
}

pub enum LanguageCode {
    En,
    Ja,
}

pub trait I18nDisplay {
    fn to_i18n_display(&self, language_code: LanguageCode) -> String;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let error = MyError::InsufficientFunds("100".to_string());
        assert_eq!(
            error.to_i18n_display(LanguageCode::En),
            "InsufficientFunds".to_string()
        );
        assert_eq!(
            error.to_i18n_display(LanguageCode::Ja),
            "InsufficientFunds".to_string()
        );

        let error = MyError::OutOfStock;
        assert_eq!(
            error.to_i18n_display(LanguageCode::En),
            "OutOfStock".to_string()
        );
        assert_eq!(
            error.to_i18n_display(LanguageCode::Ja),
            "OutOfStock".to_string()
        );
    }
}
