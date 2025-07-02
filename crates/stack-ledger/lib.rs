use sqlx::{PgPool, Error as SqlxError};
use thiserror::Error

#[derive(Debug, Error)]
pub enum LedgerError {
    #[error("Error from Sqlx: {0}")]
    Sqlx(#[from] SqlxError),
}

struct StackLedger {
    pools: Vec<SqlitePool>,
}

impl StackLedger {
    pub async fn bind(connection: String) -> Result<Self, LedgerError> {
        let pools = PgPool::connect(connection).await?;

        Self {
            pools: pools,
        }
    }

    // Todas las funciones devuelven el uuid de la entry

    pub async fn create(&self, item_id: String, qty: i32, account_id: String) -> Result<String, LedgerError> {
    }

    pub async fn destroy(&self, item_uuid: String, qty: i32, account_id: String) -> Result<String LedgerError> {
    }

    pub async fn split(&self, payload: String) -> Result<(), LedgerError> {
    }

}
