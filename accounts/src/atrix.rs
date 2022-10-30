//! atrix account definitions taken from https://github.com/skaiba0/atrix-farm/blob/main/farmSdk/idl/farm.json

use borsh_derive::{BorshDeserialize, BorshSerialize};
use solana_program::{self, pubkey::Pubkey};

pub mod farm {
    use super::*;
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
        use super::farm::*;
        use borsh::BorshDeserialize;
        use solana_client::rpc_client;
        use solana_program::{self};
        use static_pubkey::static_pubkey;
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
                    let crop_account =
                        CropAccount::deserialize(&mut &crop_account_data[..]).unwrap();
                    println!("crop_account {:#?}", crop_account);
                }
            }
        }
    }
}

pub mod pool {
    use super::*;

    #[derive(Debug, Default, Clone, BorshDeserialize, BorshSerialize)]
    pub struct ProtocolAccount {
        _buffer: [u8; 8],
        pub authority: Pubkey,
        pub bump: u8,
        pub lp_fee_numerator: u16,
        pub protocol_fee_numerator: u16,
        pub fee_denominator: u16,
        pub max_cancel_per_ix: u8,
        pub max_place_per_ix: u8,
        pub max_place_post_liq: u8,
        pub order_proportion_numerators: [u16; 12],
        pub order_proportion_len: u8,
        pub order_proportion_denominator: u16,
        pub crank_sol_account: Pubkey,
        pub pool_init_crank_fee: u64,
        pub sol_bond: u64,
    }

    #[derive(Debug, Default, Clone, BorshDeserialize, BorshSerialize)]
    pub struct PoolAccount {
        _buffer: [u8; 8],
        pub coin_mint: Pubkey,
        pub pc_mint: Pubkey,
        pub market: Pubkey,
        pub open_orders: Pubkey,
        pub pool_coin_account: Pubkey,
        pub pool_pc_account: Pubkey,
        pub pool_lp_account: Pubkey,
        pub lp_mint: Pubkey,
        pub first_placed: bool,
        pub order_index: u8,
        pub coin_current_protocol_fees: u64,
        pub pc_current_protocol_fees: u64,
        pub ixi: u8,
        pub icx: u8,
        pub client_order_id: u64,
        pub order_proportion_numerators: [u16; 12],
        pub pool_type: u8,
        pub stable_swap_amp_coef: u64,
        pub coin_decimals: u8,
        pub pc_decimals: u8,
        pub last_ask_coin: u64,
        pub last_ask_pc: u64,
        pub last_bid_coin: u64,
        pub last_bid_pc: u64,
        pub version: u64,
        pub placed_asks: [PlacedOrder; 12],
        pub placed_bids: [PlacedOrder; 12],
        pub pool_coin_amt: u64,
        pub pool_pc_amt: u64,
        pub mm_active: bool,
    }

    #[derive(Debug, Default, Clone, Copy, BorshDeserialize, BorshSerialize)]
    pub struct PlacedOrder {
        pub limit_price: u64,
        pub coin_qty: u64,
        pub max_native_pc_qty_including_fees: u64,
        pub client_order_id: u64,
    }

    #[cfg(test)]
    mod test {
        use crate::atrix::pool::PoolAccount;

        use super::ProtocolAccount;
        use borsh::BorshDeserialize;
        use solana_client::rpc_client;
        use solana_program::{self};
        use static_pubkey::static_pubkey;
        #[test]
        fn test_load_pool_account() {
            let test_key = static_pubkey!("7yQzTZ9nMpsSePZxgxWpGMK62Zrkr9u7ngEsxyC9j7pG");
            let rpc = rpc_client::RpcClient::new("https://ssc-dao.genesysgo.net".to_string());
            let farm_account_data = rpc.get_account_data(&test_key).unwrap();
            let pool_account = PoolAccount::deserialize(&mut &farm_account_data[..]).unwrap();
            assert_eq!(
                pool_account.coin_mint.to_string(),
                "smbdJcLBrtKPhjrWCpDv5ABdJwz2vYo3mm6ojmePL3t".to_string()
            );
        }
        #[test]
        fn test_load_protocol_account() {
            let test_key = static_pubkey!("3uTzTX5GBSfbW7eM9R9k95H7Txe32Qw3Z25MtyD2dzwC");
            let rpc = rpc_client::RpcClient::new("https://ssc-dao.genesysgo.net".to_string());
            let farm_account_data = rpc.get_account_data(&test_key).unwrap();
            let protocol_account =
                ProtocolAccount::deserialize(&mut &farm_account_data[..]).unwrap();
            println!("{:#?}", protocol_account);
        }
    }
}
