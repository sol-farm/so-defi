//! api binding type definitions

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use solana_program::pubkey::Pubkey;
use std::{collections::HashMap, str::FromStr};

pub const BASE_URL: &str = "https://hyper.solana.fm";
pub const V3_TRANSFER_STUB: &str = "v3/transfers";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    PayTxFees,
    /// the first
    Transfer {
        token_mint: Pubkey,
        amount: u64,
    },
}

impl TryFrom<&_TransferEntry> for Action {
    type Error = anyhow::Error;
    fn try_from(entry: &_TransferEntry) -> Result<Self, Self::Error> {
        if entry.action.eq_ignore_ascii_case("pay_tx_fees") {
            Ok(Self::PayTxFees)
        } else {
            let token_mint = Pubkey::from_str(&entry.token)?;
            Ok(Self::Transfer {
                token_mint,
                amount: entry.amount.try_into()?,
            })
        }
    }
}

///v3/transfers/:tx?friendly=:FriendlyTag"
#[derive(Default, Debug, Clone, Copy)]
pub struct TransferEntry {
    pub action: Action,
    pub source: Pubkey,
    /// only present on token transfers
    pub source_association: Option<Pubkey>,
    /// only present on token transfers
    pub destination: Option<Pubkey>,
    /// only present on token transfers
    pub destination_association: Option<Pubkey>,
}

///v3/transfers/:tx?friendly=:FriendlyTag"
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct _TransferEntry {
    pub action: String,
    pub status: String,
    pub source: String,
    #[serde(rename = "source_association")]
    pub source_association: String,
    pub destination: String,
    #[serde(rename = "destination_association")]
    pub destination_association: String,
    pub token: String,
    pub amount: i64,
    pub timestamp: i64,
}

pub struct TokenTransferEntries(pub Vec<TransferEntry>);

impl From<Vec<TransferEntry>> for TokenTransferEntries {
    fn from(entry: Vec<TransferEntry>) -> Self {
        Self { 0: entry }
    }
}

impl TokenTransferEntries {
    pub fn filter_by_destination(&self, want: Pubkey) -> Vec<TransferEntry> {
        self.0
            .iter()
            .filter_map(|entry| {
                if let Some(destination) = entry.destination.as_ref() {
                    if want.eq(destination) {
                        return Some(entry.clone());
                    } else {
                        return None;
                    }
                }
                None
            })
            .collect()
    }
    pub fn filter_by_destination_association(&self, want: Pubkey) -> Vec<TransferEntry> {
        self.0
            .iter()
            .filter_map(|entry| {
                if let Some(destination_association) = entry.destination_association.as_ref() {
                    if want.eq(destination_association) {
                        return Some(entry.clone());
                    } else {
                        return None;
                    }
                }
                None
            })
            .collect()
    }
}

impl Default for Action {
    fn default() -> Self {
        Self::PayTxFees
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_urls() {
        assert_eq!(BASE_URL, "https://hyper.solana.fm");
        assert_eq!(V3_TRANSFER_STUB, "v3/transfers");
    }
}
