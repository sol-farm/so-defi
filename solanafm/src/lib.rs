//! basic api bindings for solscan
pub mod types;

use std::str::FromStr;

use solana_program::pubkey::Pubkey;
pub use types::*;

use anyhow::{anyhow, Result};

#[derive(Clone)]
pub struct Client {
    c: reqwest::blocking::Client,
    version: API,
}

#[derive(Clone, Copy)]
pub enum API {
    V3,
}

impl API {
    pub fn get_transaction_url(&self, tx_hash: &str, friendly: bool) -> String {
        match self {
            API::V3 => {
                // https://hyper.solana.fm
                // /
                // v3/transfers
                // /4Nc7GGX1....
                // ?friendly=true
                format!(
                    "{}/{}/{}?friendly={}",
                    BASE_URL, V3_TRANSFER_STUB, tx_hash, friendly
                )
            }
        }
    }
}

/*
curl -X 'GET' \
  'https://public-api.solscan.io/transaction/4Nc7GGX139JFvt4TkccxzMtBRNDU4y4xAzExPcVncPBq1fMmgNW9xwHq5avLYDkT1r1zLbW2o1GFWmmn2UTwce9' \
  -H 'accept: application/json'
*/

impl Client {
    pub fn new() -> Self {
        Default::default()
    }
    /// returns all transfers within the given transaction
    pub fn get_transfers(
        &self,
        tx_hash: &str,
        friendly: bool,
        filter_tx_fees: bool,
    ) -> Result<Vec<TransferEntry>> {
        let client = self
            .c
            .get(self.version.get_transaction_url(tx_hash, friendly))
            .header("accept", "application/json");
        let response = client.send()?;

        let mut response: serde_json::Value = serde_json::from_str(&response.text()?)?;
        match response.get_mut("Transfers") {
            Some(i) => match i.get_mut(tx_hash) {
                Some(i) => {
                    if let Some(i_array) = i.as_array() {
                        let transfer_entries = i_array
                            .into_iter()
                            .filter_map(|i| {
                                match serde_json::from_value::<_TransferEntry>(i.clone()) {
                                    Ok(entry) => Some(entry),
                                    Err(err) => {
                                        println!("failed to deserialize {:#?}", err);
                                        None
                                    }
                                }
                            })
                            .filter_map(|transfer| {
                                let action: Action =
                                    if let Ok(action) = TryFrom::try_from(&transfer) {
                                        action
                                    } else {
                                        return None;
                                    };

                                let source = if let Ok(key) = Pubkey::from_str(&transfer.source) {
                                    key
                                } else {
                                    return None;
                                };
                                if action.eq(&Action::PayTxFees) && filter_tx_fees {
                                    return None;
                                } else if action.eq(&Action::PayTxFees) && !filter_tx_fees {
                                    return Some(TransferEntry {
                                        action,
                                        source,
                                        ..Default::default()
                                    });
                                } else {
                                    Some(TransferEntry {
                                        action,
                                        source,
                                        source_association: if let Ok(key) =
                                            Pubkey::from_str(&transfer.source_association)
                                        {
                                            Some(key)
                                        } else {
                                            return None;
                                        },
                                        destination: if let Ok(key) =
                                            Pubkey::from_str(&transfer.source)
                                        {
                                            Some(key)
                                        } else {
                                            return None;
                                        },
                                        destination_association: if let Ok(key) =
                                            Pubkey::from_str(&transfer.source)
                                        {
                                            Some(key)
                                        } else {
                                            return None;
                                        },
                                    })
                                }
                            })
                            .collect::<Vec<_>>();
                        if transfer_entries.is_empty() {
                            return Err(anyhow!("found no transfer entries"));
                        }

                        return Ok(transfer_entries);
                    } else {
                        return Err(anyhow!("failed to get transfers array"));
                    }
                }
                None => return Err(anyhow!("failed to get transfers by tx hash")),
            },
            None => return Err(anyhow!("failed to get Transfer key")),
        }
    }
}

impl Default for Client {
    fn default() -> Self {
        Self {
            c: reqwest::blocking::Client::new(),
            version: API::V3,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_client() {
        let client = Client::default();
        let transfers = client.get_transfers("4Nc7GGX139JFvt4TkccxzMtBRNDU4y4xAzExPcVncPBq1fMmgNW9xwHq5avLYDkT1r1zLbW2o1GFWmmn2UTwce9", true, false).unwrap();
        assert_eq!(transfers.len(), 8);
        let transfers = client.get_transfers("4Nc7GGX139JFvt4TkccxzMtBRNDU4y4xAzExPcVncPBq1fMmgNW9xwHq5avLYDkT1r1zLbW2o1GFWmmn2UTwce9", true, true).unwrap();
        assert_eq!(transfers.len(), 7);

        let token_transfers = TokenTransferEntries::from(transfers);

        let fee_tokens = token_transfers.filter_by_destination(
            Pubkey::from_str("HgX4fugaghRMRNtyBdBygsWSFoREphwiSSTaDQSYns7Z").unwrap(),
        );
        assert_eq!(fee_tokens.len(), 3);
    }
    #[test]
    fn test_api() {
        let api = API::V3;
        let url = api.get_transaction_url("4Nc7GGX139JFvt4TkccxzMtBRNDU4y4xAzExPcVncPBq1fMmgNW9xwHq5avLYDkT1r1zLbW2o1GFWmmn2UTwce9", true);
        assert_eq!(url, "https://hyper.solana.fm/v3/transfers/4Nc7GGX139JFvt4TkccxzMtBRNDU4y4xAzExPcVncPBq1fMmgNW9xwHq5avLYDkT1r1zLbW2o1GFWmmn2UTwce9?friendly=true");
        let url = api.get_transaction_url("4Nc7GGX139JFvt4TkccxzMtBRNDU4y4xAzExPcVncPBq1fMmgNW9xwHq5avLYDkT1r1zLbW2o1GFWmmn2UTwce9", false);
        assert_eq!(url, "https://hyper.solana.fm/v3/transfers/4Nc7GGX139JFvt4TkccxzMtBRNDU4y4xAzExPcVncPBq1fMmgNW9xwHq5avLYDkT1r1zLbW2o1GFWmmn2UTwce9?friendly=false");
    }
}
