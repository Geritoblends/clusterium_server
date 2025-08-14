use twox_hash::XxHash3_128;
use mongodb::Client;
use item_store::utils::floor_div;

fn compute_xyza_hash(x: i128, y: i128, z: i128, a: i32) -> u128 {
    let mut bytes = [0u8; 52];  
    
    bytes[0..16].copy_from_slice(&x.to_le_bytes());
    bytes[16..32].copy_from_slice(&y.to_le_bytes());
    bytes[32..48].copy_from_slice(&z.to_le_bytes());
    bytes[48..52].copy_from_slice(&a.to_le_bytes());
    
    XxHash3_128::oneshot(&bytes)
}

fn compute_account_item_hash(account_id: &str, item_type: i32) -> u128 {
    let account_bytes = account_id.as_bytes();
    let item_type_bytes = item_type.to_le_bytes();
    let mut bytes = Vec::with_capacity(account_bytes.len() + 4);
    bytes.extend_from_slice(account_bytes);
    bytes.extend_from_slice(&item_type_bytes);
    XxHash3_128::oneshot(&bytes)
}

fn compute_consumed_key(x: i128, y: i128, z: i128, a: i32) -> u128 {
    let lootable_blocks_density = 0.1;
    let looted_blocks_estimate = 0.5;
    let actually_looted_avg = lootable_blocks_density / looted_blocks_estimate
    let bloom_filter_tolerable_fp = 0.1;

    let mut bytes = [0u8; 52]

    let region_size: i128 = 64; // ~ 4KB size per bloom filter
    let region_x = floor_div(x, region_size);
    let region_y = floor_div(y, region_size);
    let region_z = floor_div(z, region_size);

    bytes[0..16].copy_from_slice(&region_x.to_le_bytes());
    bytes[16..32].copy_from_slice(&region_y.to_le_bytes());
    bytes[32..48].copy_from_slice(&region_z.to_le_bytes());
    bytes[48..52].copy_from_slice(&a.to_le_bytes());

    XxHash3_128::oneshot(&bytes)
}

#[derive(Clone, Debug)]
struct ItemAmount {
    item_type: i32,
    amount: i32
}

#[derive(Clone, Debug)]
struct ItemAmountHashed {
    hashed_item_type: String,
    amount: i32
}

struct ItemStore {
    client: Client
}

trait InventoryWrites {
    async fn obtain_from_xyza();
    async fn obtain_if_buffs(); // Buffs can include from temporal potion effects to permanent
                                // 
                                // role-level permissions or advantages
    async fn obtain_xyza_bulk(); 
    async fn obtain_if_buffs_bulk();
    async fn craft();
    async fn craft_bulk();
    async fn craft_if_buffs();
    async fn craft_if_buffs_bulk();
    async fn trade();
}

trait InventoryReads {
    async fn get_full_inventory();
    async fn get_item_balance();
    async fn get_buff();
}

trait EntityActions {
    /// Entities can be chests, vehicles, or world positions ("bags" as dropped items in a world
    /// position)
    async fn transfer_to_entity();
    async fn gather_from_entity();
}

impl InventoryWrites for ItemStore {

    async fn obtain_from_xyza(&self, account_id: &str, x: i128, y: i128, z: i128, a: i32) -> Result<(), Error> {
        // 1. generate the xyza hash
        let xyza_hash = compute_xyza_hash(x, y, z, a);
        // 2. calculate the respective chunk range position
        let consumed_key = compute_consumed_key(x, y, z, a);
        // 3. query for the current data of that chunk range
        let query = doc! {"_id": consumed_key};
        loop {
            let consumed: ConsumedDoc = self.items.find_one(query).await?;
            // 4. try to feed the bloom filter with the xyza hash
            let new_filter = consumed.filter.feed(xyza_hash)?;
            // 5. if feeding it returns "might exist in set", then finish the operation as there's
            //    nothing to update and nothing to give as loot to the player (pessimistic loot drops)
            // 6. if it definitely doesnt exist, write back the document with the updated bloom filter 
            let query = doc! {
                "_id": consumed_key,
                "sequence_number": consumed.sequence_number
            };
            let update = doc! {"bit_array": new_filter};
            // only if the sequence number is still the same
            match self.items.update_one(query, update).await {
                Ok(_) => break,
                Err(mongodb::Error::NoSuchDocument) => continue,
                Err(e) => return e,
            }
        }
        // 7. if its not the same anymore, repeat from step 3 to 7 until it is either a "might
        //    exist in set" or the insertion is successful
        Ok(())
    }

    async fn craft(account_id: &str, debited: &[ItemAmount], credited: &[ItemAmount]) -> Result<(), Error> {
        // 1. create the hash of all the associated debited elements + the account_id, then query
        //    them all and check if they have enough balances
        // 2. if they do, start a transaction and debit them
        // 3. query for the credited items, update the ones that exist, and insert any that doesn't
        //    with the item amount as balance. Check for the sequence_number to still match when
        //    updating, and set the sequence number to 0 in the inserts (ofc)
    }

    async fn craft_if_buffs(account_id: &str, buffs: &[i32], debited_items: &[ItemAmount], credited_items: &[ItemAmount]) -> Result<(), Error> {
        // do the same as craft but check if the buff associated to crafting it, has an
        // expired_at value higher than date.now()
    }

    async fn trade(account_id_A: &str, transferred_items_A: &[ItemAmount], account_id_B: &str, transferred_items_B: &[ItemAmount]) -> Result<(), Error> {
        // 1. create the hash of all the associated elements for every party, and pull them
        // 2. validate if each party has the balances they propose, and if so debit them and credit
        //    the other party
    }

    async fn obtain_if_buffs(account_id: &str, buffs: &[i32], credited_items: &[ItemAmount]) -> Result<(), Error> {
        // same as obtain_from_xyza but check if the buff associated to obtaining it, has an expired_at value
        // higher than date.now()
    }

    async fn consume_for_buff(account_id: &str, debited_items: &[ItemAmount], buff: i32, duration: i64) -> Result<(), Error> {
    }

    async fn obtain_permanent_buff(account_id: &str, buff: i32) -> Result<(), Error> {}

}

impl InventoryReads for ItemStore {
    async fn read_full_inventory(account_id: &str) -> Result<Vec<ItemAmountHashed>, Error>;
    async fn read_item_balance(account_id: &str, item_type: i32) -> Result<ItemAmountHashed, Error>;
    async fn read_buff(account_id: &str, buff: i32) -> Result<i64, Error> {} // i64 for the unix
                                                                             // timestamp
}

impl EntityActions for ItemStore {
    async fn transfer_to_entity(account_id: &str, transferred_items: &[ItemAmount], entity_id: &str) -> Result<(), Error> {}
    async fn gather_from_entity(account_id: &str, entity_id: &str, gathered_items: &[ItemAmount]) -> Result<(), Error> {}
}
