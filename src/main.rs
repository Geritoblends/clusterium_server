use sqlx::PgPool;
use stack_ledger::{InventoryManager, StackLedger};
use thiserror::{Error as ThisError};

#[derive(Debug, ThisError)]
enum Error {

    #[error("Error from sqlx: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("Error from StackLedger: {0}")]
    StackLedgerError(#[from] stack_ledger::Error),

}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let pool = PgPool::connect("postgres://postgres:mysecretpassword@localhost:5432/postgres").await.expect("Couldn't connect to the database server");

    let inventory_manager = InventoryManager::new(pool.clone());
    let ledger = StackLedger::new(pool.clone());
    Ok(())
}
