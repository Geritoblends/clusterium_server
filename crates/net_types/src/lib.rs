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

    pub fn as_bytes(&self) -> &[u8; 16] {
        &self.id
    }

}

pub struct OwnershipId {
    id: [u8; 12]
}

impl OwnershipId {

    pub fn new(bytes: &[u8; 12]) -> Self {
        Self {
            id: *bytes
        }
    }

    pub fn as_base64(&self) -> String {
        BASE64.encode(&self.id)
    }

    pub fn as_bytes(&self) -> &[u8; 12] {
        &self.id
    }

}

pub struct Item {
    id: OwnershipId,
    account_id: AccountId,
    balance: i32,
    version: i32
} 

impl Item {

    pub fn new(id: OwnershipId, account_id: AccountId, balance: i32, version: i32) -> Self {
        Self {
            id,
            account_id,
            balance,
            version
        }
    }

    pub fn get_id(&self) -> OwnershipId {
        self.id
    }

    pub fn get_account_id(&self) -> AccountId {
        self.account_id
    }

    pub fn get_balance(&self) -> i32 {
        self.balance
    }

    pub fn get_version(&self) -> i32 {
        self.version
    }

}

pub struct Buff {
    id: OwnershipId,
    account_id: AccountId,
    expires_at: i64,
}

impl Buff {

    pub fn new(id: OwnershipId, account_id: AccountId, expires_at: i64) -> Self {
        Self {
            id,
            account_id,
            expires_at
        }
    }

    pub fn get_id(&self) -> OwnershipId {
        self.id
    }

    pub fn get_account_id(&self) -> AccountId {
        self.account_id
    }

    pub fn get_expires_at(&self) -> i64 {
        self.expires_at
    }

}
