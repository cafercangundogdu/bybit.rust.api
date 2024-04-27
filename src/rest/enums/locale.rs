use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Locale {
    // https://bybit-exchange.github.io/docs/v5/enum#locale
    #[serde(rename = "de-DE")]
    de_DE, // German
    #[serde(rename = "en-US")]
    en_US, // English (United States)
    #[serde(rename = "es-AR")]
    es_AR, // Spanish (Argentina)
    #[serde(rename = "es-ES")]
    es_ES, // Spanish (Spain)
    #[serde(rename = "es-MX")]
    es_MX, // Spanish (Mexico)
    #[serde(rename = "fr-FR")]
    fr_FR, // French (France)
    #[serde(rename = "kk-KZ")]
    kk_KZ, // Kazakh
    #[serde(rename = "id-ID")]
    id_ID, // Indonesian
    #[serde(rename = "uk-UA")]
    uk_UA, // Ukrainian
    #[serde(rename = "ja-JP")]
    ja_JP, // Japanese
    #[serde(rename = "ru-RU")]
    ru_RU, // Russian
    #[serde(rename = "th-TH")]
    th_TH, // Thai
    #[serde(rename = "pt-BR")]
    pt_BR, // Portuguese (Brazil)
    #[serde(rename = "tr-TR")]
    tr_TR, // Turkish
    #[serde(rename = "vi-VN")]
    vi_VN, // Vietnamese
    #[serde(rename = "zh-TW")]
    zh_TW, // Chinese (Taiwan)
    #[serde(rename = "ar-SA")]
    ar_SA, // Arabic (Saudi Arabia)
    #[serde(rename = "hi-IN")]
    hi_IN, // Hindi (India)
    #[serde(rename = "fil-PH")]
    fil_PH, // Filipino (Philippines)
}

impl Display for Locale {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Locale::de_DE => write!(f, "de-DE"),
            Locale::en_US => write!(f, "en-US"),
            Locale::es_AR => write!(f, "es-AR"),
            Locale::es_ES => write!(f, "es-ES"),
            Locale::es_MX => write!(f, "es-MX"),
            Locale::fr_FR => write!(f, "fr-FR"),
            Locale::kk_KZ => write!(f, "kk-KZ"),
            Locale::id_ID => write!(f, "id-ID"),
            Locale::uk_UA => write!(f, "uk-UA"),
            Locale::ja_JP => write!(f, "ja-JP"),
            Locale::ru_RU => write!(f, "ru-RU"),
            Locale::th_TH => write!(f, "th-TH"),
            Locale::pt_BR => write!(f, "pt-BR"),
            Locale::tr_TR => write!(f, "tr-TR"),
            Locale::vi_VN => write!(f, "vi-VN"),
            Locale::zh_TW => write!(f, "zh-TW"),
            Locale::ar_SA => write!(f, "ar-SA"),
            Locale::hi_IN => write!(f, "hi-IN"),
            Locale::fil_PH => write!(f, "fil-PH"),
        }
    }
}
