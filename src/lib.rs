use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use near_sdk::{env, near_bindgen, setup_alloc, Promise, PromiseResult, AccountId, Timestamp, Balance};
use near_sdk::serde::{Deserialize, Serialize};

pub use crate::models::*;
pub use crate::core::*;
pub use crate::utils::*;

mod models;
mod core;
mod utils;

setup_alloc!();

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub owner_id: AccountId,
    pub entries: Vec<Entry>
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id: owner_id,
            entries: Vec::new()
        }
    }
}

// View methods
#[near_bindgen]
impl Contract {
    pub fn get_number_entries(&self) -> usize {
        self.entries.len()
    }

    pub fn get_entry_total_votes(&self, entry_id: usize) -> u128 {
        let entry = self.entries.get(entry_id).unwrap();
        return entry.total_votes;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use near_sdk::test_utils::{VMContextBuilder, accounts};
    use near_sdk::{testing_env};
    use near_sdk::MockedBlockchain;

    fn get_context(is_view: bool) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.
        current_account_id(accounts(0))
        .signer_account_id(accounts(0))
        .predecessor_account_id(accounts(0))
        .is_view(is_view);

        builder
    }

    #[test]
    fn test_create_entry() {
        let mut context = get_context(false);
        testing_env!(context.build());
        
        // Init contract
        let mut contract = Contract::new(accounts(0).to_string());

        testing_env!(
            context.storage_usage(env::storage_usage())
            .predecessor_account_id(accounts(0))
            .build()
        );

        contract.create_entry("TITLE".to_string(), "PROJECT DESCRIPTION".to_string(), "https://near.org/".to_string());

        assert_eq!(contract.get_number_entries(), 1);
        assert_eq!(contract.get_entry_total_votes(0), 0);
    }


    #[test]
    fn test_upvote() {
        let mut context = get_context(false);
        testing_env!(context.build());
        
        // Init contract
        let mut contract = Contract::new(accounts(0).to_string());

        testing_env!(
            context.storage_usage(env::storage_usage())
            .predecessor_account_id(accounts(0))
            .build()
        );

        contract.create_entry("TITLE".to_string(), "PROJECT DESCRIPTION".to_string(), "https://near.org/".to_string());

        
    }
}
