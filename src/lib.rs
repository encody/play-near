use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    near_bindgen, PanicOnDefault, store::Vector, json_types::Base64VecU8,
};

fn fake_near_bindgen() {
    let key = b"STATE";
    let managed_state_serialized = near_sdk::env::storage_read(key).expect("Contract is not yet initialized");
    let mut managed_state = Contract::try_from_slice(&managed_state_serialized).expect("Failed to deserialize contract state from storage");
    managed_state.add_message(/* whatever the transaction passed as `message` */ String::from("hi"));
    near_sdk::env::storage_write(key, &managed_state.try_to_vec().unwrap());
}

#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
#[near_bindgen]
struct Contract {
    messages: Vector<String>,
}

#[near_bindgen]
impl Contract {
    #[init(ignore_state)]
    pub fn new() -> Self {
        Self { messages: Vector::new(b"m") }
    }

    pub fn add_message(&mut self, message: String) {
        self.messages.push(message);
    }

    pub fn get_messages(&self) -> Vec<&String> {
        self.messages.iter().collect()
    }

    pub fn clear_arbitrary_key(&self, key: Base64VecU8) {
        near_sdk::env::storage_remove(&key.0);
    }
}
