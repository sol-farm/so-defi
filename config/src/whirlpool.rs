use super::*;
use serde::{Serialize, Deserialize};
use solana_program::pubkey::Pubkey;
use std::str::FromStr;
use anyhow::{Result, anyhow};
pub const WHIRLPOOL_CONFIGS_API: &str = "https://mainnet-zp2-v2.orca.so/pools";

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Whirlpools(pub Vec<Whirlpool>);


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Whirlpool {
    pub address: String,
    pub whitelisted: bool,
    pub token_mint_a: String,
    pub token_mint_b: String,
    pub tick_spacing: i64,
    pub lps_fee_rate: f64,
    pub protocol_fee_rate: f64,
    /// this field is not included in the response from orca's api
    /// so we include it here as an option to prevent (se|dese)rialization
    /// issues
    pub name: Option<String>,
}

impl Whirlpool {
    pub fn address(&self) -> Pubkey {
        Pubkey::from_str(&self.address).unwrap()
    }
    pub fn token_mint_a(&self) -> Pubkey {
        Pubkey::from_str(&self.token_mint_a).unwrap()
    }
    pub fn token_mint_b(&self) -> Pubkey {
        Pubkey::from_str(&self.token_mint_b).unwrap()
    }
}


impl Whirlpools {
    pub async fn new() -> Result<Self> {
        let client = reqwest::Client::builder().build()?;
        let res = client.get(WHIRLPOOL_CONFIGS_API).send().await?;
        let data = res.json::<Whirlpools>().await?;
        Ok(data)
    }
    
            /// attempts to fill in the missing pool and farm name information using the solana
        /// token list to map coin/pc mints -> names
        pub async fn guess_names(&mut self) -> Result<()> {
            let guesser =
                so_defi_token_list::market_name_guesser::MarketNameGuesser::initialize().await?;
            for pool in self.0.iter_mut() {
                match guesser.guess_name(&pool.token_mint_a, &pool.token_mint_b) {
                    Some(info) => {
                        pool.name = Some(info.market.clone());
                    }
                    None => {
                        continue;
                    }
                }
            }
            Ok(())
        }
    pub fn pool_by_address(&self, address: &str) -> Result<Whirlpool> {
        for pool in self.0.iter() {
            if pool.address.eq(address) {
                return Ok(pool.clone());
            }
        }
        Err(anyhow!("failed to find pool matching address {}", address))
    }
    pub fn pool_by_name(&self, name: &str) -> Result<Whirlpool> {
        for pool in self.0.iter() {
            if let Some(pool_name) = &pool.name {
                if pool_name.eq(name) {
                    return Ok(pool.clone());
                }
            }
        }
        Err(anyhow!("failed to find pool matching name {}", name))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[tokio::test]
    async fn test_fetch_whirlpool_config() {
        let mut configs = Whirlpools::new().await.unwrap();
        configs.guess_names().await.unwrap();

        let pool_conf = configs.pool_by_name("USH-USDC").unwrap();
        assert_eq!(pool_conf.address, "ApLVWYdXzjoDhBHeRx6SnbFWv4MYjFMih5FijDQUJk5R".to_string());

    }
}