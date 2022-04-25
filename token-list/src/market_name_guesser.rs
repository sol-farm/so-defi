use std::collections::HashMap;

use crate::TokenList;

pub struct MarketNameGuesser {
    token_list: TokenList,
    /// a map of an asset address -> (name, symbol)
    assets: HashMap<String, (String, String)>,
}

pub struct GuessedName {
    pub market: String,
    // (name, symbol)
    pub coin_info: (String, String),
    // (name, symbol)
    pub pc_info: (String, String),
}

impl MarketNameGuesser {
    pub async fn initialize() -> reqwest::Result<MarketNameGuesser> {
        let token_list = crate::fetch_async().await?;
        let mut assets = HashMap::with_capacity(token_list.tokens.len());
        for token in token_list.tokens.iter() {
            assets.insert(
                token.address.clone(),
                (token.name.clone(), token.symbol.clone()),
            );
        }
        Ok(MarketNameGuesser { token_list, assets })
    }
    /// attempts to guess the market name, returning None if the guess failed
    pub fn guess_name(&self, coin_address: &str, pc_address: &str) -> Option<GuessedName> {
        let coin_info = match self.assets.get(coin_address) {
            Some(coin_info) => coin_info,
            None => return None,
        };
        let pc_info = match self.assets.get(pc_address) {
            Some(pc_info) => pc_info,
            None => return None,
        };
        Some(GuessedName {
            market: format!("{}-{}", coin_info.1.clone(), pc_info.1.clone()),
            coin_info: coin_info.clone(),
            pc_info: pc_info.clone(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[tokio::test]
    async fn test_market_name_guesser() {
        let guesser = MarketNameGuesser::initialize().await.unwrap();
        let guess = guesser.guess_name(
            "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB",
            "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
        );
        assert!(guess.is_some());
        let guess = guess.unwrap();
        assert_eq!(guess.market, "USDT-USDC".to_string());
    }
}
