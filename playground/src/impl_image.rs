// use strum::{Display, EnumString};
// use thiserror::Error;
use rust_i18n::t;

// For rust_i18n
i18n!("locales", fallback = "en");

fn foo() -> String {
    t!("InsufficientFunds").to_string()
}

// #[derive(Error, Debug)]
// // #[derive(ToI18nString)]
// pub enum DomainError {
//     #[error("Insufficient funds")]
//     // #[i18n_key("InsufficientFunds")]
//     InsufficientFunds(String),
//     #[error("Out of stock")]
//     // #[i18n_key("OutOfStock")]
//     OutOfStock(String),
// }

// #[derive(Error, Debug)]
// // #[derive(ToI18nString)]
// pub enum UseCaseError {
//     #[error("Domain error")]
//     // #[i18n_delegate]
//     Domain(DomainError),
//     #[error("Permission deny")]
//     // #[i18n_key("PermissionDeny")]
//     PermissionDeny(String),
// }

// #[derive(Debug, Display, EnumString)]
// pub enum LanguageCode {
//     #[strum(serialize = "en")]
//     En,
//     #[strum(serialize = "ja")]
//     Ja,
// }

// #[allow(dead_code)]
// trait ToI18nString {
//     fn to_i18n_string(&self, language_code: LanguageCode) -> String;
// }

// impl ToI18nString for DomainError {
//     fn to_i18n_string(&self, language_code: LanguageCode) -> String {
//         match self {
//             Self::InsufficientFunds(_) => {
//                 t!("InsufficientFunds", locale = &language_code.to_string())
//             }
//             Self::OutOfStock(_) => t!("OutOfStock", locale = &language_code.to_string()),
//         }
//         .to_string()
//     }
// }

// impl ToI18nString for UseCaseError {
//     fn to_i18n_string(&self, language_code: LanguageCode) -> String {
//         match self {
//             Self::Domain(inner) => inner.to_i18n_string(language_code),
//             Self::PermissionDeny(_) => {
//                 t!("PermissionDeny", locale = &language_code.to_string()).to_string()
//             }
//         }
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn create_translated_message_from_domain_error_for_ja() {
//         let domain_error = DomainError::InsufficientFunds("FooBar".to_string());

//         let actual = domain_error.to_i18n_string(LanguageCode::Ja);

//         assert_eq!(
//             actual,
//             "この取引を完了するための残高が不足しています。残高を確認してもう一度お試しください。"
//                 .to_string()
//         )
//     }

//     #[test]
//     fn create_translated_message_from_domain_error_for_en() {
//         let domain_error = DomainError::InsufficientFunds("FooBar".to_string());

//         let actual = domain_error.to_i18n_string(LanguageCode::En);

//         assert_eq!(
//             actual,
//             "You do not have enough funds to complete this transaction. Please check your balance and try again."
//                 .to_string()
//         )
//     }
// }
