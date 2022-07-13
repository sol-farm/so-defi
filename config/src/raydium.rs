//! provides helepr functions for parsing raydium's configuration api

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// the hostname and main path for Raydium's V2 api
pub const RAYDIUM_API: &str = "https://api.raydium.io/v2";

/// a common type used to denote the raydium api version
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub major: i64,
    pub minor: i64,
    pub patch: i64,
}

pub mod price_list {
    //! configuration helpers for the raydium pairs api request

    use super::*;
    pub const RAYDIUM_API_PRICE_LIST: &str = "main/price";

    pub type PriceList = HashMap<String, f64>;

    pub fn api_url() -> String {
        format_api_url(RAYDIUM_API_PRICE_LIST)
    }

    pub async fn fetch_async() -> Result<PriceList> {
        let client = reqwest::Client::builder().build()?;
        let res = client.get(api_url()).send().await?;
        let data = res.json::<PriceList>().await?;
        Ok(data)
    }
    pub fn fetch() -> Result<PriceList> {
        let client = reqwest::blocking::Client::builder().build()?;
        let res = client.get(api_url()).send()?;
        let data = res.json::<PriceList>()?;
        Ok(data)
    }

    #[cfg(test)]
    mod test {
        use super::*;
        #[test]
        fn test_api_url() {
            assert_eq!(api_url(), "https://api.raydium.io/v2/main/price");
        }

        #[tokio::test]
        async fn test_fetch_async() {
            let _ = fetch_async().await.unwrap();
        }
        #[test]
        fn test_fetch() {
            let _ = fetch().unwrap();
        }
    }
}

pub mod pairs_list {
    //! configuration helpers for the raydium pairs api request

    use super::*;
    pub const RAYDIUM_API_PAIRS_LIST: &str = "main/pairs";

    pub type PairList = Vec<Pair>;

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Pair {
        pub name: String,
        pub amm_id: String,
        pub lp_mint: String,
        pub market: String,
        pub liquidity: Option<f64>,
        pub volume24h: Option<f64>,
        pub volume24h_quote: Option<f64>,
        pub fee24h: Option<f64>,
        pub fee24h_quote: Option<f64>,
        pub volume7d: Option<f64>,
        pub volume7d_quote: Option<f64>,
        pub fee7d: Option<f64>,
        pub fee7d_quote: Option<f64>,
        pub volume30d: Option<f64>,
        pub volume30d_quote: Option<f64>,
        pub fee30d: Option<f64>,
        pub fee30d_quote: Option<f64>,
        pub price: Option<f64>,
        pub lp_price: Option<f64>,
        pub token_amount_coin: Option<f64>,
        pub token_amount_pc: Option<f64>,
        pub token_amount_lp: Option<f64>,
        pub apr24h: Option<f64>,
        pub apr7d: Option<f64>,
        pub apr30d: Option<f64>,
    }

    pub fn api_url() -> String {
        format_api_url(RAYDIUM_API_PAIRS_LIST)
    }

    pub async fn fetch_async() -> Result<PairList> {
        let client = reqwest::Client::builder().build()?;
        let res = client.get(api_url()).send().await?;
        let data = res.json::<PairList>().await?;
        Ok(data)
    }
    pub fn fetch() -> Result<PairList> {
        let client = reqwest::blocking::Client::builder().build()?;
        let res = client.get(api_url()).send()?;
        let data = res.json::<PairList>()?;
        Ok(data)
    }

    #[cfg(test)]
    mod test {
        use super::*;
        #[test]
        fn test_api_url() {
            assert_eq!(api_url(), "https://api.raydium.io/v2/main/pairs");
        }

        #[tokio::test]
        async fn test_fetch_async() {
            let _ = fetch_async().await.unwrap();
        }
        #[test]
        fn test_fetch() {
            let _ = fetch().unwrap();
        }
    }
}

pub mod farm_list {
    //! configuration helpers for raydium farms

    use super::*;
    pub const RAYDIUM_API_FARM_LIST: &str = "sdk/farm/mainnet.json";

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FarmList {
        pub name: String,
        pub version: Option<Version>,
        pub official: Vec<FarmListEntry>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FarmListEntry {
        pub id: String,
        pub lp_mint: String,
        pub reward_mints: Vec<String>,
        pub version: Option<i64>,
        pub program_id: String,
        pub authority: String,
        pub lp_vault: String,
        pub reward_vaults: Vec<String>,
        pub upcoming: bool,
    }

    pub fn api_url() -> String {
        format_api_url(RAYDIUM_API_FARM_LIST)
    }

    pub async fn fetch_async() -> Result<FarmList> {
        let client = reqwest::Client::builder().build()?;
        let res = client.get(api_url()).send().await?;
        let data = res.json::<FarmList>().await?;
        Ok(data)
    }
    pub fn fetch() -> Result<FarmList> {
        let client = reqwest::blocking::Client::builder().build()?;
        let res = client.get(api_url()).send()?;
        let data = res.json::<FarmList>()?;
        Ok(data)
    }
    #[cfg(test)]
    mod test {
        use super::*;
        #[test]
        fn test_api_url() {
            assert_eq!(api_url(), "https://api.raydium.io/v2/sdk/farm/mainnet.json");
        }

        #[tokio::test]
        async fn test_fetch_async() {
            let _ = fetch_async().await.unwrap();
        }
        #[test]
        fn test_fetch() {
            let _ = fetch().unwrap();
        }
    }
}

pub mod liquidity_list {
    //! configuration helpers for raydium amms

    use super::*;
    pub const RAYDIUM_API_LIQUIDITY_LIST: &str = "sdk/liquidity/mainnet.json";

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct LiquidityList {
        pub name: String,
        pub version: Option<Version>,
        pub official: Vec<LiquidityListEntry>,
        pub un_official: Vec<LiquidityListEntry>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct LiquidityListEntry {
        pub id: String,
        pub base_mint: String,
        pub quote_mint: String,
        pub lp_mint: String,
        pub version: i64,
        pub program_id: String,
        pub authority: String,
        pub open_orders: String,
        pub target_orders: String,
        pub base_vault: String,
        pub quote_vault: String,
        pub withdraw_queue: String,
        pub lp_vault: String,
        pub market_version: i64,
        pub market_program_id: String,
        pub market_id: String,
        pub market_authority: String,
        pub market_base_vault: String,
        pub market_quote_vault: String,
        pub market_bids: String,
        pub market_asks: String,
        pub market_event_queue: String,
    }

    pub fn api_url() -> String {
        format_api_url(RAYDIUM_API_LIQUIDITY_LIST)
    }

    pub async fn fetch_async() -> Result<LiquidityList> {
        let client = reqwest::Client::builder().build()?;
        let res = client.get(api_url()).send().await?;
        let data = res.json::<LiquidityList>().await?;
        Ok(data)
    }
    pub fn fetch() -> Result<LiquidityList> {
        let client = reqwest::blocking::Client::builder().build()?;
        let res = client.get(api_url()).send()?;
        let data = res.json::<LiquidityList>()?;
        Ok(data)
    }
    #[cfg(test)]
    mod test {
        use super::*;
        #[test]
        fn test_api_url() {
            assert_eq!(
                api_url(),
                "https://api.raydium.io/v2/sdk/liquidity/mainnet.json"
            );
        }

        #[tokio::test]
        async fn test_fetch_async() {
            let liquidity = fetch_async().await.unwrap();
            let mut ok = false;
            for liquidity in liquidity.un_official.iter() {
                if liquidity
                    .id
                    .eq(&"C614Uy93kGJrmuMRkPBUXtYu6E9MMRieKLcK3YUZGgxG")
                {
                    println!("found it {:#?}", liquidity);
                    ok = true;
                    break;
                }
            }
            assert_eq!(ok, true);
        }

        #[tokio::test]
        async fn test_fetch_async_stable_swap() {
            let liquidity = fetch_async().await.unwrap();
            let mut ok = false;
            for liquidity in liquidity.un_official.iter() {
                if liquidity
                    .id
                    .eq(&"9DTY3rv8xRa3CnoPoWJCMcQUSY7kUHZAoFKNsBhx8DDz")
                {
                    println!("found it {:#?}", liquidity);
                    ok = true;
                    break;
                }
            }
            assert_eq!(ok, true);
        }
        #[test]
        fn test_fetch() {
            let _ = fetch().unwrap();
        }
    }
}

pub mod token_list {
    //! configuration helpers for raydium related tokens

    use super::*;
    pub const RAYDIUM_API_TOKEN_LIST: &str = "sdk/token/raydium.mainnet.json";
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TokenList {
        pub name: String,
        pub version: Version,
        pub official: Vec<TokenListEntry>,
        // ignore this for now
        // pub blacklist: Vec<Value>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TokenListEntry {
        pub symbol: String,
        pub name: String,
        pub mint: String,
        pub decimals: i64,
        pub extensions: Extensions,
        pub icon: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Extensions {
        pub coingecko_id: Option<String>,
    }

    pub fn api_url() -> String {
        format_api_url(RAYDIUM_API_TOKEN_LIST)
    }

    pub async fn fetch_async() -> Result<TokenList> {
        let client = reqwest::Client::builder().build()?;
        let res = client.get(api_url()).send().await?;
        let data = res.json::<TokenList>().await?;
        Ok(data)
    }
    pub fn fetch() -> Result<TokenList> {
        let client = reqwest::blocking::Client::builder().build()?;
        let res = client.get(api_url()).send()?;
        let data = res.json::<TokenList>()?;
        Ok(data)
    }

    #[cfg(test)]
    mod test {
        use super::*;
        #[test]
        fn test_api_url() {
            assert_eq!(
                api_url(),
                "https://api.raydium.io/v2/sdk/token/raydium.mainnet.json"
            );
        }

        #[tokio::test]
        async fn test_fetch_async() {
            let _ = fetch_async().await.unwrap();
        }
        #[test]
        fn test_fetch() {
            let _ = fetch().unwrap();
        }
    }
}

/// basic helper function that combines the main api path
/// and the request path
pub fn format_api_url(request: &str) -> String {
    format!("{}/{}", RAYDIUM_API, request)
}
