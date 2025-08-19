use mongodb::bson::{Binary};

#[derive(Debug, Clone, PartialEq)]
pub struct InvalidOwnershipIdSize {
    pub expected: usize,
    pub actual: usize,
}

impl std::fmt::Display for InvalidOwnershipIdSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid OwnershipId size: expected {} bytes, got {}", 
               self.expected, self.actual)
    }
}

impl std::error::Error for InvalidOwnershipIdSize {}

impl TryFrom<Binary> for OwnershipId {
    type Error = InvalidOwnershipIdSize;
    
    fn try_from(binary: Binary) -> Result<Self, Self::Error> {
        let bytes: [u8; 12] = binary.bytes.try_into()
            .map_err(|_| InvalidOwnershipIdSize { 
                expected: 12, 
                actual: binary.bytes.len() 
            })?;
        Ok(Self::new(&bytes))
    }
}

impl From<OwnershipId> for Binary {
    fn from(ownership_id: OwnershipId) -> Self {
        Binary {
            subtype: mongodb::bson::spec::BinarySubtype::Generic,
            bytes: ownership_id.as_bytes().to_vec()
        }
    }
}
