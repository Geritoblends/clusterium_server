use serde::{Serialize, Deserialize};
use mongodb::bson::{Oid::ObjectId, DateTime};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

pub struct ChunkId {
    bytes: [u8; 10] // 4 bytes for x, 4 for z and 2 for y
}

// 16x16x16
pub struct RelevantChunk {
    id: ChunkId,
    blocks: Vec<(i16, i32)>, // first integer is position in the flat array, second is the item_type
    version: i32, // optimistic locking
}

// 32x32x64. 16 RelevantChunks will fit in one CommonChunk. Relevant blocks will overlap the common
pub struct CommonChunk {
    id: ChunkId,
    blocks: Vec<(i16, i32)>,
}

#[derive(Serialize, Deserialize)]
pub enum OwnerId {
    Player(ObjectId),
}

impl OwnerId {

    fn as_bytes(&self) -> [u8; 12] {
        match self {
            Player(object_id) => {
                object_id.as_bytes()
            },
            Block(individual_block_id) => {
                individual_block_id.as_bytes()
            }
        }
    }

}

pub struct OwnershipId {
    bytes: [u8; 16] // 12 bytes of the OwnerId and 4 bytes of the object type (i32)
                    // or buff_type
}

impl OwnershipId {

    pub fn new(bytes: &[u8; 16]) -> Self {
        Self {
            bytes: *bytes
        }
    }

    /* pub fn as_base64(&self) -> String {
        BASE64.encode(&self.bytes)
    } */

    pub fn as_bytes(&self) -> &[u8; 16] {
        &self.bytes
    }

    pub fn get_obj_type(&self) -> i32 {
        let arr = self.bytes;
        i32::from_le_bytes([
            arr[12], arr[13], arr[14], arr[15]
        ])
    }

    pub fn get_owner_bytes(&self) -> [u8; 12] {
        let arr = self.bytes;
        [
            arr[0], arr[1], arr[2],
            arr[3], arr[4], arr[5],
            arr[6], arr[7], arr[8],
            arr[9], arr[10], arr[11]
        ]
    }

}

pub struct Item {
    id: OwnershipId,
    owner_id: OwnerId,
    balance: i32,
    version: i32
} 

impl Item {

    pub fn new(id: OwnershipId, owner_id: OwnerId, balance: i32, version: i32) -> Self {
        Self {
            id,
            owner_id,
            balance,
            version
        }
    }

    pub fn get_id(&self) -> OwnershipId {
        self.id
    }

    pub fn get_owner_id(&self) -> OwnerId {
        self.owner_id
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
    owner_id: OwnerId,
    expires_at: i64,
}

impl Buff {

    pub fn new(id: OwnershipId, owner_id: OwnerId, expires_at: i64) -> Self {
        Self {
            id,
            owner_id,
            expires_at
        }
    }

    pub fn get_id(&self) -> OwnershipId {
        self.id
    }

    pub fn get_owner_id(&self) -> OwnerId {
        self.owner_id
    }

    pub fn get_expires_at(&self) -> i64 {
        self.expires_at
    }

}

pub trait XYZValues {

    fn get_bytes(&self) -> [u8; 12];

    fn get_x(&self) -> i32 {
        let arr = self.get_bytes();
        i32::from_le_bytes([
            arr[0], arr[1], arr[2], arr[3]
        ])
    }

    fn get_y(&self) -> i32 {
        let arr = self.get_bytes();
        i32::from_le_bytes([
            arr[4], arr[5], arr[6], arr[7]
        ])
    }

    fn get_z(&self) -> i32 {
        let arr = self.get_bytes();
        i32::from_le_bytes([
            arr[8], arr[9], arr[10], arr[11]
        ])
    }

}

struct Position3D {
    bytes: [u8; 12]
}

impl Position3D {

    pub fn new(bytes: &[u8; 12]) -> Self {
        Self {
            bytes
        }
    }

    pub fn as_bytes(&self) -> &[u8; 12] {
        self.bytes
    }

}

impl XYZValues for Position3D {

    fn get_bytes(&self) -> [u8; 12] {
        self.bytes
    }

}
        

impl XYZValues for IndividualBlockId {
    fn get_bytes(&self) -> [u8; 12] {
        self.bytes
    }
}

impl IndividualBlockId {

    pub fn new(bytes: &[u8; 12]) -> Self {
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
    item_type: i32,
}

