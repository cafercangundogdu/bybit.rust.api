use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum AnnouncementTag {
    // https://bybit-exchange.github.io/docs/v5/enum#announcementtag
    #[serde(rename = "Spot")]
    Spot, // Spot
    #[serde(rename = "Derivatives")]
    Derivatives, // Derivatives
    #[serde(rename = "Spot Listings")]
    SpotListings, // Spot Listings
    #[serde(rename = "BTC")]
    BTC, // BTC
    #[serde(rename = "ETH")]
    ETH, // ETH
    #[serde(rename = "Trading Bots")]
    TradingBots, // Trading Bots
    #[serde(rename = "USDC")]
    USDC, // USDC
    #[serde(rename = "Leveraged Tokens")]
    LeveragedTokens, // Leveraged Tokens
    #[serde(rename = "USDT")]
    USDT, // USDT
    #[serde(rename = "Margin Trading")]
    MarginTrading, // Margin Trading
    #[serde(rename = "Partnerships")]
    Partnerships, // Partnerships
    #[serde(rename = "Launchpad")]
    Launchpad, // Launchpad
    #[serde(rename = "Upgrades")]
    Upgrades, // Upgrades
    #[serde(rename = "ByVotes")]
    ByVotes, // ByVotes
    #[serde(rename = "Delistings")]
    Delistings, // Delistings
    #[serde(rename = "VIP")]
    VIP, // VIP
    #[serde(rename = "Futures")]
    Futures, // Futures
    #[serde(rename = "Institutions")]
    Institutions, // Institutions
    #[serde(rename = "Options")]
    Options, // Options
    #[serde(rename = "WEB3")]
    WEB3, // WEB3
    #[serde(rename = "Copy Trading")]
    CopyTrading, // Copy Trading
    #[serde(rename = "Earn")]
    Earn, // Earn
    #[serde(rename = "Bybit Savings")]
    BybitSavings, // Bybit Savings
    #[serde(rename = "Dual Asset")]
    DualAsset, // Dual Asset
    #[serde(rename = "Liquidity Mining")]
    LiquidityMining, // Liquidity Mining
    #[serde(rename = "Shark Fin")]
    SharkFin, // Shark Fin
    #[serde(rename = "Launchpool")]
    Launchpool, // Launchpool
    #[serde(rename = "NFT GrabPic")]
    NFTGrabPic, // NFT GrabPic
    #[serde(rename = "Buy Crypto")]
    BuyCrypto, // Buy Crypto
    #[serde(rename = "P2P Trading")]
    P2PTrading, // P2P Trading
    #[serde(rename = "Fiat Deposit")]
    FiatDeposit, // Fiat Deposit
    #[serde(rename = "Crypto Deposit")]
    CryptoDeposit, // Crypto Deposit
    #[serde(rename = "Спот")]
    Спот, // Спот
    #[serde(rename = "Спот лістинги")]
    Спотлістинги, // Спот лістинги
    #[serde(rename = "Торгові боти")]
    Торговіботи, // Торгові боти
    #[serde(rename = "Токени з кредитним плечем")]
    Токенизкредитнимплечем, // Токени з кредитним плечем
    #[serde(rename = "Маржинальна торгівля")]
    Маржинальнаторгівля, // Маржинальна торгівля
    #[serde(rename = "Партнерство")]
    Партнерство, // Партнерство
    #[serde(rename = "Оновлення")]
    Оновлення, // Оновлення
    #[serde(rename = "Делістинги")]
    Делістинги, // Делістинги
    #[serde(rename = "Ф'ючерси")]
    Фючерси, // Ф'ючерси
    #[serde(rename = "Опціони")]
    Опціони, // Опціони
    #[serde(rename = "Копітрейдинг")]
    Копітрейдинг, // Копітрейдинг
    #[serde(rename = "Bybit Накопичення")]
    BybitНакопичення, // Bybit Накопичення
    #[serde(rename = "Бівалютні інвестиції")]
    Бівалютніінвестиції, // Бівалютні інвестиції
    #[serde(rename = "Майнінг ліквідності")]
    Майнінгліквідності, // Майнінг ліквідності
    #[serde(rename = "Купівля криптовалюти")]
    Купівлякриптовалюти, // Купівля криптовалюти
    #[serde(rename = "P2P торгівля")]
    P2Pторгівля, // P2P торгівля
    #[serde(rename = "Фіатні депозити")]
    Фіатнідепозити, // Фіатні депозити
    #[serde(rename = "Криптодепозити")]
    Криптодепозити, // Криптодепозити
    #[serde(rename = "Копитрейдинг")]
    Копитрейдинг, // Копитрейдинг
    #[serde(rename = "Торговые боты")]
    Торговыеботы, // Торговые боты
    #[serde(rename = "Деривативы")]
    Деривативы, // Деривативы
    #[serde(rename = "P2P")]
    P2P, // P2P
    #[serde(rename = "Спот листинги")]
    Спотлистинги, // Спот листинги
    #[serde(rename = "Деривативи")]
    Деривативи, // Деривативи
    #[serde(rename = "MT4")]
    MT4, // MT4
    #[serde(rename = "Lucky Draw")]
    LuckyDraw, // Lucky Draw
    #[serde(rename = "Unified Trading Account")]
    UnifiedTradingAccount, // Unified Trading Account
    #[serde(rename = "Єдиний торговий акаунт")]
    Єдинийторговийакаунт, // Єдиний торговий акаунт
    #[serde(rename = "Единый торговый аккаунт")]
    Единыйторговыйаккаунт, // Единый торговый аккаунт
    #[serde(rename = "Институциональный трейдинг")]
    Институциональныйтрейдинг, // Институциональный трейдинг
    #[serde(rename = "Інституціональний трейдинг")]
    Інституціональнийтрейдинг, // Інституціональний трейдинг
    #[serde(rename = "Делистинг")]
    Делистинг, // Делистинг
}

impl Display for AnnouncementTag {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            AnnouncementTag::Spot => write!(f, "Spot"),
            AnnouncementTag::Derivatives => write!(f, "Derivatives"),
            AnnouncementTag::SpotListings => write!(f, "Spot Listings"),
            AnnouncementTag::BTC => write!(f, "BTC"),
            AnnouncementTag::ETH => write!(f, "ETH"),
            AnnouncementTag::TradingBots => write!(f, "Trading Bots"),
            AnnouncementTag::USDC => write!(f, "USDC"),
            AnnouncementTag::LeveragedTokens => write!(f, "Leveraged Tokens"),
            AnnouncementTag::USDT => write!(f, "USDT"),
            AnnouncementTag::MarginTrading => write!(f, "Margin Trading"),
            AnnouncementTag::Partnerships => write!(f, "Partnerships"),
            AnnouncementTag::Launchpad => write!(f, "Launchpad"),
            AnnouncementTag::Upgrades => write!(f, "Upgrades"),
            AnnouncementTag::ByVotes => write!(f, "ByVotes"),
            AnnouncementTag::Delistings => write!(f, "Delistings"),
            AnnouncementTag::VIP => write!(f, "VIP"),
            AnnouncementTag::Futures => write!(f, "Futures"),
            AnnouncementTag::Institutions => write!(f, "Institutions"),
            AnnouncementTag::Options => write!(f, "Options"),
            AnnouncementTag::WEB3 => write!(f, "WEB3"),
            AnnouncementTag::CopyTrading => write!(f, "Copy Trading"),
            AnnouncementTag::Earn => write!(f, "Earn"),
            AnnouncementTag::BybitSavings => write!(f, "Bybit Savings"),
            AnnouncementTag::DualAsset => write!(f, "Dual Asset"),
            AnnouncementTag::LiquidityMining => write!(f, "Liquidity Mining"),
            AnnouncementTag::SharkFin => write!(f, "Shark Fin"),
            AnnouncementTag::Launchpool => write!(f, "Launchpool"),
            AnnouncementTag::NFTGrabPic => write!(f, "NFT GrabPic"),
            AnnouncementTag::BuyCrypto => write!(f, "Buy Crypto"),
            AnnouncementTag::P2PTrading => write!(f, "P2P Trading"),
            AnnouncementTag::FiatDeposit => write!(f, "Fiat Deposit"),
            AnnouncementTag::CryptoDeposit => write!(f, "Crypto Deposit"),
            AnnouncementTag::Спот => write!(f, "Спот"),
            AnnouncementTag::Спотлістинги => write!(f, "Спот лістинги"),
            AnnouncementTag::Торговіботи => write!(f, "Торгові боти"),
            AnnouncementTag::Токенизкредитнимплечем => {
                write!(f, "Токени з кредитним плечем")
            }
            AnnouncementTag::Маржинальнаторгівля => {
                write!(f, "Маржинальна торгівля")
            }
            AnnouncementTag::Партнерство => write!(f, "Партнерство"),
            AnnouncementTag::Оновлення => write!(f, "Оновлення"),
            AnnouncementTag::Делістинги => write!(f, "Делістинги"),
            AnnouncementTag::Фючерси => write!(f, "Ф'ючерси"),
            AnnouncementTag::Опціони => write!(f, "Опціони"),
            AnnouncementTag::Копітрейдинг => write!(f, "Копітрейдинг"),
            AnnouncementTag::BybitНакопичення => write!(f, "Bybit Накопичення"),
            AnnouncementTag::Бівалютніінвестиції => {
                write!(f, "Бівалютні інвестиції")
            }
            AnnouncementTag::Майнінгліквідності => {
                write!(f, "Майнінг ліквідності")
            }
            AnnouncementTag::Купівлякриптовалюти => {
                write!(f, "Купівля криптовалюти")
            }
            AnnouncementTag::P2Pторгівля => write!(f, "P2P торгівля"),
            AnnouncementTag::Фіатнідепозити => write!(f, "Фіатні депозити"),
            AnnouncementTag::Криптодепозити => write!(f, "Криптодепозити"),
            AnnouncementTag::Копитрейдинг => write!(f, "Копитрейдинг"),
            AnnouncementTag::Торговыеботы => write!(f, "Торговые боты"),
            AnnouncementTag::Деривативы => write!(f, "Деривативы"),
            AnnouncementTag::P2P => write!(f, "P2P"),
            AnnouncementTag::Спотлистинги => write!(f, "Спот листинги"),
            AnnouncementTag::Деривативи => write!(f, "Деривативи"),
            AnnouncementTag::MT4 => write!(f, "MT4"),
            AnnouncementTag::LuckyDraw => write!(f, "Lucky Draw"),
            AnnouncementTag::UnifiedTradingAccount => write!(f, "Unified Trading Account"),
            AnnouncementTag::Єдинийторговийакаунт => {
                write!(f, "Єдиний торговий акаунт")
            }
            AnnouncementTag::Единыйторговыйаккаунт => {
                write!(f, "Единый торговый аккаунт")
            }
            AnnouncementTag::Институциональныйтрейдинг => {
                write!(f, "Институциональный трейдинг")
            }
            AnnouncementTag::Інституціональнийтрейдинг => {
                write!(f, "Інституціональний трейдинг")
            }
            AnnouncementTag::Делистинг => write!(f, "Делистинг"),
        }
    }
}
