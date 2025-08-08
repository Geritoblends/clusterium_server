// Alternative approach: Use unique hashed index (MongoDB 4.4+)
// db.consumed.createIndex({ "chunkrange_hash": "hashed" }, { unique: true })

// Or use upsert pattern in application code:
/*
db.consumed.replaceOne(
  { "chunkrange_hash": hashValue },
  {
    "chunkrange_hash": hashValue,
    "bloom_filter": { ... }
  },
  { upsert: true }
)
*/// MongoDB Schema and Index Setup

// ===============================
// ITEMS COLLECTION
// ===============================

// 1. Create the items collection with validation
db.createCollection("items", {
  validator: {
    $jsonSchema: {
      bsonType: "object",
      required: ["_id", "account_id", "item_type", "balance", "sequence_number"],
      properties: {
        _id: {
          bsonType: "binData",
          description: "16-byte UUID/hash for the item"
        },
        account_id: {
          bsonType: "binData", 
          description: "Account identifier bytes"
        },
        item_type: {
          bsonType: "int",
          description: "Type of item (int32)"
        },
        balance: {
          bsonType: "int",
          minimum: 0,
          description: "Item balance/quantity (int32)"
        },
        sequence_number: {
          bsonType: "int",
          description: "Sequence number (int32)"
        },
        expires_at: {
          bsonType: "long",
          description: "Expiration timestamp for buffs (int64)"
        }
      }
    }
  }
})

// 2. Create indexes for items collection
// Shard key (hashed index on _id)
db.items.createIndex({ "_id": "hashed" })

// Account lookup (hashed index - best for equality queries with 10B+ values)
db.items.createIndex({ "account_id": "hashed" })

// 3. Enable sharding on items collection
sh.enableSharding("clusterium")
sh.shardCollection("clusterium.items", { "_id": "hashed" })

// ===============================
// CONSUMED COLLECTION  
// ===============================

// 1. Create the consumed collection
db.createCollection("consumed", {
  validator: {
    $jsonSchema: {
      bsonType: "object",
      required: ["chunkrange_hash", "bloom_filter", "sequence_number"],
      properties: {
        chunkrange_hash: {
          bsonType: "binData",
          description: "16-byte hash identifying the chunk range"
        },
        bloom_filter: {
          bsonType: "object",
          required: ["bit_array", "num_hashes", "num_bits"],
          properties: {
            bit_array: {
              bsonType: "binData",
              description: "Bloom filter bit array"
            },
            num_hashes: {
              bsonType: "int",
              description: "Number of hash functions"
            },
            num_bits: {
              bsonType: "int", 
              description: "Number of bits in the filter"
            }
          }
        },
        sequence_number: {
            bsonType: "int",
            description: "the sequence number (int32)"
        }
      }
    }
  }
})

// 2. Create indexes for consumed collection
// Hashed index for lookups
db.consumed.createIndex({ "chunkrange_hash": "hashed" }, { unique: true })

// On player disconnect (after 5 minutes):
/*
db.items.updateMany(
  { account_id: disconnectedAccountId },
  { 
    $unset: { expired_keys: 1 },    // Remove the array
    $set: { sequence_number: 0 }    // Reset to 0
  }
)
*/

// ===============================
// SAMPLE DOCUMENTS
// ===============================

// Items collection document structure:
/*
{
  "_id": BinData(4, "base64-encoded-16-byte-hash"),
  "account_id": BinData(4, "base64-encoded-account-id"),
  "item_type": 3,
  "balance": 100,
  "sequence_number": 42,
  "expired_keys": [123, 456, 789],  // Array of u16 integers
  "expires_at": NumberLong("82928393029283930202882992")  // Only for buffs
}
*/

// Consumed collection document structure:
/*
{
  "chunkrange_hash": BinData(4, "base64-encoded-16-byte-hash"),
  "bloom_filter": {
    "bit_array": BinData(0, "base64-encoded-bitarray"),
    "num_hashes": 7,
    "num_bits": 1024
  }
}
*/

// ===============================
// INDEX STRATEGY EXPLANATION
// ===============================

/*
FOR ITEMS COLLECTION:

1. _id (hashed index): 
   - Used as shard key for even distribution
   - Perfect for single-document lookups
   - Hash distribution prevents hotspots

2. account_id (hashed index):
   - With 10B+ unique values, hash index is optimal
   - O(1) average lookup time vs O(log n) for B-tree
   - Collision rate will be very low with good hash function
   - Saves significant memory vs B-tree at this scale

3. Optional compound index (account_id + item_type):
   - Use B-tree here since you might want range queries
   - Good for queries like "get all items of type X for account Y"

FOR CONSUMED COLLECTION:

1. chunkrange_hash (hashed index):
   - Since it's a hash of chunk position, only exact equality lookups needed
   - Hashed index provides O(1) average lookup time
   - More efficient than B-tree for this use case
   - Better memory usage for large datasets that grow forever

PERFORMANCE CONSIDERATIONS:
- Hash indexes: O(1) average, but can degrade with collisions
- With 10B values and good hash function, collision rate < 1%
- B-tree indexes: O(log n) but very predictable performance
- Memory usage: Hash indexes generally use less memory
*/

