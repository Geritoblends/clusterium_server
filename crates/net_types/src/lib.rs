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

pub trait XYZValues {
    fn get_bytes(&self) -> [u8; 24];
}

impl XYZValues {

    pub fn get_x(&self) -> i64 {
        let arr = self.get_bytes();
        i64::from_le_bytes([
            arr[0], arr[1], arr[2], arr[3],
            arr[4], arr[5], arr[6], arr[7]
        ])
    }
    
    pub fn get_y(&self) -> i64 {
        let arr = self.get_bytes();
        i64::from_le_bytes([
            arr[8], arr[9], arr[10], arr[11],
            arr[12], arr[13], arr[14], arr[15]
        ])
    }

    pub fn get_z(&self) -> i64 {
        let arr = self.get_bytes();
        i64::from_le_bytes([
            arr[16], arr[17], arr[18], arr[19],
            arr[20], arr[21], arr[22], arr[23]
        ])
    }

}

impl Position3D {
    pub fn new(bytes: &[u8; 24]) -> Self {
        Self {
            bytes
        }
    }
}

impl XYZValues for Position3D {
    fn get_bytes(&self) -> [u8; 24] {
        self.bytes
    }
}
        
struct IndividualBlockId {
    bytes: [u8; 24]
}

impl XYZValues for IndividualBlockId {
    fn get_bytes(&self) -> [u8; 24] {
        self.bytes
    }
}

impl IndividualBlockId {
    pub fn new(bytes: &[u8; 24]) -> Self {
        Self {
            bytes
        }
    }
}

struct RangeBlockId {
    bytes: [u8; 12]
}

impl RangeBlockId {

    pub fn new(bytes: &[u8; 12]) -> Self {
        Self {
            bytes
        }
    }

    pub fn get_timestamp(&self) -> i64 {
        let arr = self.get_bytes();
        i64::from_le_bytes([
            arr[0], arr[1], arr[2], arr[3],
            arr[4], arr[5], arr[6], arr[7]
        ])
    }

}

pub enum BlockId {
    Individual(IndividualBlockId),
    Range(RangeBlockId),
}

pub enum BlockType {
    Individual {
        chunk_pos: Position3D,
    },
    Range {
        from: Position3D,
        to: Position3D,
        chunks: Vec<Position3D>,
    },
}

pub struct Block {
    id: BlockId,
    block_type: BlockType,
    item_type: ItemType,
}
