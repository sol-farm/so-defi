use bytemuck::cast_slice;
use solana_program::pubkey::Pubkey;
use std::convert::identity;

/// returns a public key from a byte slice as
/// is used to store public keys in the serum market state account
pub fn pubkey_from_serum_slice(input: [u64; 4]) -> Pubkey {
    Pubkey::new(cast_slice(&identity(input) as &[_]))
}
