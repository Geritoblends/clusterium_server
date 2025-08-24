use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Clone, Copy)]
pub struct OwnershipId {
    bytes: [u8; 16] // 12 bytes of the AccountId and 4 bytes of the object type (i32)
                    // or buff_type
}

impl OwnershipId {

    pub fn new(bytes: &[u8; 16]) -> Self {
        Self {
            bytes: *bytes
        }
    }

    pub fn as_bytes(&self) -> &[u8; 16] {
        &self.bytes
    }

    pub fn get_obj_type(&self) -> i32 {
        let arr = self.bytes;
        i32::from_le_bytes([
            arr[12], arr[13], arr[14], arr[15]
        ])
    }

    pub fn get_object_id(&self) -> ObjectId {
        let arr = self.bytes;
        let bytes = [
            arr[0], arr[1], arr[2],
            arr[3], arr[4], arr[5],
            arr[6], arr[7], arr[8],
            arr[9], arr[10], arr[11]
        ];

        ObjectId::from_bytes(bytes)
    }

}

