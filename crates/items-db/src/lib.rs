use twox_hash::XxHash3_128;
use mongodb::Client;
use item_store::utils::{compute_xyza_hash, compute_account_item_hash, compute_consumed_key};


#[derive(Clone, Debug)]
struct ItemAmount {
    item_type: i32,
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
                Err(e) => return Err(e),
            }
        }
        // 7. if its not the same anymore, repeat from step 3 to 7 until it is either a "might
        //    exist in set" or the insertion is successful
        Ok(())
    }

    async fn craft(&self, account_id: &str, debited: &[ItemAmount], credited: &[ItemAmount]) -> Result<(), Error> {
        // 1. create the hash of all the associated debited elements + the account_id
        let debited_keys: Vec<u128> = Vec::with_capacity(debited.len())
        for item_amount in debited {
            let key = compute_account_item_hash(account_id, item_amount.item_type);
            debited_keys.push(key);
        }
        //    then query them all and check if they have enough balances
        let filter = doc! { "_id": { "$in": debited_keys } };   
        let cursor = self.items.find(filter).await?;
        let results: Vec<ItemDocument> = cursor
            .try_collect()
            .await?;
        let items = Vec<Item> = results
            .into_iter()
            .map(|doc| doc.as_item())
            .collect::<Result<Vec<Item>, _>>()?;
        
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
