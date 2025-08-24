use serde::{Serialize, Deserialize};
use crate::ownership_id::OwnershipId;
use mongodb::bson::oid::ObjectId;

#[derive(Serialize, Deserialize)]
pub struct Item {
    id: OwnershipId,
    account_id: ObjectId,
    balance: i32,
    version: i32
} 

impl Item {

    pub fn new(id: OwnershipId, account_id: ObjectId, balance: i32, version: i32) -> Self {
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

    pub fn get_account_id(&self) -> ObjectId {
        self.account_id
    }

    pub fn get_balance(&self) -> i32 {
        self.balance
    }

    pub fn get_version(&self) -> i32 {
        self.version
    }

}

