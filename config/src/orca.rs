//! providers helper functions for parsing Orca's configuration api
//! and generating rust types corresponding to the emitted JSON

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const ORCA_CONFIGS_API: &str = "https://api.orca.so/configs";

/// the resposne body from orca's configuration api
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrcaConfigsApiResponse {
    pub aquafarms: HashMap<String, AquaFarm>,
    pub collectibles: HashMap<String, Collectible>,
    pub double_dips: HashMap<String, DoubleDip>,
    pub pools: HashMap<String, Pool>,
    pub program_ids: ProgramIds,
    pub tokens: HashMap<String, Token>,
    pub coingecko_ids: HashMap<String, String>,
    pub ftx_ids: HashMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AquaFarm {
    pub account: String,
    pub nonce: u8,
    pub token_program_id: String,
    pub emissions_authority: String,
    pub remove_rewards_authority: String,
    pub base_token_mint: String,
    pub base_token_vault: String,
    pub reward_token_mint: String,
    pub reward_token_vault: String,
    pub farm_token_mint: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DoubleDip {
    pub account: String,
    pub nonce: u8,
    pub token_program_id: String,
    pub emissions_authority: String,
    pub remove_rewards_authority: String,
    pub base_token_mint: String,
    pub base_token_vault: String,
    pub reward_token_mint: String,
    pub reward_token_vault: String,
    pub farm_token_mint: String,
    pub date_start: String,
    pub date_end: String,
    pub total_emissions: String,
    pub custom_gradient_start_color: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collectible {
    pub mint: String,
    pub decimals: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Pool {
    pub account: String,
    pub authority: String,
    pub nonce: u8,
    pub pool_token_mint: String,
    pub token_account_a: String,
    pub token_account_b: String,
    pub fee_account: String,
    pub fee_numerator: u64,
    pub fee_denominator: u64,
    pub owner_trade_fee_numerator: u64,
    pub owner_trade_fee_denominator: u64,
    pub owner_withdraw_fee_numerator: u64,
    pub host_fee_numerator: u64,
    pub token_a_name: String,
    pub token_b_name: String,
    pub curve_type: String,
    pub deprecated: Option<bool>,
    pub program_version: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramIds {
    pub serum_token_swap: String,
    pub token_swap_v2: String,
    pub token_swap: String,
    pub token: String,
    pub aquafarm: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub mint: String,
    pub name: String,
    pub decimals: u8,
    pub fetch_price: Option<bool>,
}

impl OrcaConfigsApiResponse {
    pub async fn fetch_orca_config() -> Result<Self> {
        let client = reqwest::Client::builder().build()?;
        let res = client.get(ORCA_CONFIGS_API).send().await?;
        let data = res.json::<Self>().await?;
        Ok(data)
    }
    /// used to lookup pool information for the given pool matching `name`
    /// if aquafarm is true, search query becomes `name[aquafarm]`
    /// if stable is true, search query becomes `name[stable]`
    /// if stable & aquafarm is true, search query becomes `name[stable][aquafarm]`
    pub fn find_pool(&self, name: &str, stable: bool, aquafarm: bool) -> Result<Pool> {
        let name = format_orca_amm_name(name, stable, aquafarm);
        for pool in self.pools.iter() {
            if pool.0.eq(&name) {
                return Ok(pool.1.clone());
            }
        }
        Err(anyhow!("failed to find pool for {}", name))
    }
    /// used to lookup information for an aquafarm, returning a tuple of the
    /// pool and aquafarm config
    pub fn find_aquafarm(&self, name: &str, stable: bool) -> Result<(Pool, AquaFarm)> {
        let pool = self.find_pool(name, stable, true)?;
        for farm in self.aquafarms.iter() {
            if farm.0.eq(&pool.account) {
                return Ok((pool, farm.1.clone()));
            }
        }
        Err(anyhow!("failed to find aquafarm for {}", name))
    }
    /// used to lookup information for a double dip
    pub fn find_double_dip(&self, name: &str, stable: bool) -> Result<(Pool, DoubleDip, AquaFarm)> {
        let pool = self.find_pool(name, stable, true)?;
        for doubledip in self.double_dips.iter() {
            if doubledip.0.eq(&pool.account) {
                for aquafarm in self.aquafarms.iter() {
                    if aquafarm.0.eq(&pool.account) {
                        return Ok((pool, doubledip.1.clone(), aquafarm.1.clone()));
                    }
                }
            }
        }
        Err(anyhow!("failed to find doubledip for {}", name))
    }
}

pub fn format_orca_amm_name(name: &str, stable: bool, aquafarm: bool) -> String {
    // orca uses a platform specifier for pairs with names that would conflict
    // with other vaults
    let name = if name.split('-').count() == 3 {
        let lp_name_str = name.to_string();
        let parts: Vec<_> = lp_name_str.split('-').collect();
        let mut lp_name_parsed = String::with_capacity(name.len() - 5); // 5 for '-ORCA'
        for (idx, part) in parts.iter().enumerate() {
            if idx == parts.len() - 1 {
                break;
            }
            lp_name_parsed.push_str(*part);
            if idx != parts.len() - 2 {
                lp_name_parsed.push('/');
            }
        }
        lp_name_parsed
    } else {
        name.replace('-', "/")
    };
    // scnSOL previously used to be labeled as SOCN
    // so handle that edgecase, todo(bonedaddy): test
    let name = name.replace("SOCN", "scnSOL");
    let name = if stable && !name.contains("[stable]") {
        format!("{}[stable]", name)
    } else {
        name
    };
    let name = if aquafarm && !name.contains("[aquafarm]") {
        format!("{}[aquafarm]", name)
    } else {
        name
    };
    name
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_format_orca_amm_name() {
        let name_one = "SAMO-USDC[stable][aquafarm]".to_string();
        let name_two = "SAMO-USDC".to_string();
        assert_eq!(
            format_orca_amm_name(&name_one, true, true),
            "SAMO/USDC[stable][aquafarm]"
        );
        assert_eq!(
            format_orca_amm_name(&name_two, false, true),
            "SAMO/USDC[aquafarm]"
        );
        assert_eq!(
            format_orca_amm_name(&name_two, true, false),
            "SAMO/USDC[stable]"
        );
    }
    #[tokio::test]
    async fn test_orca_config() {
        let orca_config = OrcaConfigsApiResponse::fetch_orca_config().await.unwrap();

        let pool_config = orca_config.find_pool("SOL/USDC", false, false).unwrap();
        assert_eq!(
            pool_config.account,
            "6fTRDD7sYxCN7oyoSQaN1AWC3P2m8A6gVZzGrpej9DvL".to_string()
        );

        let pool_config = orca_config.find_pool("SOL/USDC", false, true).unwrap();
        assert_eq!(
            pool_config.account,
            "EGZ7tiLeH62TPV1gL8WwbXGzEPa9zmcpVnnkPKKnrE2U".to_string()
        );

        let aquafarm_config = orca_config.find_aquafarm("SOL/USDC", false).unwrap();
        assert_eq!(aquafarm_config.0, pool_config);
        println!(
            "sol/usdc aquafarm information\npool {:#?}\nfarm {:#?}",
            aquafarm_config.0, aquafarm_config.1
        );

        let pool_config = orca_config.find_pool("LIQ/USDC", false, true).unwrap();
        let doubledip_config = orca_config.find_double_dip("LIQ/USDC", false).unwrap();
        assert_eq!(doubledip_config.0, pool_config);
        println!(
            "liq/usdc doubledip information\npool {:#?}\nfarm {:#?}\ndouble dip {:#?}",
            doubledip_config.0, doubledip_config.1, doubledip_config.0,
        );
    }
}
