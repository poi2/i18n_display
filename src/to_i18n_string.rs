use crate::LanguageCode;

pub trait ToI18nString {
    fn to_i18n_string(&self, language_code: LanguageCode) -> String;
}
