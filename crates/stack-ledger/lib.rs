use sqlx::{PgPool, Error as SqlxError};
use thiserror::Error

#[derive(Debug, Error)]
pub enum LedgerError {
    #[error("Error from Sqlx: {0}")]
    Sqlx(#[from] SqlxError),

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

    pub async fn create(&self, stack_uuid: String, item_type: u32, qty: u32, account_id: String) -> Result<(), LedgerError> {
        let key = hash_function(account_id, stack_uuid);
        let mut tx = self.pool.begin().await?;

        // Garantiza que solo un jugador pueda obtener el drop
        sqlx::query!(
            r#"INSERT INTO consumed (uuid)
            VALUES ($1);
            "#,
            stack_uuid)
            .execute(&mut *tx)?;

        // Sirve para posteriores inserciones al ledger
        sqlx::query!(
            r#"INSERT INTO latest_account_id_x_stack_uuid (key, sequence_number, balance, item_type)
            VALUES ($1, $2, $3, $4);
            "#,
            key,
            0,
            qty,
            item_type)
            .execute(&mut *tx)?;

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
            .execute(&mut *tx)?;

        tx.commit().await?;

    }

    pub async fn destroy(&self, stack_uuid: String, expected_item_type: u32, qty: i32, account_id: String, qty: u32) -> Result<(), LedgerError> {
        /// Method to destroy a quantity from an existing stack
        /// for performance reasons, stack_uuid might be provided by the client, that is why we need to make sure the
        /// expected_item_type is determined server-side, this way we can compare the
        /// expected_item_type with the latest.item_type

        let key = hash_function(account_id, stack_uuid);
        let qty: i64 = -qty;
        let mut tx = &self.pool.begin().await?;

        // Obtención de los valores reales
        let latest = sqlx::query!(r#"
        SELECT sequence_number, balance, item_type
        FROM latest_account_id_x_stack_uuid
        WHERE key = $1;
        "#,
        key)
            .fetch_one(&mut *tx)
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
            .execute(&mut *tx)
            .await?;

        sqlx::query!(r#"
        UPDATE latest_account_id_x_stack_uuid (sequence_number, balance)
        VALUES ($1, $2)
        WHERE key = $3;
        "#,
        latest.sequence_number + 1,
        latest.balance + qty,
        key)
            .execute(&mut *tx)
            .await?
            
        tx.commit().await?;
    }

    pub async fn split(&self, payload: String) -> Result<(), LedgerError> {
    }

    pub async fn create_from_xyza(&self, x: i128, y: i128, z: i128, a: u8, item_type: u32, qty: u32, account_id: String) -> Result<String, LedgerError> {
        let stack_uuid: Uuid; // from xyz
    }
}
