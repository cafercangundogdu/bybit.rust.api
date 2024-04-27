use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum AnnouncementType {
    // https://bybit-exchange.github.io/docs/v5/enum#announcementtype
    #[serde(rename = "new_crypto")]
    NewCrypto, // New crypto listing
    #[serde(rename = "latest_bybit_news")]
    LatestBybitNews, // Latest Bybit news
    #[serde(rename = "delistings")]
    Delistings, // Delistings
    #[serde(rename = "latest_activities")]
    LatestActivities, // Latest activities
    #[serde(rename = "product_updates")]
    ProductUpdates, // Product updates
    #[serde(rename = "maintenance_updates")]
    MaintenanceUpdates, // Maintenance updates
    #[serde(rename = "new_fiat_listings")]
    NewFiatListings, // New fiat listings
    #[serde(rename = "other")]
    Other, // Other
}

impl Display for AnnouncementType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            AnnouncementType::NewCrypto => write!(f, "new_crypto"),
            AnnouncementType::LatestBybitNews => write!(f, "latest_bybit_news"),
            AnnouncementType::Delistings => write!(f, "delistings"),
            AnnouncementType::LatestActivities => write!(f, "latest_activities"),
            AnnouncementType::ProductUpdates => write!(f, "product_updates"),
            AnnouncementType::MaintenanceUpdates => write!(f, "maintenance_updates"),
            AnnouncementType::NewFiatListings => write!(f, "new_fiat_listings"),
            AnnouncementType::Other => write!(f, "other"),
        }
    }
}
