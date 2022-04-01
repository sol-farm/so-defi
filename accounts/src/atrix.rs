//! atrix account definitions taken from https://github.com/skaiba0/atrix-farm/blob/main/farmSdk/idl/farm.json

use solana_program::{self, pubkey::Pubkey};
use borsh_derive::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Default, Clone, BorshDeserialize, BorshSerialize)]
pub struct FarmAccount {
    _buffer: [u8; 8],
    pub base: Pubkey,
    pub bump: u8,
    pub state_mint: Pubkey,
    pub farm_stake_token_account: Pubkey,
    pub crop_accounts: [Option<Pubkey>; 4],
    pub authority: Pubkey,
}

#[derive(Debug, Default, Clone, BorshDeserialize, BorshSerialize)]
pub struct CropAccount {
    _buffer: [u8; 8],
    pub bump: u8,
    pub authority: Pubkey,
    pub farm_account: Pubkey,
    pub reward_mint: Pubkey,
    pub reward_amount_per_day: u64,
    pub rewards_locked: bool,
    pub crop_reward_token_account: Pubkey,
    pub accrued_reward_per_stake: u128,
    pub last_reward_timestamp: i64,
}

#[derive(Debug, Default, Clone, BorshDeserialize, BorshSerialize)]
pub struct StakerAccount {
    _buffer: [u8; 8],
    pub bump: u8,
    pub farm_account: Pubkey,
    pub authority: Pubkey,
    pub staked_amount: u64,
}

#[derive(Debug, Default, Clone, BorshDeserialize, BorshSerialize)]
pub struct HarvesterAccount {
    _buffer: [u8; 8],
    pub bump: u8,
    pub crop_account: Pubkey,
    pub reward_debt: u128,
    pub earned_rewards: u64,
    pub authority: Pubkey,
}


#[cfg(test)]
mod test {
    use super::*;
    use borsh::BorshDeserialize;
    use static_pubkey::static_pubkey;
    use solana_program::{self, system_program};
    use solana_client::rpc_client;
    #[test]
    fn test_load_farm_account() {
        let test_key = static_pubkey!("J55atXt8BnF99YUC4AmpHY2VuxZ6XbBTjL7dHaePid42");
        let rpc = rpc_client::RpcClient::new("https://ssc-dao.genesysgo.net".to_string());
        let farm_account_data = rpc.get_account_data(&test_key).unwrap();
        let farm_account = FarmAccount::deserialize(&mut &farm_account_data[..]).unwrap();
        println!("{:#?}", farm_account);
        for crop_account in farm_account.crop_accounts.iter() {
            if let Some(crop_account) = crop_account {
                let crop_account_data = rpc.get_account_data(crop_account).unwrap();
                println!("crop_address {}", crop_account);
                let crop_account = CropAccount::deserialize(&mut &crop_account_data[..]).unwrap();
                println!("crop_account {:#?}", crop_account);
            }
        }
    }
}