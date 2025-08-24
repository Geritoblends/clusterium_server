use serde::{Serialize, Deserialize};
use crate::ownership_id::OwnershipId;
use mongodb::bson::oid::ObjectId;

#[derive(Serialize, Deserialize)]
pub struct Buff {
    id: OwnershipId,
    account_id: ObjectId,
    expires_at: i32,
    version: i32,
}

impl Buff {

    pub fn new(id: OwnershipId, account_id: ObjectId, expires_at: i32, version: i32) -> Self {
        Self {
            id,
            account_id,
            expires_at,
            version
        }
    }

    pub fn get_id(&self) -> OwnershipId {
        self.id
    }

    pub fn get_account_id(&self) -> ObjectId {
        self.account_id
    }

    pub fn get_expires_at(&self) -> i32 {
        self.expires_at
    }

}
