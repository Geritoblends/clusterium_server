use mongodb::Client;

struct ItemStore {
    client: Client
}

trait InventoryActions {
    async fn obtain_from_xyza();
    async fn craft();
    async fn drop();
    async fn trade();
    async fn obtain_if_buffs();
}

