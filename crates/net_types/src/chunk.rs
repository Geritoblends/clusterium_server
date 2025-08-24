use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ChunkId {
    bytes: [u8; 10] // 4 bytes for x, 4 for z and 2 for y
}

// 16x16x16
#[derive(Serialize, Deserialize)]
pub struct RelevantChunk {
    id: ChunkId,
    blocks: Vec<(i16, i32)>, // first integer is position in the flat array, second is the item_type
    version: i32, // optimistic locking
}

// 32x32x64. 16 RelevantChunks will fit in one CommonChunk. Relevant blocks might want to override the common to avoid looking up CommonChunks every time you only want to edit RelevantChunks
#[derive(Serialize, Deserialize)]
pub struct CommonChunk {
    id: ChunkId,
    blocks: Vec<(i16, i32)>,
}

pub trait XYZValues {

    fn get_bytes(&self) -> [u8; 10];

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

    fn get_z(&self) -> i16 {
        let arr = self.get_bytes();
        i16::from_le_bytes([
            arr[8], arr[9]
        ])
    }

}

impl ChunkId {

    pub fn new(bytes: &[u8; 10]) -> Self {
        Self {
            bytes
        }
    }

    pub fn as_bytes(&self) -> [u8; 10] {
        self.bytes
    }

}

impl XYZValues for ChunkId {
    fn get_bytes(&self) -> [u8; 10] {
        self.bytes
    }

}
