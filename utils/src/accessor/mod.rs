//! utilities for accessor style functions

use solana_program::account_info::{Account, AccountInfo};
use solana_program::pubkey::Pubkey;

pub trait Accessor<T: Account> {
    fn access(self, accessor_type: AccessorType) -> Vec<u8>;
}

/// defines data types which can be used for accessing values
/// the value contained within the enum is the offset at which
/// the data type starts
pub enum AccessorType {
    Bool(usize),
    U64(usize),
    U128(usize),
    Pubkey(usize),
    U8(usize),
    U32(usize),
}

#[cfg(feature = "experimental")]
impl<'a, T: Account> Accessor<T> for &mut T {
    fn access(self, accessor_type: AccessorType) -> Vec<u8> {
        let (_, data, _, _, _) = self.get();
        let (output_size, offset) = match accessor_type {
            AccessorType::Bool(offset) => (accessor_type.data_size(), offset),
            AccessorType::U64(offset) => (accessor_type.data_size(), offset),
            AccessorType::U128(offset) => (accessor_type.data_size(), offset),
            AccessorType::Pubkey(offset) => (accessor_type.data_size(), offset),
            AccessorType::U8(offset) => (accessor_type.data_size(), offset),
            AccessorType::U32(offset) => (accessor_type.data_size(), offset),
        };
        // initialize the output vector with a given capacity
        let mut output_bytes = vec![0_u8; output_size];
        solana_program::program_memory::sol_memcpy(
            &mut output_bytes,
            &data[offset..output_size + offset],
            output_size,
        );
        output_bytes
    }
}

impl AccessorType {
    pub fn access(&self, account: &AccountInfo) -> Vec<u8> {
        let (output_size, offset) = match self {
            AccessorType::Bool(offset) => (self.data_size(), *offset),
            AccessorType::U64(offset) => (self.data_size(), *offset),
            AccessorType::U128(offset) => (self.data_size(), *offset),
            AccessorType::Pubkey(offset) => (self.data_size(), *offset),
            AccessorType::U8(offset) => (self.data_size(), *offset),
            AccessorType::U32(offset) => (self.data_size(), *offset),
        };
        let bytes = account.data.borrow();
        // initialize the output vector with a given capacity
        let mut output_bytes = vec![0_u8; output_size];
        solana_program::program_memory::sol_memcpy(
            &mut output_bytes,
            &bytes[offset..output_size + offset],
            output_size,
        );
        output_bytes
    }
    pub fn data_size(&self) -> usize {
        match *self {
            AccessorType::Bool(_) => 1,
            AccessorType::U64(_) => 8,
            AccessorType::U128(_) => 16,
            AccessorType::Pubkey(_) => 32,
            AccessorType::U8(_) => 1,
            AccessorType::U32(_) => 4,
        }
    }
}

pub fn to_u64(bytes: &Vec<u8>) -> u64 {
    let mut amount: [u8; 8] = [0_u8; 8];
    amount.copy_from_slice(&bytes[..]);
    u64::from_le_bytes(amount)
}

pub fn to_pubkey(bytes: &Vec<u8>) -> Pubkey {
    let mut key: [u8; 32] = [0_u8; 32];
    key.copy_from_slice(&bytes[..]);
    Pubkey::new_from_array(key)
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::*;
    use solana_client::rpc_client::RpcClient;
    use solana_program::account_info::IntoAccountInfo;
    #[test]
    fn test_accessor_trait() {
        let test_srm_acct =
            Pubkey::from_str("Hu43u5GxMfSwjPsYF6STMDbuu91mUJKZXfFm4roQnbA2").unwrap();

        let rpc = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
        let mut srm_acct = rpc.get_account(&test_srm_acct).unwrap();
        let amount = srm_acct.access(AccessorType::U64(64));
        let amount = to_u64(&amount);
        assert_eq!(amount, 108547950373);
        let value = srm_acct.access(AccessorType::U8(64));
        assert_eq!(value.len(), 1);
        assert_eq!(value[0], 37);
    }
    #[test]
    fn test_accessor() {
        let test_srm_acct =
            Pubkey::from_str("A49TahKG1c6KsqNhCtjkEYT1TV65LXqq4eGMJUNYXxDQ").unwrap();

        let rpc = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
        let srm_acct = rpc.get_account(&test_srm_acct).unwrap();
        let mut srm_tuple = (test_srm_acct, srm_acct);
        let srm_info = srm_tuple.into_account_info();

        let amount = AccessorType::U64(64).access(&srm_info);

        let amount = to_u64(&amount);
        assert_eq!(amount, 1000000);

        let mint = AccessorType::Pubkey(0).access(&srm_info);
        let mint = to_pubkey(&mint);
        assert_eq!(
            mint.to_string(),
            "SRMuApVNdxXokk5GT7XD5cUUgXMBCoAz2LHeuAoKWRt".to_string()
        );

        let authority = AccessorType::Pubkey(32).access(&srm_info);
        let authority = to_pubkey(&authority);
        assert_eq!(
            authority.to_string(),
            "3B1TL3u5Z6sVzgXtgjsoQ2dY85YFXs6fNA8yCpHfyzDQ".to_string()
        );
    }
}
