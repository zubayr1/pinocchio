use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct AccountInfo {
    pub name: String,
    pub age: u8,
    pub email: String,
}

impl AccountInfo {
    pub fn new(name: String, age: u8, email: String) -> Self {
        AccountInfo { name, age, email }
    }
}
