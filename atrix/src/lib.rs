use solana_program;
use solana_program::pubkey::Pubkey;
use static_pubkey::static_pubkey;

pub mod prelude {
    pub use super::addresses;
    pub use super::instructions;
    pub use solana_program;
    pub use static_pubkey::static_pubkey;
    pub use so_defi_accounts::atrix as atrix_accounts;
}

pub mod addresses {
    use super::*;

    /// atrix farm program address
    pub const FARM_PROGRAM_ID: Pubkey =
        static_pubkey!("BLDDrex4ZSWBgPYaaH6CQCzkJXWfzCiiur9cSFJT8t3x");
    /// atrix pool program
    pub const POOL_PROGRAM_ID: Pubkey =
        static_pubkey!("HvwYjjzPbXWpykgVZhqvvfeeaSraQVnTiQibofaFw9M7");
    /// address of the protocol account
    pub const PROTOCOL_ACCOUNT: Pubkey = static_pubkey!("3uTzTX5GBSfbW7eM9R9k95H7Txe32Qw3Z25MtyD2dzwC");
    /// serum v3 dex program address
    pub const SERUM_DEX_PROGRAM_ID: Pubkey = static_pubkey!("9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin");
    pub const FARM_SEED: &[u8; 10] = b"atrix-farm";
    pub const CROP_SEED: &[u8; 15] = b"atrix-farm-crop";
    pub const FARM_STAKE_SEED: &[u8; 16] = b"atrix-farm-stake";
    pub const FARM_HARVESTER_SEED: &[u8; 20] = b"atrix-farm-harvester";
    pub const POOL_LP_MINT_SEED: &[u8; 18] = b"atrix-pool-lp-mint";
    pub const OPEN_ORDERS_V2_SEED: &[u8; 20] = b"atrix-open-orders-v2";

    pub fn find_farm_address(base: Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[FARM_SEED, base.as_ref()], &FARM_PROGRAM_ID)
    }

    pub fn find_crop_address(farm_key: Pubkey, reward_mint: Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[CROP_SEED, farm_key.as_ref(), reward_mint.as_ref()],
            &FARM_PROGRAM_ID,
        )
    }

    pub fn find_staker_address(farm_key: Pubkey, authority: Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[FARM_STAKE_SEED, authority.as_ref(), farm_key.as_ref()],
            &FARM_PROGRAM_ID,
        )
    }

    pub fn find_harvester_address(crop_key: Pubkey, authority: Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[FARM_HARVESTER_SEED, authority.as_ref(), crop_key.as_ref()],
            &FARM_PROGRAM_ID,
        )
    }

    #[cfg(test)]
    mod test {
        use solana_program::system_program;

        use super::*;
        #[test]
        fn test_derive_addresses() {
            let base = static_pubkey!("93n4XBvCvKxvv4rhB1x5ACR2TvEt1hQ5P5o3a2w1yTKM");
            let (got_farm, got_nonce) = find_farm_address(base);
            assert_eq!(
                got_farm.to_string(),
                "J55atXt8BnF99YUC4AmpHY2VuxZ6XbBTjL7dHaePid42".to_string()
            );
            assert_eq!(got_nonce, 255);

            let reward_mint = static_pubkey!("MNDEFzGvMt87ueuHvVU9VcTqsAP5b3fTGPsHuuPA5ey");

            let (got_crop, got_crop_nonce) = find_crop_address(got_farm, reward_mint);
            assert_eq!(
                got_crop.to_string(),
                "GcAYkGrZx97u3wUVkjz4z74M2NZhBq3V7bWXmyadvdiC".to_string()
            );
            assert_eq!(got_crop_nonce, 255);

            let crop_auth = static_pubkey!("AufL1ZuuAZoX7jBw8kECvjUYjfhWqZm13hbXeqnLMhFu");

            let (got_harvester, got_harvester_nonce) = find_harvester_address(got_crop, crop_auth);
            assert_eq!(
                got_harvester.to_string(),
                "DxHDqv8fABj56GHMa2PaSuou2NGWe2txXjTmjuC8o45J".to_string()
            );
            assert_eq!(got_harvester_nonce, 255);

            let (got_staker, got_staker_nonce) =
                find_staker_address(got_farm, system_program::id());
            assert_eq!(
                got_staker.to_string(),
                "G3sab7XGM2WcBQzdgV6SMv64QVuTHAJsW2uvnuDRxikK".to_string()
            );
            assert_eq!(got_staker_nonce, 254);
        }
    }
}

pub mod instructions {
    use super::*;
    use addresses;
    use solana_program::{
        instruction::{AccountMeta, Instruction},
        system_program, sysvar,
    };
    use sighashdb::GlobalSighashDB;
    pub mod farm {
        use super::*;
        pub fn new_create_staker_account_ix(
            farm_key: Pubkey,
            authority: Pubkey,
            staker_account: Pubkey,
            staker_account_bump: u8,
        ) -> Instruction {
            let mut data = GlobalSighashDB.get("create_staker").unwrap().to_vec();
            data.push(staker_account_bump);
            Instruction {
                program_id: addresses::FARM_PROGRAM_ID,
                accounts: vec![
                    AccountMeta::new_readonly(farm_key, false),
                    AccountMeta::new(staker_account, false),
                    AccountMeta::new(authority, true),
                    AccountMeta::new(authority, true),
                    AccountMeta::new_readonly(system_program::id(), false),
                    AccountMeta::new_readonly(sysvar::rent::id(), false),
                ],
                data,
            }
        }
        pub fn new_stake_ix(
            farm_account: Pubkey,
            staker_account: Pubkey,
            farm_stake_token_account: Pubkey,
            crop_account: Pubkey,
            crop_reward_token_account: Pubkey,
            harvester_account: Pubkey,
            user_reward_token_account: Pubkey,
            user_stake_account_account: Pubkey,
            authority: Pubkey,
            amount: u64,
        ) -> Instruction {
            let mut data = GlobalSighashDB.get("stake").unwrap().to_vec();
            data.extend_from_slice(&amount.to_le_bytes()[..]);
            Instruction {
                program_id: addresses::FARM_PROGRAM_ID,
                accounts: vec![
                    AccountMeta::new_readonly(farm_account, false),
                    AccountMeta::new(staker_account, false),
                    AccountMeta::new(farm_stake_token_account, false),
                    AccountMeta::new(crop_account, false),
                    AccountMeta::new(crop_reward_token_account, false),
                    AccountMeta::new(harvester_account, false),
                    AccountMeta::new(user_reward_token_account, false),
                    AccountMeta::new(user_stake_account_account, false),
                    AccountMeta::new_readonly(authority, true),
                    AccountMeta::new_readonly(spl_token::id(), false),
                    AccountMeta::new_readonly(sysvar::clock::id(), false),
                ],
                data,
            }
        }
        pub fn new_stake_dual_crop_ix(
            farm_account: Pubkey,
            farm_stake_token_account: Pubkey,
            staker_account: Pubkey,
            crop_1_crop_account: Pubkey,
            crop_1_crop_reward_token_account: Pubkey,
            crop_1_harvester_account: Pubkey,
            crop_1_user_reward_token_account: Pubkey,
            crop_2_crop_account: Pubkey,
            crop_2_crop_reward_token_account: Pubkey,
            crop_2_harvester_account: Pubkey,
            crop_2_user_reward_token_account: Pubkey,
            user_stake_token_account: Pubkey,
            authority: Pubkey,
            amount: u64,
        ) -> Instruction {
            let mut data = GlobalSighashDB.get("stake_dual_crop").unwrap().to_vec();
            data.extend_from_slice(&amount.to_le_bytes()[..]);
            Instruction {
                program_id: addresses::FARM_PROGRAM_ID,
                accounts: vec![
                    AccountMeta::new_readonly(farm_account, false),
                    AccountMeta::new(farm_stake_token_account, false),
                    AccountMeta::new(staker_account, false),
                    AccountMeta::new(crop_1_crop_account, false),
                    AccountMeta::new(crop_1_crop_reward_token_account, false),
                    AccountMeta::new(crop_1_harvester_account, false),
                    AccountMeta::new(crop_1_user_reward_token_account, false),
                    AccountMeta::new(crop_2_crop_account, false),
                    AccountMeta::new(crop_2_crop_reward_token_account, false),
                    AccountMeta::new(crop_2_harvester_account, false),
                    AccountMeta::new(crop_2_user_reward_token_account, false),
                    AccountMeta::new(user_stake_token_account, false),
                    AccountMeta::new_readonly(authority, true),
                    AccountMeta::new_readonly(spl_token::id(), false),
                    AccountMeta::new_readonly(sysvar::clock::id(), false),
                ],
                data,
            }
        }
        pub fn new_unstake_ix(
            farm_account: Pubkey,
            staker_account: Pubkey,
            farm_stake_token_account: Pubkey,
            crop_account: Pubkey,
            crop_reward_token_account: Pubkey,
            harvester_account: Pubkey,
            user_reward_token_account: Pubkey,
            user_stake_account_account: Pubkey,
            authority: Pubkey,
            amount: u64,
        ) -> Instruction {
            let mut data = GlobalSighashDB.get("unstake").unwrap().to_vec();
            data.extend_from_slice(&amount.to_le_bytes()[..]);
            Instruction {
                program_id: addresses::FARM_PROGRAM_ID,
                accounts: vec![
                    AccountMeta::new_readonly(farm_account, false),
                    AccountMeta::new(staker_account, false),
                    AccountMeta::new(farm_stake_token_account, false),
                    AccountMeta::new(crop_account, false),
                    AccountMeta::new(crop_reward_token_account, false),
                    AccountMeta::new(harvester_account, false),
                    AccountMeta::new(user_reward_token_account, false),
                    AccountMeta::new(user_stake_account_account, false),
                    AccountMeta::new_readonly(authority, true),
                    AccountMeta::new_readonly(spl_token::id(), false),
                    AccountMeta::new_readonly(sysvar::clock::id(), false),
                ],
                data,
            }
        }
        pub fn new_unstake_dual_crop_ix(
            farm_account: Pubkey,
            farm_stake_token_account: Pubkey,
            staker_account: Pubkey,
            crop_1_crop_account: Pubkey,
            crop_1_crop_reward_token_account: Pubkey,
            crop_1_harvester_account: Pubkey,
            crop_1_user_reward_token_account: Pubkey,
            crop_2_crop_account: Pubkey,
            crop_2_crop_reward_token_account: Pubkey,
            crop_2_harvester_account: Pubkey,
            crop_2_user_reward_token_account: Pubkey,
            user_stake_token_account: Pubkey,
            authority: Pubkey,
            amount: u64,
        ) -> Instruction {
            let mut data = GlobalSighashDB.get("unstake_dual_crop").unwrap().to_vec();
            data.extend_from_slice(&amount.to_le_bytes()[..]);
            Instruction {
                program_id: addresses::FARM_PROGRAM_ID,
                accounts: vec![
                    AccountMeta::new_readonly(farm_account, false),
                    AccountMeta::new(farm_stake_token_account, false),
                    AccountMeta::new(staker_account, false),
                    AccountMeta::new(crop_1_crop_account, false),
                    AccountMeta::new(crop_1_crop_reward_token_account, false),
                    AccountMeta::new(crop_1_harvester_account, false),
                    AccountMeta::new(crop_1_user_reward_token_account, false),
                    AccountMeta::new(crop_2_crop_account, false),
                    AccountMeta::new(crop_2_crop_reward_token_account, false),
                    AccountMeta::new(crop_2_harvester_account, false),
                    AccountMeta::new(crop_2_user_reward_token_account, false),
                    AccountMeta::new(user_stake_token_account, false),
                    AccountMeta::new_readonly(authority, true),
                    AccountMeta::new_readonly(spl_token::id(), false),
                    AccountMeta::new_readonly(sysvar::clock::id(), false),
                ],
                data,
            }
        }
        pub fn new_claim_ix(
            farm_account: Pubkey,
            staker_account: Pubkey,
            farm_stake_token_account: Pubkey,
            crop_account: Pubkey,
            crop_reward_token_account: Pubkey,
            harvester_account: Pubkey,
            user_reward_token_account: Pubkey,
            user_stake_token_account: Pubkey,
            authority: Pubkey,
        ) -> Instruction {
            let data = GlobalSighashDB.get("claim").unwrap().to_vec();
            Instruction {
                program_id: addresses::FARM_PROGRAM_ID,
                accounts: vec![
                    AccountMeta::new_readonly(farm_account, false),
                    AccountMeta::new(staker_account, false),
                    AccountMeta::new(farm_stake_token_account, false),
                    AccountMeta::new(crop_account, false),
                    AccountMeta::new(crop_reward_token_account, false),
                    AccountMeta::new(harvester_account, false),
                    AccountMeta::new(user_reward_token_account, false),
                    AccountMeta::new(user_stake_token_account, false),
                    AccountMeta::new(authority, true),
                    AccountMeta::new_readonly(spl_token::id(), false),
                    AccountMeta::new_readonly(sysvar::clock::id(), false),
                ],
                data,
            }
        }
        pub fn new_claim_dual_crop_ix(
            farm_account: Pubkey,
            farm_stake_token_account: Pubkey,
            staker_account: Pubkey,
            crop_1_crop_account: Pubkey,
            crop_1_crop_reward_token_account: Pubkey,
            crop_1_harvester_account: Pubkey,
            crop_1_user_reward_token_account: Pubkey,
            crop_2_crop_account: Pubkey,
            crop_2_crop_reward_token_account: Pubkey,
            crop_2_harvester_account: Pubkey,
            crop_2_user_reward_token_account: Pubkey,
            user_stake_token_account: Pubkey,
            authority: Pubkey,
        ) -> Instruction {
            let data = GlobalSighashDB.get("claim_dual_crop").unwrap().to_vec();
            Instruction {
                program_id: addresses::FARM_PROGRAM_ID,
                accounts: vec![
                    AccountMeta::new_readonly(farm_account, false),
                    AccountMeta::new(farm_stake_token_account, false),
                    AccountMeta::new(staker_account, false),
                    AccountMeta::new(crop_1_crop_account, false),
                    AccountMeta::new(crop_1_crop_reward_token_account, false),
                    AccountMeta::new(crop_1_harvester_account, false),
                    AccountMeta::new(crop_1_user_reward_token_account, false),
                    AccountMeta::new(crop_2_crop_account, false),
                    AccountMeta::new(crop_2_crop_reward_token_account, false),
                    AccountMeta::new(crop_2_harvester_account, false),
                    AccountMeta::new(crop_2_user_reward_token_account, false),
                    AccountMeta::new(user_stake_token_account, false),
                    AccountMeta::new_readonly(authority, true),
                    AccountMeta::new_readonly(spl_token::id(), false),
                    AccountMeta::new_readonly(sysvar::clock::id(), false),
                ],
                data,
            }
        }

        pub fn new_create_harvester_ix(
            crop_account: Pubkey,
            harvester_account: Pubkey,
            authority: Pubkey,
            payer: Pubkey,
            harvester_bump: u8,
        ) -> Instruction {
            let mut data = GlobalSighashDB.get("create_harvester").unwrap().to_vec();
            data.push(harvester_bump);
            Instruction {
                program_id: addresses::FARM_PROGRAM_ID,
                accounts: vec![
                    AccountMeta::new_readonly(crop_account, false),
                    AccountMeta::new(harvester_account, false),
                    AccountMeta::new(authority, true),
                    AccountMeta::new(payer, true),
                    AccountMeta::new_readonly(system_program::id(), false),
                    AccountMeta::new_readonly(sysvar::rent::id(), false),
                ],
                data,
            }
        }
    }
    pub mod pool {
        use crate::addresses::SERUM_DEX_PROGRAM_ID;

        use super::*;

        pub fn new_deposit_ix(
            protocol_account: Pubkey,
            pool_account: Pubkey,
            pool_coin_token_account: Pubkey,
            pool_pc_token_account: Pubkey,
            pool_lp_mint: Pubkey,
            user_coin_token_account: Pubkey,
            user_pc_token_account: Pubkey,
            user_lp_token_account: Pubkey,
            user_authority: Pubkey,
            market: Pubkey,
            open_orders: Pubkey,
            request_queue: Pubkey,
            event_queue: Pubkey,
            bids: Pubkey,
            asks: Pubkey,
            serum_coin_vault: Pubkey,
            serum_pc_vault: Pubkey,
            serum_vault_signer: Pubkey,
            desired_coin_amount: u64,
            desired_pc_amount: u64,
            min_coin_amount: u64,
            min_pc_amount: u64,
        ) -> Instruction {
            let mut data = Vec::with_capacity(8 + (8 * 4));
            data.extend_from_slice(&GlobalSighashDB.get("deposit").unwrap()[..]);
            data.extend_from_slice(&desired_coin_amount.to_le_bytes()[..]);
            data.extend_from_slice(&desired_pc_amount.to_le_bytes()[..]);
            data.extend_from_slice(&min_coin_amount.to_le_bytes()[..]);
            data.extend_from_slice(&min_pc_amount.to_le_bytes()[..]);
            Instruction {
                program_id: addresses::POOL_PROGRAM_ID,
                accounts: vec![
                    AccountMeta::new_readonly(protocol_account, false),
                    AccountMeta::new(pool_account, false),
                    AccountMeta::new(pool_coin_token_account, false),
                    AccountMeta::new(pool_pc_token_account, false),
                    AccountMeta::new(pool_lp_mint, false),
                    AccountMeta::new(user_coin_token_account, false),
                    AccountMeta::new(user_pc_token_account, false),
                    AccountMeta::new(user_lp_token_account, false),
                    AccountMeta::new_readonly(user_authority, true),
                    AccountMeta::new(market, false),
                    AccountMeta::new(open_orders, false),
                    AccountMeta::new(request_queue, false),
                    AccountMeta::new(event_queue, false),
                    AccountMeta::new(bids, false),
                    AccountMeta::new(asks, false),
                    AccountMeta::new(serum_coin_vault, false),
                    AccountMeta::new(serum_pc_vault, false),
                    AccountMeta::new_readonly(serum_vault_signer, false),
                    AccountMeta::new_readonly(spl_token::id(), false),
                    AccountMeta::new_readonly(SERUM_DEX_PROGRAM_ID, false),
                    AccountMeta::new_readonly(sysvar::rent::id(), false),
                ],
                data,
            }
        }
        pub fn new_withdraw_ix(
            protocol_account: Pubkey,
            pool_account: Pubkey,
            pool_coin_token_account: Pubkey,
            pool_pc_token_account: Pubkey,
            pool_lp_mint :Pubkey,
            user_coin_token_account: Pubkey,
            user_pc_token_account: Pubkey,
            user_lp_token_account: Pubkey,
            user_authority: Pubkey,
            market: Pubkey,
            open_orders: Pubkey,
            request_queue: Pubkey,
            event_queue: Pubkey,
            bids: Pubkey,
            asks: Pubkey,
            serum_coin_vault: Pubkey,
            serum_pc_vault: Pubkey,
            serum_vault_signer: Pubkey,
            withdraw_lp_amount: u64,
        ) -> Instruction {
            let mut data = Vec::with_capacity(16);
            data.extend_from_slice(&GlobalSighashDB.get("withdraw").unwrap()[..]);
            data.extend_from_slice(&withdraw_lp_amount.to_le_bytes()[..]);
            Instruction {
                program_id: addresses::POOL_PROGRAM_ID,
                accounts: vec![
                    AccountMeta::new_readonly(protocol_account, false),
                    AccountMeta::new(pool_account, false),
                    AccountMeta::new(pool_coin_token_account, false),
                    AccountMeta::new(pool_pc_token_account, false),
                    AccountMeta::new(pool_lp_mint, false),
                    AccountMeta::new(user_coin_token_account, false),
                    AccountMeta::new(user_pc_token_account, false),
                    AccountMeta::new(user_lp_token_account, false),
                    AccountMeta::new_readonly(user_authority, true),
                    AccountMeta::new(market, false),
                    AccountMeta::new(open_orders, false),
                    AccountMeta::new(request_queue, false),
                    AccountMeta::new(event_queue, false),
                    AccountMeta::new(bids, false),
                    AccountMeta::new(asks, false),
                    AccountMeta::new(serum_coin_vault, false),
                    AccountMeta::new(serum_pc_vault, false),
                    AccountMeta::new_readonly(serum_vault_signer, false),
                    AccountMeta::new_readonly(spl_token::id(), false),
                    AccountMeta::new_readonly(SERUM_DEX_PROGRAM_ID, false),
                    AccountMeta::new_readonly(sysvar::rent::id(), false),
                ],
                data,
            }
        }
    }
}
