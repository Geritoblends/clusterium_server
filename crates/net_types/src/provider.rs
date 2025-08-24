use serde::{Serialize, Deserialize};
use crate::ownership_id::OwnershipId;

#[derive(Serialize, Deserialize)]
pub struct Provider {
    id: OwnershipId,
    account_id: ObjectId,
    timestamps: Vec<i32>,
    version: i32,
}

impl Provider {

    pub fn new(id: OwnershipId, account_id: ObjectId, timestamps: &[i32]) -> Self {
        Self {
            id,
            account_id,
            timestamps: timestamps.to_vec()
        }
    }

    pub fn get_id(&self) -> OwnershipId {
        self.id
    }

    pub fn get_account_id(&self) -> ObjectId {
        self.account_id
    }

    pub fn get_timestamps(&self) -> &[i32] {
        &self.timestamps
    }

}
