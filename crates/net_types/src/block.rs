use serde::{Serialize, Deserialize};
use crate::utils::{floori32, floori16};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Block {
    x: i32,
    y: i32,
    z: i16,
    item_type: i32,
}

impl Block {

    pub fn new(x: i32, y: i32, z: i16, item_type: i32) -> Self {
        Self {
            x,
            y,
            z,
            item_type,
        }
    }

    pub fn get_common_chunk_id(&self) -> ChunkId {
        let mut bytes = [0u8; 10];

        let common_x = floori32(self.x, 32);
        let common_y = floori32(self.y, 32);
        let common_z = floori16(self.z, 64);

        let x_bytes = common_x.to_le_bytes();
        let y_bytes = common_y.to_le_bytes();
        let z_bytes = common_z.to_le_bytes();

        bytes[0..4].copy_from_slice(&x_bytes);
        bytes[4..8].copy_from_slice(&y_bytes);
        bytes[8..10].copy_from_slice(&z_bytes);

        ChunkId::new(&bytes)
    }

    pub fn get_relevant_chunk_id(&self) -> ChunkId {
        let mut bytes = [0u8; 10];

        let relevant_x = floori32(self.x, 16);
        let relevant_y = floori32(self.y, 16);
        let relevant_z = floori16(self.z, 16);

        let x_bytes = relevant_x.to_le_bytes();
        let y_bytes = relevant_y.to_le_bytes();
        let z_bytes = relevant_z.to_le_bytes();

        bytes[0..4].copy_from_slice(&x_bytes);
        bytes[4..8].copy_from_slice(&y_bytes);
        bytes[8..10].copy_from_slice(&z_bytes);

        ChunkId::new(&bytes)
    }

}
