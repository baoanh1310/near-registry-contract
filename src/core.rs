use crate::*;

#[near_bindgen]
impl Contract {
    pub fn create_entry(&mut self, title: String, description: String, url: String) {
        let entry_id = self.entries.len() as i32;
        let creator = env::predecessor_account_id();

        self.entries.push(Entry::new(
            entry_id,
            creator,
            title,
            description,
            url
        ));

        env::log("Added a new entry".as_bytes());
    }

}