use mongodb::bson::{Binary};
use serde::{Serialize, Deserialize};

struct ItemDocument {
    #[serde(rename = "_id")]
    id: Binary,
    account_id: Binary,
    balance: i32,
    version: i32
}

struct BuffDocument {
    #[serde(rename = "_id")]
    id: Binary,
    account_id: Binary,
    expires_at: i64
}

enum BlockDocumentType {
    Individual {
        chunk_pos: Binary,
    },
    Range {
        from: Binary,
        to: Binary,
        chunks: Vec<Binary>,
    },
}

#[derive(Clone, Serialize, Deserialize)]
struct BlockDocument {
    #[serde(rename = "_id")]
    id: Binary,
    #[serde(flatten)]
    block_type: BlockDocumentType,
    item_type: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InvalidSize {
    OwnershipId {
        pub expected: usize,
        pub actual: usize,
    },
    AccountId {
        pub expected: usize,
        pub actual: usize,
    },
    Position3D {
        pub expected: usize,
        pub actual: usize,
    },
    IndividualBlockId {
        pub expected: usize,
        pub actual: usize,
    },
    RangeBlockId {
        pub expected: usize,
        pub actual: usize,
    },
}

impl std::fmt::Display for InvalidSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid size: expected {} bytes, got {}", 
               self.expected, self.actual)
    }
}

impl std::error::Error for InvalidSize {}

impl TryFrom<Binary> for OwnershipId {
    type Error = InvalidSize::OwnershipId;
    
    fn try_from(binary: Binary) -> Result<Self, Self::Error> {
        let bytes: [u8; 12] = binary.bytes.try_into()
            .map_err(|_| InvalidSize::OwnershipId { 
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

impl TryFrom<Binary> for AccountId {
    type Error = InvalidSize::AccountId;

    fn try_from(binary: Binary) -> Result<Self, Self::Error> {
        let bytes: [u8; 12] = binary.bytes.try_into()
            .map-err(|_| InvalidSize::AccountId {
                expected: 16,
                actual: binary.bytes.len()
            })?;
        Ok(Self::new(&bytes))
    }
}

impl From<AccountId> for Binary {
    fn from(account_id: AccountId) -> Self {
        Binary {
            subtype: mongodb::bson::spec::BinarySubtype::Generic,
            bytes: account_id.as_bytes().to_vec()
        }
    }
}

impl From<Item> for ItemDocument {
    fn from(item: Item) -> Self {
        ItemDocument {
            id: item.get_id().into(),
            account_id: item.get_account_id().into(),
            balance: item.get_balance(),
            version: item.get_version()
        }
    }
}

impl TryFrom<ItemDocument> for Item {
    type Error = InvalidSize;
    fn try_from(doc: ItemDocument) -> Result<Self, Self::Error> {
        let ownership_id: OwnershipId = doc.id.try_into()?;
        let account_id: AccountId = doc.account_id.try_into()?;
        Ok(Self::new(ownership_id, account_id, doc.balance, doc.version))
    }
}

impl From<Buff> for BuffDocument {
    fn from(buff: Buff) -> Self {
        BuffDocument {
            id: buff.get_id().into(),
            account_id: buff.get_account_id().into(),
            expires_at: buff.get_expires_at()
        }
    }
}

impl TryFrom<BuffDocument> for Buff {
    type Error = InvalidSize;
    fn try_from(doc: BuffDocument) -> Result<Self, Self::Error> {
        let ownership_id: OwnershipId = doc.id.try_into()?;
        let account_id: AccountId = doc.account_id.try_into()?;
        Ok(Self::new(ownership_id, account_id, doc.expires_at))
    }
}

impl From<Position3D> for Binary {
    fn from(position: Position3D) -> Self {
        Binary {
            subtype: mongodb::bson::spec::BinarySubtype::Generic,
            bytes: position.as_bytes().to_vec()
        }
    }
}

impl TryFrom<Binary> for Position3D {
    type Error = InvalidSize::Position3D;
    
    fn try_from(binary: Binary) -> Result<Self, Self::Error> {
        let bytes: [u8; 24] = binary.bytes.try_into()
            .map_err(|_| InvalidSize::Position3D { 
                expected: 24, 
                actual: binary.bytes.len() 
            })?;
        Ok(Self::new(&bytes))
    }
}

impl From<IndividualBlockId> for Binary {
    fn from(id: IndividualBlockId) -> Self {
        Binary {
            subtype: mongodb::bson::spec::BinarySubtype::Generic,
            bytes: id.as_bytes().to_vec()
        }
    }
}

impl TryFrom<Binary> for IndividualBlockId {
    type Error = InvalidSize::IndividualBlockId;
    
    fn try_from(binary: Binary) -> Result<Self, Self::Error> {
        let bytes: [u8; 24] = binary.bytes.try_into()
            .map_err(|_| InvalidSize::IndividualBlockId { 
                expected: 24, 
                actual: binary.bytes.len() 
            })?;
        Ok(Self::new(&bytes))
    }
}

impl From<RangeBlockId> for Binary {
    fn from(id: RangeBlockId) -> Self {
        Binary {
            subtype: mongodb::bson::spec::BinarySubtype::Generic,
            bytes: id.as_bytes().to_vec()
        }
    }
}

impl TryFrom<Binary> for RangeBlockId {
    type Error = InvalidSize::RangeBlockId;
    
    fn try_from(binary: Binary) -> Result<Self, Self::Error> {
        let bytes: [u8; 12] = binary.bytes.try_into()
            .map_err(|_| InvalidSize::RangeBlockId { 
                expected: 12, 
                actual: binary.bytes.len() 
            })?;
        Ok(Self::new(&bytes))
    }
}

impl From<BlockType> for BlockDocumentType {
    fn from(block_type: BlockType) -> Self {
        match block_type {

            BlockType::Individual {chunk_pos} => {
                BlockDocumentType::Individual {
                    chunk_pos: chunk_pos.into()
                }
            },

            BlockType::Range {from, to, chunks} => {
                BlockDocumentType::Range {
                    from: from.into(),
                    to: to.into(),
                    chunks: chunks.into_iter().map(|chunk| chunk.into()).collect()
                }
            }

        }
    }
}

impl TryFrom<BlockDocumentType> for BlockType {
    type Error = InvalidSize;

    fn try_from(doc: BlockDocumentType) -> Result<Self, Self::Error> {
        match doc {

            BlockDocumentType::Individual {chunk_pos} => {
                Ok(BlockType::Individual {
                    chunk_pos: chunk_pos.try_into()?
                })
            },

            BlockDocumentType::Range {from, to, chunks} => {
                Ok(BlockType::Range {
                    from: from.try_into()?,
                    to: to.try_into()?,
                    chunks: chunks.into_iter().map(|chunk| chunk.try_into()).try_collect()?
                })
            }

        }
    }
}
        
impl From<Block> for BlockDocument {
    fn from(block: Block) -> Self {
        BlockDocument {
            id: block.id.into(),
            block_type: block.block_type.into(),
            item_type: block.item_type
        }
    }
}

impl TryFrom<BlockDocument> for Block {
    type Error = InvalidSize;

    fn try_from(doc: BlockDocument) -> Result<Self, Self::Error> {
        Block {
            id: doc.id.try_into()?,
            block_type: doc.block_type.try_into()?,
            item_type: doc.item_type
        }
    }
}


