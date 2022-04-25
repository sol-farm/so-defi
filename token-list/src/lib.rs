//! provides a crate for parsing the solana token list file, and a module for guessing a market name
//! given it's asset pairs. this is needed for some protocols such as atrix which do not include market names
//! in the api results

pub mod market_name_guesser;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// solana token list file containing all tokens for us to parse
pub const TOKEN_LIST_FILE: &str = "https://raw.githubusercontent.com/solana-labs/token-list/main/src/tokens/solana.tokenlist.json";

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenList {
    pub name: String,
    #[serde(rename = "logoURI")]
    pub logo_uri: String,
    pub keywords: Vec<String>,
    pub timestamp: String,
    pub tokens: Vec<Token>,
    pub version: Version,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub chain_id: i64,
    pub address: String,
    pub symbol: String,
    pub name: String,
    pub decimals: i64,
    #[serde(rename = "logoURI")]
    pub logo_uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub major: i64,
    pub minor: i64,
    pub patch: i64,
}

/// fetches the token list asynchronously
pub async fn fetch_async() -> reqwest::Result<TokenList> {
    let client = reqwest::Client::builder().build()?;
    let data = client.get(TOKEN_LIST_FILE).send().await?.bytes().await?;
    Ok(serde_json::from_slice(&data[..]).unwrap())
}

#[cfg(test)]
mod test {
    use super::*;
    #[tokio::test]
    async fn test_fetch_async() {
        let _ = fetch_async().await.unwrap();
    }
}
