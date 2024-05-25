use i18n_error::I18nError;
use rust_i18n::t;
use strum::{Display, EnumString};
use thiserror::Error;

#[allow(dead_code)]
#[derive(I18nError, Debug, Error)]
#[i18n_language_codes(Ja, En)]
pub enum MyError {
    #[error("Insufficient funds")]
    #[i18n_key("InsufficientFunds")]
    InsufficientFunds(String),
    #[error("Out of stock")]
    #[i18n_key("OutOfStock")]
    OutOfStock,
    #[error("MultipleTuple")]
    #[i18n_key("MultipleTuple")]
    MultipleTuple(String, i32),
    #[error("FieldError")]
    #[i18n_key("FieldError")]
    Field { field1: String, field2: i32 },
}

#[allow(dead_code)]
#[derive(I18nError, Debug, Error)]
#[i18n_language_codes(Ja, En)]
pub enum UseCaseError {
    #[error("FooError")]
    #[i18n_key("FooError")]
    Foo,
    #[error("DomainError")]
    #[i18n_delegate]
    Domain(MyError),
}

#[allow(dead_code)]
#[derive(Debug, Display, EnumString)]
pub enum LanguageCode {
    #[strum(serialize = "en")]
    En,
    #[strum(serialize = "ja")]
    Ja,
}

pub trait ToI18nString {
    fn to_i18n_string(&self, language_code: LanguageCode) -> String;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_unnamed_enum_with_en() {
        let error = MyError::InsufficientFunds("100".to_string());
        assert_eq!(
            error.to_i18n_string(LanguageCode::En),
            "You do not have enough funds to complete this transaction. Please check your balance and try again.".to_string()
        );
    }

    #[test]
    fn test_unnamed_enum_with_ja() {
        let error = MyError::InsufficientFunds("100".to_string());
        assert_eq!(
            error.to_i18n_string(LanguageCode::Ja),
            "この取引を完了するための残高が不足しています。残高を確認してもう一度お試しください。"
                .to_string()
        );
    }

    #[test]
    fn test_empty_unnamed_enum_with_en() {
        let error = MyError::OutOfStock;
        assert_eq!(
            error.to_i18n_string(LanguageCode::En),
            "This item is currently out of stock. Please check back later.".to_string()
        );
    }

    #[test]
    fn test_empty_unnamed_enum_with_ja() {
        let error = MyError::OutOfStock;
        assert_eq!(
            error.to_i18n_string(LanguageCode::Ja),
            "この商品は現在在庫切れです。後ほど再度ご確認ください。".to_string()
        );
    }
}
