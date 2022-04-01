use solana_program;
use solana_program::pubkey::Pubkey;
use static_pubkey::static_pubkey;

pub mod prelude {
    pub use super::addresses;
    pub use super::instructions;
    pub use solana_program;
    pub use static_pubkey::static_pubkey;
}


pub mod addresses {
    use super::*;

    /// atrix farm program address
    pub const PROGRAM_ID: Pubkey = static_pubkey!("BLDDrex4ZSWBgPYaaH6CQCzkJXWfzCiiur9cSFJT8t3x");
    pub const FARM_SEED: &[u8; 10] = b"atrix-farm";
    pub const CROP_SEED: &[u8; 15] = b"atrix-farm-crop";
    pub const FARM_STAKE_SEED: &[u8; 16] = b"atrix-farm-stake";
    pub const FARM_HARVESTER_SEED: &[u8; 20] = b"atrix-farm-harvester";

    pub fn find_farm_address(
        base: Pubkey,
    ) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                FARM_SEED,
                base.as_ref(),
            ],
            &PROGRAM_ID,
        )
    }

    pub fn find_crop_address(
        farm_key: Pubkey,
        reward_mint: Pubkey,
    ) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                CROP_SEED, 
                farm_key.as_ref(), 
                reward_mint.as_ref(),
                ], &PROGRAM_ID,
        )
    }

    pub fn find_staker_address(
        farm_key: Pubkey,
        authority: Pubkey,
    ) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                FARM_STAKE_SEED,
                authority.as_ref(),
                farm_key.as_ref(),
            ], &PROGRAM_ID,
        )
    }

    pub fn find_harvester_address(
        crop_key: Pubkey,
        authority: Pubkey,
    ) -> (Pubkey, u8) {
        Pubkey::find_program_address(
            &[
                FARM_HARVESTER_SEED,
                authority.as_ref(),
                crop_key.as_ref(),
            ],
            &PROGRAM_ID,
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
            assert_eq!(got_farm.to_string(), "J55atXt8BnF99YUC4AmpHY2VuxZ6XbBTjL7dHaePid42".to_string());
            assert_eq!(got_nonce, 255);

            let reward_mint = static_pubkey!("MNDEFzGvMt87ueuHvVU9VcTqsAP5b3fTGPsHuuPA5ey");

            let (got_crop, got_crop_nonce) = find_crop_address(
                got_farm, reward_mint,
            );
            assert_eq!(got_crop.to_string(), "GcAYkGrZx97u3wUVkjz4z74M2NZhBq3V7bWXmyadvdiC".to_string());
            assert_eq!(got_crop_nonce, 255);

            let crop_auth = static_pubkey!("AufL1ZuuAZoX7jBw8kECvjUYjfhWqZm13hbXeqnLMhFu");

            let (got_harvester, got_harvester_nonce) = find_harvester_address(
                got_crop, crop_auth,
            );
            assert_eq!(got_harvester.to_string(), "DxHDqv8fABj56GHMa2PaSuou2NGWe2txXjTmjuC8o45J".to_string());
            assert_eq!(got_harvester_nonce, 255);

            let (got_staker, got_staker_nonce) = find_staker_address(
                got_farm, system_program::id(),
            );
            assert_eq!(got_staker.to_string(), "G3sab7XGM2WcBQzdgV6SMv64QVuTHAJsW2uvnuDRxikK".to_string());
            assert_eq!(got_staker_nonce, 254);
        }
    }
}

pub mod instructions {
    use super::*;
    use addresses;
    use solana_program::{instruction::{Instruction, AccountMeta}, system_program, sysvar};

    /// instruction sighash used by the create_staker instruction
    pub const CREATE_STAKER_SIGHASH: [u8; 8] = [14, 28, 165, 74, 243, 144, 108, 177];
    /// instruction sighash used by the stake instruction
    pub const STAKE_SIGHASH: [u8; 8] = [206, 176, 202, 18, 200, 209, 179, 108];
    /// instruction sighash used by the stake_dual_crop instruction
    pub const STAKE_DUAL_CROP_SIGHASH: [u8; 8] = [241, 42, 177, 56, 14, 203, 117, 253];
    /// instruction sighash used by the unstake instruction
    pub const UNSTAKE_SIGHASH: [u8; 8] = [90, 95, 107, 42, 205, 124, 50, 225];
    /// instruction sighash used by the unstake_dual_crop instruction
    pub const UNSTAKE_DUAL_CROP_SIGHASH: [u8; 8] = [125, 31, 2, 239, 223, 165, 240, 249];
    /// instruction sighash used by the claim instruction
    pub const CLAIM_SIGHASH: [u8; 8] = [62, 198, 214, 193, 213, 159, 108, 210];
    /// instruction sighash used by the claim_dual_crop instruction
    pub const CLAIM_DUAL_CROP_SIGHASH: [u8; 8] = [128, 32, 146, 208, 138, 252, 110, 71];
    pub fn new_create_staker_account_ix(
        farm_key: Pubkey,
        authority: Pubkey,
        staker_account: Pubkey,
        staker_account_bump: u8,
    ) -> Instruction {
        let mut data = CREATE_STAKER_SIGHASH.to_vec();
        data.push(staker_account_bump);
        Instruction {
            program_id: addresses::PROGRAM_ID,
            accounts: vec![
                AccountMeta::new_readonly(farm_key, false),
                AccountMeta::new(staker_account, false),
                AccountMeta::new_readonly(authority, false),
                AccountMeta::new_readonly(authority, false),
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
        let mut data = STAKE_SIGHASH.to_vec();
        data.extend_from_slice(&amount.to_le_bytes()[..]);
        Instruction {
            program_id: addresses::PROGRAM_ID,
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
        let mut data = STAKE_DUAL_CROP_SIGHASH.to_vec();
        data.extend_from_slice(&amount.to_le_bytes()[..]);
        Instruction {
            program_id: addresses::PROGRAM_ID,
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
        let mut data = UNSTAKE_SIGHASH.to_vec();
        data.extend_from_slice(&amount.to_le_bytes()[..]);
        Instruction {
            program_id: addresses::PROGRAM_ID,
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
        let mut data = UNSTAKE_DUAL_CROP_SIGHASH.to_vec();
        data.extend_from_slice(&amount.to_le_bytes()[..]);
        Instruction {
            program_id: addresses::PROGRAM_ID,
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
        let data = CLAIM_SIGHASH.to_vec();
        Instruction {
            program_id: addresses::PROGRAM_ID,
            accounts: vec![
                AccountMeta::new_readonly(farm_account, false),
                AccountMeta::new(staker_account, false),
                AccountMeta::new(farm_stake_token_account, false),
                AccountMeta::new(crop_account, false),
                AccountMeta::new(crop_reward_token_account, false),
                AccountMeta::new(harvester_account, false),
                AccountMeta::new(user_reward_token_account, false),
                AccountMeta::new(user_stake_token_account, false),
                AccountMeta::new_readonly(authority, false),
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
        let data = CLAIM_DUAL_CROP_SIGHASH.to_vec();
        Instruction {
            program_id: addresses::PROGRAM_ID,
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
    #[cfg(test)]
    mod test {
        use super::*;
        use ring::digest::{Context, SHA256};
        #[test]
        fn test_sighashes() {
            {
                let mut context = Context::new(&SHA256);
                context.update(b"global:create_staker");
                let digest = context.finish();
                println!(
                    "pub const CREATE_STAKER_SIGHASH: [u8; 8] = {:?};",
                    &digest.as_ref()[0..8]
                );
            }
            {
                let mut context = Context::new(&SHA256);
                context.update(b"global:stake");
                let digest = context.finish();
                println!(
                    "pub const STAKE_SIGHASH: [u8; 8] = {:?};",
                    &digest.as_ref()[0..8]
                );
            }
            {
                let mut context = Context::new(&SHA256);
                context.update(b"global:stake_dual_crop");
                let digest = context.finish();
                println!(
                    "pub const STAKE_DUAL_CROP_SIGHASH: [u8; 8] = {:?};",
                    &digest.as_ref()[0..8]
                );
            }
            {
                let mut context = Context::new(&SHA256);
                context.update(b"global:unstake");
                let digest = context.finish();
                println!(
                    "pub const UNSTAKE_SIGHASH: [u8; 8] = {:?};",
                    &digest.as_ref()[0..8]
                );
            }
            {
                let mut context = Context::new(&SHA256);
                context.update(b"global:unstake_dual_crop");
                let digest = context.finish();
                println!(
                    "pub const UNSTAKE_DUAL_CROP_SIGHASH: [u8; 8] = {:?};",
                    &digest.as_ref()[0..8]
                );
            }
            {
                let mut context = Context::new(&SHA256);
                context.update(b"global:claim");
                let digest = context.finish();
                println!(
                    "pub const CLAIM_SIGHASH: [u8; 8] = {:?};",
                    &digest.as_ref()[0..8]
                );
            }
            {
                let mut context = Context::new(&SHA256);
                context.update(b"global:claim_dual_crop");
                let digest = context.finish();
                println!(
                    "pub const CLAIM_DUAL_CROP_SIGHASH: [u8; 8] = {:?};",
                    &digest.as_ref()[0..8]
                );
            }
        }
    }
}