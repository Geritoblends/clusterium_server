use serde::{Serialize, Deserialize};
use mongodb::bson::{Oid::ObjectId, DateTime};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

pub struct AccountId {
    id: [u8; 16]
}

impl AccountId {

    pub fn as_ascii(&self) -> String {
        self.id
            .iter()
            .filter(|&&b| b.is_ascii())
            .map(|&b| b as char)
            .collect()
    }

    pub fn as_bytes(&self) -> [u8; 16] {
        self.id
    }

}

pub struct OwnershipId {
    id: [u8; 12]
}

impl OwnershipId {

    pub fn as_base64(&self) -> String {
        BASE64.encode(&self.id)
    }

    pub fn as_bytes(&self -> [u8; 12] {
        self.id
    }

}

pub struct Item {
    id: OwnershipId,
    account_id: AccountId,
    balance: i32,
    version: i32
} 
