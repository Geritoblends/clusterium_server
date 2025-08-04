use sqlx::{PgPool, Transaction, Postgres};
use thiserror::{Error as ThisError};
use twox_hash::XxHash3_128;

#[derive(Debug, ThisError)]
pub enum Error {

    #[error("Error from Sqlx: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("Item type mismatch for stack '{stack_uuid}': expected {expected}, got {actual}")]
    ItemTypeMismatch {
        account_id: String,
        stack_uuid: u128,
        expected: i32,
        actual: i32,
    },

    #[error("Not enough balance for stack '{stack_uuid}': requested {qty}, had {balance}")]
    NotEnoughBalance {
        account_id: String,
        stack_uuid: u128,
        qty: i32,
        balance: i32,
    },

}

fn compute_xyza_uuid(x: i128, y: i128, z: i128, a: u32) -> u128 {
    let mut bytes = [0u8; 52];  
    
    bytes[0..16].copy_from_slice(&x.to_le_bytes());
    bytes[16..32].copy_from_slice(&y.to_le_bytes());
    bytes[32..48].copy_from_slice(&z.to_le_bytes());
    bytes[48..52].copy_from_slice(&a.to_le_bytes());
    
    XxHash3_128::oneshot(&bytes)
}

fn compute_latest_key(account_id: &str, stack_uuid: u128) -> u128 {
    let account_bytes = account_id.as_bytes();
    let uuid_bytes = stack_uuid.to_le_bytes();
    let mut bytes = Vec::with_capacity(account_bytes.len() + 16);
    bytes.extend_from_slice(account_bytes);
    bytes.extend_from_slice(&uuid_bytes);
    XxHash3_128::oneshot(&bytes)
}

pub struct StackLedger {
    pool: PgPool,
}

impl StackLedger {

    pub async fn connect(connection: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPool::connect(connection).await?;
        Ok(Self {
            pool
        })
    }

    pub async fn new(pool: PgPool) -> Self {
        Self {
            pool
        }
    }

    async fn create(tx: &mut Transaction<'_, Postgres>, stack_uuid: u128, item_type: i32, qty: u32, account_id: &str) -> Result<(), Error> {

        let latest_key = compute_latest_key(account_id, stack_uuid);
        let stack_uuid_bytes = stack_uuid.to_le_bytes();
        let latest_key_bytes = latest_key.to_le_bytes();

        // Garantiza que solo un jugador pueda obtener el drop
        sqlx::query!(
            r#"INSERT INTO consumed (stack_uuid)
            VALUES ($1);
            "#,
            stack_uuid_bytes.as_slice())
            .execute(&mut **tx)
            .await?;

        let ledger_entry = sqlx::query!(
            r#"INSERT INTO ledger (account_id, stack_uuid, sequence_number, qty, balance, item_type)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING key;
            "#,
            account_id,
            stack_uuid_bytes.as_slice(),
            0,
            qty as i32,
            qty as i32,
            item_type)
            .fetch_one(&mut **tx)
            .await?;

        // Sirve para posteriores inserciones al ledger
        sqlx::query!(
            r#"INSERT INTO latest (key, account_id, stack_uuid, sequence_number, balance, item_type)
            VALUES ($1, $2, $3, $4, $5, $6);
            "#,
            latest_key_bytes.as_slice(),
            account_id,
            stack_uuid_bytes.as_slice(),
            0,
            qty as i32,
            item_type)
            .execute(&mut **tx)
            .await?;

       sqlx::query!(
           r#"INSERT INTO stacks (stack_uuid, latest_keys, ledger_entries)
           VALUES ($1, $2, $3);"#,
           stack_uuid_bytes.as_slice(),
           &[latest_key_bytes.to_vec()],
           &[ledger_entry.key])
           .execute(&mut **tx)
           .await?;

        sqlx::query!(
            r#"UPDATE inventories 
            SET latest_keys = array_append(latest_keys, $1)
            WHERE account_id = $2;"#,
            latest_key_bytes.as_slice(), 
            account_id)
            .execute(&mut **tx)
            .await?;

        Ok(())
    }

    pub async fn destroy(tx: &mut Transaction<'_, Postgres>, stack_uuid: u128, expected_item_type: i32, account_id: &str, qty: u32) -> Result<(), Error> {

        let stack_uuid_bytes = stack_uuid.to_le_bytes();
        let latest_key = compute_latest_key(account_id, stack_uuid);
        let latest_key_bytes = latest_key.to_le_bytes();
        let qty = qty as i32;

        let latest = sqlx::query!(r#"
        SELECT sequence_number, balance, item_type
        FROM latest
        WHERE key = $1;
        "#,
        latest_key_bytes.as_slice())
            .fetch_one(&mut **tx)
            .await?;
        
        // Check if client is lying about the item type
        if expected_item_type != latest.item_type {
            // tx.rollback().await?; Rollback happens automatically
            return Err(Error::ItemTypeMismatch {
                account_id: account_id.to_string(),
                stack_uuid,
                expected: expected_item_type,
                actual: latest.item_type,
            });
        }

        if qty > latest.balance {
            // tx.rollback().await?; 
            return Err(Error::NotEnoughBalance {
                account_id: account_id.to_string(),
                stack_uuid,
                qty,
                balance: latest.balance
            });
        }

        sqlx::query!(r#"
        INSERT INTO ledger (stack_uuid, account_id, item_type, qty, balance, sequence_number)
        VALUES ($1, $2, $3, $4, $5, $6);
        "#,
        stack_uuid_bytes.as_slice(),
        account_id,
        latest.item_type,
        qty,
        latest.balance + qty,
        latest.sequence_number + 1)
            .execute(&mut **tx)
            .await?;

        sqlx::query!(r#"
        UPDATE latest 
        SET sequence_number = $1, balance = $2
        WHERE account_id = $3 AND stack_uuid = $4;
        "#,
        latest.sequence_number + 1,
        latest.balance + qty,
        account_id,
        stack_uuid_bytes.as_slice())
            .execute(&mut **tx)
            .await?;

        let empty_stack: bool = latest.balance == qty;           

        if empty_stack {
            sqlx::query!(r#"
            UPDATE inventories
            SET latest_keys = array_remove(latest_keys, $1)
            WHERE account_id = $2;"#,
            latest_key_bytes.as_slice(),
            account_id)
                .execute(&mut **tx)
                .await?;

            sqlx::query!(r#"
            UPDATE stacks
            SET latest_keys = array_remove(latest_keys, $1)
            WHERE stack_uuid = $2;"#,
            latest_key_bytes.as_slice(),
            stack_uuid_bytes.as_slice())
                .execute(&mut **tx)
                .await?;

        }

            Ok(())
    }

    pub async fn split(&self, tx: &mut Transaction<'_, Postgres>, stack_uuid: u128, expected_item_type: i32, sender_id: &str, recipient_id: &str, qty: u32) -> Result<(), Error> {

        Self::destroy(tx, stack_uuid, expected_item_type, sender_id, qty).await?;
        
        let stack_uuid_bytes = stack_uuid.to_le_bytes();
        let qty = qty as i32;

        let result = sqlx::query!(
            r#"SELECT  key, sequence_number, balance, item_type
            FROM latest
            WHERE account_id = $1 AND stack_uuid = $2;
            "#,
            recipient_id,
            stack_uuid_bytes.as_slice())
            .fetch_one(&mut **tx)
            .await;

        match result {
            Ok(latest) => {

                sqlx::query!(
                    r#"INSERT INTO ledger (stack_uuid, account_id, item_type, qty, balance, sequence_number)
                    VALUES ($1, $2, $3, $4, $5, $6);"#,
                    stack_uuid_bytes.as_slice(),
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
                    WHERE account_id = $3 AND stack_uuid = $4;
                    "#,
                    latest.sequence_number + 1,
                    latest.balance + qty,
                    recipient_id,
                    stack_uuid_bytes.as_slice())
                    .execute(&mut **tx)
                    .await?;

                // Having a balance of 0 means the user has had this stack, but the balance was
                // empty, so stacks and inventories were updated previously and now we need to
                // update again
                if latest.balance == 0 {

                    sqlx::query!(
                        r#"UPDATE inventories
                        SET latest_keys = array_append(latest_keys, $1)
                        WHERE account_id = $2;"#,
                        latest.key,
                        recipient_id)
                        .execute(&mut **tx)
                        .await?;

                    sqlx::query!(
                        r#"UPDATE stacks
                        SET latest_keys = array_append(latest_keys, $1)
                        WHERE stack_uuid = $2;"#,
                        latest.key,
                        stack_uuid_bytes.as_slice())
                        .execute(&mut **tx)
                        .await?;

                }

            },

            Err(sqlx::Error::RowNotFound) => {

                sqlx::query!(
                    r#"INSERT INTO ledger (stack_uuid, account_id, item_type, qty, balance, sequence_number)
                    VALUES ($1, $2, $3, $4, $5, $6);
                    "#,
                    stack_uuid_bytes.as_slice(),
                    recipient_id,
                    expected_item_type,
                    qty,
                    qty,
                    0)
                    .execute(&mut **tx)
                    .await?;

                let latest_key = compute_latest_key(recipient_id, stack_uuid);
                let latest_key_bytes = latest_key.to_le_bytes();

                sqlx::query!(
                    r#"INSERT INTO latest (key, account_id, stack_uuid, sequence_number, balance, item_type)
                    VALUES ($1, $2, $3, $4, $5, $6);
                    "#,
                    latest_key_bytes.as_slice(),
                    recipient_id,
                    stack_uuid_bytes.as_slice(),
                    0,
                    qty,
                    expected_item_type)
                    .execute(&mut **tx)
                    .await?;

                sqlx::query!(
                    r#"UPDATE inventories
                    SET latest_keys = array_append(latest_keys, $1)
                    WHERE account_id = $2;"#,
                    latest_key_bytes.as_slice(),
                    recipient_id)
                    .execute(&mut **tx)
                    .await?;

                sqlx::query!(
                    r#"UPDATE stacks
                    SET latest_keys = array_append(latest_keys, $1)
                    WHERE stack_uuid = $2;"#,
                    latest_key_bytes.as_slice(),
                    stack_uuid_bytes.as_slice())
                    .execute(&mut **tx)
                    .await?;


            },

            Err(e) => {
                // tx.rollback().await?;
                return Err(Error::Sqlx(e));
            },

        };

        Ok(())
    }

    pub async fn create_from_xyza(&self, tx: &mut Transaction<'_, Postgres>, x: i128, y: i128, z: i128, a: u32, item_type: i32, qty: u32, account_id: &str) -> Result<u128, Error> {
        let stack_uuid: u128 = compute_xyza_uuid(x, y, z, a); 

        Self::create(tx, stack_uuid, item_type, qty, account_id).await?;
        Ok(stack_uuid)
    }
}

#[derive(Clone)]
pub struct Stack {
    stack_uuid: Vec<u8>,
    balance: i32,
    item_type: i32,
}

impl Stack {

    pub fn get_uuid(&self) -> Result<u128, std::array::TryFromSliceError> {
        let uuid_bytes: [u8; 16] = self.stack_uuid
            .as_slice()
            .try_into()?;
        Ok(u128::from_le_bytes(uuid_bytes))
    }

    pub fn get_balance(&self) -> i32 {
        self.balance
    }

    pub fn get_type(&self) -> i32 {
        self.item_type
    }

}

pub struct Inventory {
    stacks: Vec<Stack>
}

impl Inventory {
    pub fn new(stacks: &[Stack]) -> Self {
        Self {
            stacks: stacks.to_vec(),
        }
    }
}

pub struct InventoryManager {
    pool: PgPool
}

impl InventoryManager {

    pub async fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_inventory(&self, account_id: &str) -> Result<Inventory, sqlx::Error> {

        let inventory_row = sqlx::query!(
            r#"SELECT latest_keys FROM inventories
            WHERE account_id = $1;"#,
            account_id)
            .fetch_one(&self.pool)
            .await?;
        
        let stacks = sqlx::query_as!(Stack,
            r#"SELECT stack_uuid, balance, item_type FROM latest
            WHERE key = ANY($1);"#,
            &inventory_row.latest_keys)
            .fetch_all(&self.pool)
            .await?;


        let inventory = Inventory::new(&stacks);
        Ok(inventory)
    }
}
