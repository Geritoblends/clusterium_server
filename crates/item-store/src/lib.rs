use mongodb::Client;

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
                                // role-level permissions or advantages
    async fn craft();
    async fn craft_if_buffs();
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

    async fn obtain_from_xyza(account_id: &str, ) {
        // 1. generate the xyza hash
        // 2. calculate the respective chunk range position
        // 3. query for the current data of that chunk range
        // 4. try to feed the bloom filter with the xyza hash
        // 5. if feeding it returns "might exist in set", then finish the operation as there's
        //    nothing to update and nothing to give as loot to the player (pessimistic loot drops)
        // 6. if it definitely doesnt exist, write back the document with the updated bloom filter 
        // only if the sequence number is still the same
        // 7. if its not the same anymore, repeat from step 3 to 7 until it is either a "might
        //    exist in set" or the insertion is successful
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
