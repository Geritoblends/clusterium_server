use sqlx::{PgPool, Transaction, Postgres};
use thiserror::{Error as ThisError};

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Error from Sqlx: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("Item type mismatch for stack '{stack_uuid}': expected {expected}, got {actual}")]
    ItemTypeMismatch {
        stack_uuid: String,
        expected: u32,
        actual: u32,
    },
}

struct StackLedger {
    pools: Vec<PgPool>,
}

impl StackLedger {
    
    pub async fn connect(connection: String) -> Result<Self, SqlxError> {
        let pools = PgPool::connect(connection).await?;
        Self {
            pools
        }
    }

    pub async fn create(tx: &mut Transaction<'_, Postgres>, stack_uuid: String, item_type: u32, qty: u32, account_id: String) -> Result<(), Error> {
        let key = hash_function(account_id, stack_uuid);

        // Garantiza que solo un jugador pueda obtener el drop
        sqlx::query!(
            r#"INSERT INTO consumed (uuid)
            VALUES ($1);
            "#,
            stack_uuid)
            .execute(&mut **tx)
            .await?;


        // Inserción al ledger que solo sucede si se cumple UNIQUE(stack_uuid, account_id,
        // sequence_number), lo cual elimina dupes de race conditions de raìz.
        sqlx::query!(
            r#"INSERT INTO ledger (stack_uuid, account_id, item_type, qty, current_balance, sequence_number)
            VALUES ($1, $2, $3, $4, $5, $6);
            "#,
            stack_uuid,
            account_id,
            item_type,
            qty,
            qty,
            0)
            .execute(&mut **tx)
            .await?;

        // Sirve para posteriores inserciones al ledger
        sqlx::query!(
            r#"INSERT INTO latest (key, sequence_number, balance, item_type, account_id, stack_uuid)
            VALUES ($1, $2, $3, $4, $5, $6);
            "#,
            key,
            0,
            qty,
            item_type,
            account_id,
            stack_uuid)
            .execute(&mut **tx)
            .await?;

        Ok(())
    }

    pub async fn destroy(tx: &mut Transaction<'_, Postgres>, stack_uuid: String, expected_item_type: u32, account_id: String, qty: u32) -> Result<(), Error> {
        /// Method to destroy a quantity from an existing stack
        /// for performance reasons, stack_uuid might be provided by the client, that is why we need to make sure the
        /// expected_item_type is determined server-side, this way we can compare the
        /// expected_item_type with the latest.item_type. client-side stack_uuid also introduces
        /// the possibility of the client lying about it being owner of a stack, but that is solved
        /// when querying the "latest" table for the account_id+stack_uuid key.

        let key = hash_function(account_id, stack_uuid);
        let qty: i32 = -qty;

        // Real current values table. This select also ensures the account_id really has said stack
        let latest = sqlx::query!(r#"
        SELECT sequence_number, balance, item_type
        FROM latest
        WHERE key = $1;
        "#,
        key)
            .fetch_one(&mut **tx)
            .await?;
        
        // Check if client is lying about the item type
        if expected_item_type != latest.item_type {
            tx.rollback().await?;
            return Err(Error::ItemTypeMismatch {
                stack_uuid: stack_uuid.to_string(),
                expected: expected_item_type,
                actual: latest.item_type,
            });
        }

        sqlx::query!(r#"
        INSERT INTO ledger (stack_uuid, account_id, item_type, qty, current_balance, sequence_number)
        VALUES ($1, $2, $3, $4, $5, $6);
        "#,
        stack_uuid,
        account_id,
        latest.item_type,
        qty,
        latest.balance + qty,
        latest.sequence_number + 1)
            .execute(&mut **tx)
            .await?;

        sqlx::query!(r#"
        UPDATE latest (sequence_number, balance)
        VALUES ($1, $2)
        WHERE key = $3;
        "#,
        latest.sequence_number + 1,
        latest.balance + qty,
        key)
            .execute(&mut **tx)
            .await?
            
            Ok(())
    }

    pub async fn split(&self, tx: &mut Transaction<'_, Postgres>, stack_uuid: String, expected_item_type: u32, sender_id: String, recipient_id: String, qty: u32) -> Result<(), Error> {
        let key = hash_function(recipient_id, stack_uuid);

        self.destroy(tx, stack_uuid, expected_item_type, sender_id, qty).await?;

        let result = sqlx::query!(
            r#"SELECT  sequence_number, balance, item_type
            FROM latest
            WHERE key = $1;
            "#,
            key)
            .fetch_one(&mut **tx)
            .await;

        match result {
            Ok(latest) => {

                sqlx::query!(
                    r#"INSERT INTO ledger (stack_uuid, account_id, item_type, qty, current_balance, sequence_number)
                    VALUES ($1, $2, $3, $4, $5, $6);"#,
                    stack_uuid,
                    recipient_id,
                    expected_item_type,
                    qty,
                    latest.balance + qty,
                    latest.sequence_number + 1)
                    .execute(&mut **tx)
                    .await?;

                sqlx::query!(
                    r#"UPDATE latest 
                    SET sequence_number = $1, balance = $2
                    WHERE key = $3;
                    "#,
                    latest.sequence_number + 1,
                    latest.balance + qty,
                    key)
                    .execute(&mut **tx)
                    .await?;

                },

            Err(sqlx::Error::RowNotFound) => {

                    sqlx::query!(
                        r#"INSERT INTO ledger (stack_uuid, account_id, item_type, qty, current_balance, sequence_number)
                        VALUES ($1, $2, $3, $4, $5, $6);
                        "#,
                        stack_uuid,
                        recipient_id,
                        expected_item_type,
                        qty,
                        qty,
                        0)
                        .execute(&mut **tx)
                        .await?;

                    sqlx::query!(
                        r#"INSERT INTO latest (key, sequence_number, balance, item_type, account_id, stack_uuid)
                        VALUES ($1, $2, $3, $4, $5, $6)
                        "#,
                        key,
                        0,
                        qty,
                        expected_item_type,
                        recipient_id,
                        stack_uuid)
                        .execute(&mut **tx)
                        .await?;

                    },

                Err(e) => {
                    tx.rollback().await?;
                    return Err(e);
                },

        };

        Ok(())
    }

    pub async fn create_from_xyza(&self, x: i128, y: i128, z: i128, a: u8, item_type: u32, qty: u32, account_id: String) -> Result<String, Error> {
        let stack_uuid: Uuid; // from xyz
    }
}

struct InventoryManager {
    pool: PgPool
}

impl InventoryManager {
    pub async fn get_inventory(&self, account_id: String) -> Result<Inventory, sqlx::Error> {
        let stacks = sqlx::query_as!(Stack,
            r#"SELECT stack_uuid, item_type, balance FROM latest
            WHERE account_id = $1;
            "#,
            account_id)
            .fetch_all(&mut self.pool)
            .await?;
        let inventory = Inventory::new(&stacks[..]);
        Ok(inventory)
    }
}
