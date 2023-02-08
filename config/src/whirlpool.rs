use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;
use std::str::FromStr;
pub const WHIRLPOOL_CONFIGS_API: &str = "https://api.mainnet.orca.so/v1/whirlpool/list";

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Whirlpools{
    whirlpools: Vec<Whirlpool>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Whirlpool {
    pub address: String,
    pub token_a: TokenInfo,
    pub token_b: TokenInfo,
    pub whitelisted: bool,
    pub tick_spacing: i64,
    pub price: f64,
    pub lp_fee_rate: f64,
    pub protocol_fee_rate: f64,
    pub whirlpools_config: String,
    pub modified_time_ms: Option<i64>,
    /// this field is not included in the response from orca's api
    /// so we include it here as an option to prevent (se|dese)rialization
    /// issues
    pub name: Option<String>,
    /// same thing as above, but includes the sum of lp and protocol fee rate
    /// for example: SOL-USDC-500
    pub formatted_name: Option<String>,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenInfo {
    pub mint: String,
    pub symbol: String,
    pub name: String,
    pub decimals: i64,
    #[serde(rename = "logoURI")]
    pub logo_uri: Option<String>,
    pub coingecko_id: Option<String>,
    pub whitelisted: Option<bool>,
    pub pool_token: Option<bool>,
}
impl Whirlpool {
    pub fn address(&self) -> Pubkey {
        Pubkey::from_str(&self.address).unwrap()
    }
    pub fn token_mint_a(&self) -> Pubkey {
        Pubkey::from_str(&self.token_a.mint).unwrap()
    }
    pub fn token_mint_b(&self) -> Pubkey {
        Pubkey::from_str(&self.token_b.mint).unwrap()
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
        for pool in self.whirlpools.iter_mut() {
            match guesser.guess_name(&pool.token_a.mint, &pool.token_b.mint) {
                Some(info) => {

                    pool.name = Some(info.market.clone());
                    pool.formatted_name = Some(format!(
                        "{}-{}",
                        pool.name.as_ref().unwrap(),
                        pool.lp_fee_rate + pool.protocol_fee_rate
                    ));

                }
                None => {
                    continue;
                }
            }
        }
        Ok(())
    }
    pub fn pool_by_address(&self, address: &str) -> Result<Whirlpool> {
        super::whirlpool::pool_by_address(&self.whirlpools, address)
    }
    pub fn pool_by_name(&self, name: &str) -> Result<Vec<Whirlpool>> {
        let pools = self
            .whirlpools
            .iter()
            .filter_map(|pool| {
                if let Some(pool_name) = &pool.name {
                    if pool_name.eq_ignore_ascii_case(name) {
                        return Some(pool.clone());
                    }
                }
                None
            })
            .collect::<Vec<_>>();
        if pools.is_empty() {
            return Err(anyhow!("failed to find pools matching name {}", name));
        }
        Ok(pools)
    }
}

pub fn pool_by_address(pools: &[Whirlpool], address: &str) -> Result<Whirlpool> {
    for pool in pools.iter() {
        if pool.address.eq(address) {
            return Ok(pool.clone());
        }
    }
    Err(anyhow!("failed to find pool matching address {}", address))
}

#[cfg(test)]
mod test {
    use super::*;
    #[tokio::test]
    async fn test_fetch_whirlpool_config() {
        let mut configs = Whirlpools::new().await.unwrap();
        configs.guess_names().await.unwrap();

        let pool_conf = configs.pool_by_name("USH-USDC").unwrap();
        assert!(pool_conf.len() >= 1);
        assert!(pool_conf[0].token_mint_a().to_string().eq("9iLH8T7zoWhY7sBmj1WK9ENbWdS1nL8n9wAxaeRitTa6"));
        assert!(pool_conf[0].token_mint_b().to_string().eq("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"));
        println!("found pools {}", pool_conf.len());
        let pool_conf =
            pool_by_address(&pool_conf, "ApLVWYdXzjoDhBHeRx6SnbFWv4MYjFMih5FijDQUJk5R").unwrap();
        assert_eq!(
            pool_conf.address,
            "ApLVWYdXzjoDhBHeRx6SnbFWv4MYjFMih5FijDQUJk5R".to_string()
        );
        assert_eq!(pool_conf.formatted_name.as_ref().unwrap(), "USH-USDC-0.0301");
        println!("{:#?}", pool_conf);

        let pool_conf = configs.pool_by_name("SOL-USDC").unwrap();
        assert!(pool_conf.len() >= 1);
        assert!(pool_conf[0].token_mint_a().to_string().eq("So11111111111111111111111111111111111111112"));
        assert!(pool_conf[0].token_mint_b().to_string().eq("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"));
        pool_conf.iter().for_each(|pool_conf| {
            assert_eq!(
                pool_conf.formatted_name.as_ref().unwrap(),
                format!("SOL-USDC-{}", pool_conf.lp_fee_rate + pool_conf.protocol_fee_rate).as_str()
            );
        });
        println!("found pools {}", pool_conf.len());
    }
}
