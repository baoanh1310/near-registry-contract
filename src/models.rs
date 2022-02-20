use crate::*;

#[derive(Clone, Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Entry {
    pub id: i32,
    pub creator: AccountId,
    pub title: String,
    pub description: String,
    pub url: String,
    pub total_votes: u128
}

impl Entry {
    pub fn new(id: i32, creator: AccountId, title: String, description: String, url: String) -> Self {
        Self {
            id: id,
            creator: creator,
            title: title,
            description: description,
            url: url,
            total_votes: 0
        }
    }
}