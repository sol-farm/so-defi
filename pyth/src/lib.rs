use borsh::BorshDeserialize;
use pyth_sdk::PriceFeed;
use pyth_sdk::Price;
use pyth_sdk::PriceStatus;
use solana_program::clock::Clock;


pub struct PythPriceAccount {
    pub feed: PriceFeed,
    pub status: PriceStatus,
    pub price: Price,
    pub publish_time: i64,
}


impl PythPriceAccount {
    pub fn new(data: &mut &[u8], clock: Option<Clock>) -> PythPriceAccount {
        let price_feed = PriceFeed::deserialize(data).unwrap();
        let status = price_feed.status;
        let price = price_feed.get_ema_price().unwrap();
        let publish_time = price_feed.publish_time;
        if let Some(clock) = clock {
            if let Some(diff) = clock.unix_timestamp.checked_sub(publish_time) {
                let diff_duration = std::time::Duration::from_secs(diff as u64);
                print!("{:#?}", diff_duration);
            }
        }
        PythPriceAccount { 
            feed: price_feed,
            status,
            price,
            publish_time
        }
    }
}



#[cfg(test)]
mod test {
    use solana_sdk::commitment_config::CommitmentConfig;
    use solana_client::rpc_client::RpcClient;
    use static_pubkey::static_pubkey;
    pub fn get_clock_account(rpc: &RpcClient, commitment: CommitmentConfig) -> Result<solana_sdk::sysvar::clock::Clock> {
        use solana_sdk::account_info::IntoAccountInfo;
        use solana_client::rpc_config::RpcAccountInfoConfig;
        use solana_account_decoder::UiAccountEncoding;
        use solana_sdk::sysvar::Sysvar;
        match rpc.get_multiple_accounts_with_config(
            &[solana_program::sysvar::clock::id()],
            RpcAccountInfoConfig {
                encoding: Some(UiAccountEncoding::Base64),
                data_slice: None,
                commitment: Some(commitment),
                min_context_slot: None,
            },
        ) {
            // this can technically panic
            Ok(mut response) => match std::mem::take(&mut response.value[0]) {
                Some(account) => {
                    let mut clock_tup = (solana_program::sysvar::clock::id(), account);
                    let clock_acct = clock_tup.into_account_info();
                    let clock = solana_sdk::sysvar::clock::Clock::from_account_info(&clock_acct)?;
                    Ok(clock)
                }
                None => Err(anyhow!("accounts is None")),
            },
            Err(err) => Err(anyhow!("failed to load accounts {:#?}", err)),
        }
    }
    
    
    
    use super::*;
    #[test]
    fn test_pyth_price_account() {
        let pyth_feed = static_pubkey!("Gnt27xtC473ZT2Mw5u8wZ68Z3gULkSTb5DuxJy7eJotD");
        let rpc = RpcClient::new("https://ssc-dao.genesysgo.net".to_string());
        let price_feed_data = rpc.get_account_data(pubkey).unwrap();
        let clock = get_clock_account(&rpc, CommitmentConfig::confirmed).unwrap();
        let price_feed = PythPriceAccount::new(&mut &price_feed_data[..], None);

    }
}