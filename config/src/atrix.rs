//! provides helepr functions for parsing atrix's configuration api

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// the hostname and main path for Atrix's
pub const ATRIX_API: &str = "https://api.atrix.finance/api";

/// a common type used to denote the raydium api version
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub major: i64,
    pub minor: i64,
    pub patch: i64,
}

pub mod tvl_list {
    //! configuration helpers for the atrix's tvl api request

    use super::*;
    pub const ATRIX_API_TVL_LIST: &str = "tvl";

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TvlList {
        pub tvl: f64,
        pub pools: Vec<Pool>,
        pub farms: Vec<Farm>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Pool {
        pub pool_key: String,
        pub tvl: Option<f64>,
        pub lp_mint: String,
        pub lp_supply: f64,
        pub coin_mint: String,
        pub coin_tokens: f64,
        pub coin_decimals: i64,
        pub pc_mint: String,
        pub pc_tokens: f64,
        pub pc_decimals: i64,
        pub farms: Vec<Farm>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Farm {
        pub key: String,
        pub tvl: f64,
        pub apy: f64,
    }

    pub fn api_url() -> String {
        format_api_url(ATRIX_API_TVL_LIST)
    }

    pub async fn fetch_async() -> Result<TvlList> {
        let client = reqwest::Client::builder().build()?;
        let res = client.get(api_url()).send().await?;
        let data = res.json::<TvlList>().await?;
        Ok(data)
    }
    pub fn fetch() -> Result<TvlList> {
        let client = reqwest::blocking::Client::builder().build()?;
        let res = client.get(api_url()).send()?;
        let data = res.json::<TvlList>()?;
        Ok(data)
    }

    #[cfg(test)]
    mod test {
        use super::*;
        #[test]
        fn test_api_url() {
            assert_eq!(api_url(), "https://api.atrix.finance/api/tvl");
        }

        #[tokio::test]
        async fn test_fetch_async() {
            let results = fetch_async().await.unwrap();
            assert!(results.pools.len() > 0);
        }
        #[test]
        fn test_fetch() {
            let results = fetch().unwrap();
            assert!(results.pools.len() > 0);
        }
    }
}

pub mod pools_list {
    //! configuration helpers for the raydium pairs api request

    use super::*;
    pub const ATRIX_API_POOLS_LIST: &str = "pools";

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PoolsList {
        pub pools: Vec<Pool>,
    }
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Pool {
        pub id: String,
        #[serde(rename = "created_at")]
        pub created_at: String,
        #[serde(rename = "coin_mint")]
        pub coin_mint: String,
        #[serde(rename = "pc_mint")]
        pub pc_mint: String,
        pub market: String,
        #[serde(rename = "open_orders")]
        pub open_orders: String,
        #[serde(rename = "pool_coin_account")]
        pub pool_coin_account: String,
        #[serde(rename = "pool_pc_account")]
        pub pool_pc_account: String,
        #[serde(rename = "pool_lp_account")]
        pub pool_lp_account: String,
        #[serde(rename = "lp_mint")]
        pub lp_mint: String,
        #[serde(rename = "pool_type")]
        pub pool_type: i64,
        #[serde(rename = "stableswap_amp_coef")]
        pub stableswap_amp_coef: i64,
        #[serde(rename = "pool_coin_amt")]
        pub pool_coin_amt: String,
        #[serde(rename = "pool_pc_amt")]
        pub pool_pc_amt: String,
        pub farms: Vec<Farm>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Farm {
        pub key: String,
        pub tvl: f64,
        pub apy: f64,
    }

    pub fn api_url() -> String {
        format_api_url(ATRIX_API_POOLS_LIST)
    }

    pub async fn fetch_async() -> Result<PoolsList> {
        let client = reqwest::Client::builder().build()?;
        let res = client.get(api_url()).send().await?;
        let data = res.json::<PoolsList>().await?;
        Ok(data)
    }
    pub fn fetch() -> Result<PoolsList> {
        let client = reqwest::blocking::Client::builder().build()?;
        let res = client.get(api_url()).send()?;
        let data = res.json::<PoolsList>()?;
        Ok(data)
    }

    #[cfg(test)]
    mod test {
        use super::*;
        #[test]
        fn test_api_url() {
            assert_eq!(api_url(), "https://api.atrix.finance/api/pools");
        }

        #[tokio::test]
        async fn test_fetch_async() {
            let result = fetch_async().await.unwrap();
            assert!(result.pools.len() > 0);
            println!("pool {:#?}", result.pools[result.pools.len() - 1]);
        }
        #[test]
        fn test_fetch() {
            let result = fetch().unwrap();
            assert!(result.pools.len() > 0);
        }
    }
}

pub mod farms_list {
    //! configuration helpers for raydium farms

    use super::*;
    pub const ATRIX_API_FARMS_LIST: &str = "farms";

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FarmList {
        pub farms: Vec<Farm>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Farm {
        pub id: String,
        #[serde(rename = "created_at")]
        pub created_at: String,
        #[serde(rename = "farm_stake_token_account")]
        pub farm_stake_token_account: String,
        #[serde(rename = "crop_accounts")]
        pub crop_accounts: Vec<Option<String>>,
        pub authority: String,
        #[serde(rename = "stake_mint")]
        pub stake_mint: String,
        pub apy: f64,
        pub tvl: f64,
    }

    pub fn api_url() -> String {
        format_api_url(ATRIX_API_FARMS_LIST)
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
            assert_eq!(api_url(), "https://api.atrix.finance/api/farms");
        }

        #[tokio::test]
        async fn test_fetch_async() {
            let result = fetch_async().await.unwrap();
            assert!(result.farms.len() > 0);
        }
        #[test]
        fn test_fetch() {
            let result = fetch().unwrap();
            assert!(result.farms.len() > 0);
        }
    }
}

/// basic helper function that combines the main api path
/// and the request path
pub fn format_api_url(request: &str) -> String {
    format!("{}/{}", ATRIX_API, request)
}
