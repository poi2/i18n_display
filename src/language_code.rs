use strum::{Display, EnumString};

#[allow(dead_code)]
#[derive(Debug, Display, EnumString)]
pub enum LanguageCode {
    #[strum(serialize = "en")]
    En,
    #[strum(serialize = "ja")]
    Ja,
}
