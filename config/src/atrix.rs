//! provides helepr functions for parsing atrix's configuration api

use anyhow::Result;
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;
use std::str::FromStr;

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
        /// this is not returned from atrix's api, however
        /// we include this as an option to avoid deserialization
        /// but allow manually updating the object with the pool name
        pub name: Option<String>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Farm {
        pub key: String,
        pub tvl: f64,
        pub apy: f64,
    }

    impl Pool {
        pub fn pool_key(&self) -> Pubkey {
            Pubkey::from_str(&self.pool_key).unwrap()
        }
        pub fn lp_mint(&self) -> Pubkey {
            Pubkey::from_str(&self.lp_mint).unwrap()
        }
        pub fn coin_mint(&self) -> Pubkey {
            Pubkey::from_str(&self.coin_mint).unwrap()
        }
        pub fn pc_mint(&self) -> Pubkey {
            Pubkey::from_str(&self.pc_mint).unwrap()
        }
    }

    impl Farm {
        pub fn key(&self) -> Pubkey {
            Pubkey::from_str(&self.key).unwrap()
        }
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

    use super::{*, farms_list::FarmsList};
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
        /// this is not returned from atrix's api, however
        /// we include this as an option to avoid deserialization
        /// but allow manually updating the object with the pool name
        pub name: Option<String>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Farm {
        pub key: String,
        pub apy: f64,
    }

    impl Pool {
        pub fn id(&self) -> Pubkey {
            Pubkey::from_str(&self.id).unwrap()
        }
        pub fn coin_mint(&self) -> Pubkey {
            Pubkey::from_str(&self.coin_mint).unwrap()
        }
        pub fn pc_mint(&self) -> Pubkey {
            Pubkey::from_str(&self.pc_mint).unwrap()
        }
        pub fn market(&self) -> Pubkey {
            Pubkey::from_str(&self.market).unwrap()
        }
        pub fn open_orders(&self) -> Pubkey {
            Pubkey::from_str(&self.open_orders).unwrap()
        }
        pub fn pool_coin_account(&self) -> Pubkey {
            Pubkey::from_str(&self.pool_coin_account).unwrap()
        }
        pub fn pool_pc_account(&self) -> Pubkey {
            Pubkey::from_str(&self.pool_pc_account).unwrap()
        }
        pub fn pool_lp_account(&self) -> Pubkey {
            Pubkey::from_str(&self.pool_lp_account).unwrap()
        }
        pub fn lp_mint(&self) -> Pubkey {
            Pubkey::from_str(&self.lp_mint).unwrap()
        }
    }

    impl Farm {
        pub fn key(&self) -> Pubkey {
            Pubkey::from_str(&self.key).unwrap()
        }
    }

    impl PoolsList {
        /// intiailizes the PoolList object, populating with all values
        /// returned from atrix's API
        pub async fn initialize() -> Result<PoolsList> {
            fetch_async().await
        }
        /// attempts to fill in the missing pool and farm name information using the solana
        /// token list to map coin/pc mints -> names
        pub async fn guess_names(&mut self, farms_list: &mut FarmsList) -> Result<()> {
            let guesser =
                so_defi_token_list::market_name_guesser::MarketNameGuesser::initialize().await?;
            for pool in self.pools.iter_mut() {
                match guesser.guess_name(&pool.coin_mint, &pool.pc_mint) {
                    Some(info) => {
                        if let Some(name) = pool.name.as_mut() {
                            *name = info.market.clone();
                        } else {
                            pool.name = Some(info.market.clone());
                        }
                        for (idx, pool_farm) in pool.farms.iter().enumerate() {
                            for farm in farms_list.farms.iter_mut() {
                                if farm.id.eq(&pool_farm.key) {
                                    // for farms with an idx > 0 (which will be very few)
                                    // append the idx to the end of the name
                                    let farm_name = if idx > 0 {
                                        format!("{}-{}", info.market.clone(), idx)
                                    } else {
                                        info.market.clone()
                                    };

                                    if let Some(name) = farm.name.as_mut() {
                                        *name = farm_name;
                                    } else {
                                        farm.name = Some(farm_name);
                                    }
                                }
                            }
                        }
                    }
                    None => {
                        println!("failed to guess name for pool_id {},pc {}, coin {}", pool.id, pool.pc_mint, pool.coin_mint);
                        continue;
                    }
                }
            }
            Ok(())
        }
    }
    impl From<Vec<Pool>> for PoolsList {
        fn from(pools: Vec<Pool>) -> Self {
            Self::from(&pools)
        }
    }

    impl From<&Vec<Pool>> for PoolsList {
        fn from(pools: &Vec<Pool>) -> Self {
            Self {
                pools: pools.clone(),
            }
        }
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
        #[tokio::test]
        async fn test_pool_list() {
            let mut pool_list = PoolsList::initialize().await.unwrap();
            let mut farm_list = FarmsList::initialize().await.unwrap();
            pool_list.guess_names(&mut farm_list).await.unwrap();
        }
    }
}

pub mod farms_list {
    //! configuration helpers for raydium farms

    use super::*;
    pub const ATRIX_API_FARMS_LIST: &str = "farms";

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FarmsList {
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
        /// this is not returned from atrix's api, however
        /// we include this as an option to avoid deserialization
        /// but allow manually updating the object with the pool name
        pub name: Option<String>,
    }
    impl FarmsList {
        /// intiailizes the PoolList object, populating with all values
        /// returned from atrix's API
        pub async fn initialize() -> Result<FarmsList> {
            fetch_async().await
        }
    }
    impl Farm {
        pub fn id(&self) -> Pubkey {
            Pubkey::from_str(&self.id).unwrap()
        }
        pub fn farm_stake_token_account(&self) -> Pubkey {
            Pubkey::from_str(&self.farm_stake_token_account).unwrap()
        }
        pub fn crop_accounts(&self) -> Vec<Option<Pubkey>> {
            self.crop_accounts
                .iter()
                .map(|val| {
                    if let Some(val) = val {
                        match Pubkey::from_str(val.as_str()) {
                            Ok(val_key) => Some(val_key),
                            Err(_) => None,
                        }
                    } else {
                        None
                    }
                })
                .collect()
        }

    }
    pub fn api_url() -> String {
        format_api_url(ATRIX_API_FARMS_LIST)
    }

    pub async fn fetch_async() -> Result<FarmsList> {
        let client = reqwest::Client::builder().build()?;
        let res = client.get(api_url()).send().await?;
        let data = res.json::<FarmsList>().await?;
        Ok(data)
    }
    pub fn fetch() -> Result<FarmsList> {
        let client = reqwest::blocking::Client::builder().build()?;
        let res = client.get(api_url()).send()?;
        let data = res.json::<FarmsList>()?;
        Ok(data)
    }

    impl From<Vec<Farm>> for FarmsList {
        fn from(farms: Vec<Farm>) -> Self {
            Self::from(&farms)
        }
    }

    impl From<&Vec<Farm>> for FarmsList {
        fn from(farms: &Vec<Farm>) -> Self {
            Self {
                farms: farms.clone(),
            }
        }
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
