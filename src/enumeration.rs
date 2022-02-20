use crate::*;

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