use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Locale {
    // https://bybit-exchange.github.io/docs/v5/enum#locale
    #[serde(rename = "de-DE")]
    DeDe, // German
    #[serde(rename = "en-US")]
    EnUs, // English (United States)
    #[serde(rename = "es-AR")]
    EsAr, // Spanish (Argentina)
    #[serde(rename = "es-ES")]
    EsEs, // Spanish (Spain)
    #[serde(rename = "es-MX")]
    EsMx, // Spanish (Mexico)
    #[serde(rename = "fr-FR")]
    FrFr, // French (France)
    #[serde(rename = "kk-KZ")]
    KkKz, // Kazakh
    #[serde(rename = "id-ID")]
    IdId, // Indonesian
    #[serde(rename = "uk-UA")]
    UkUa, // Ukrainian
    #[serde(rename = "ja-JP")]
    JaJp, // Japanese
    #[serde(rename = "ru-RU")]
    RuRu, // Russian
    #[serde(rename = "th-TH")]
    ThTh, // Thai
    #[serde(rename = "pt-BR")]
    PtBr, // Portuguese (Brazil)
    #[serde(rename = "tr-TR")]
    TrTr, // Turkish
    #[serde(rename = "vi-VN")]
    ViVn, // Vietnamese
    #[serde(rename = "zh-TW")]
    ZhTw, // Chinese (Taiwan)
    #[serde(rename = "ar-SA")]
    ArSa, // Arabic (Saudi Arabia)
    #[serde(rename = "hi-IN")]
    HiIn, // Hindi (India)
    #[serde(rename = "fil-PH")]
    FilPh, // Filipino (Philippines)
}

impl Display for Locale {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Locale::DeDe => write!(f, "de-DE"),
            Locale::EnUs => write!(f, "en-US"),
            Locale::EsAr => write!(f, "es-AR"),
            Locale::EsEs => write!(f, "es-ES"),
            Locale::EsMx => write!(f, "es-MX"),
            Locale::FrFr => write!(f, "fr-FR"),
            Locale::KkKz => write!(f, "kk-KZ"),
            Locale::IdId => write!(f, "id-ID"),
            Locale::UkUa => write!(f, "uk-UA"),
            Locale::JaJp => write!(f, "ja-JP"),
            Locale::RuRu => write!(f, "ru-RU"),
            Locale::ThTh => write!(f, "th-TH"),
            Locale::PtBr => write!(f, "pt-BR"),
            Locale::TrTr => write!(f, "tr-TR"),
            Locale::ViVn => write!(f, "vi-VN"),
            Locale::ZhTw => write!(f, "zh-TW"),
            Locale::ArSa => write!(f, "ar-SA"),
            Locale::HiIn => write!(f, "hi-IN"),
            Locale::FilPh => write!(f, "fil-PH"),
        }
    }
}
