-- Add migration script here

-- Consumed table with hash-based exclusion constraint
CREATE TABLE consumed (
    stack_uuid BYTEA NOT NULL
);

-- Exclusion constraint for uniqueness using hash
ALTER TABLE consumed ADD CONSTRAINT exc_consumed_stack_uuid 
EXCLUDE USING hash (stack_uuid WITH =);

-- Latest balances table
CREATE TABLE latest (
    key BYTEA NOT NULL,
    account_id TEXT NOT NULL,
    stack_uuid BYTEA NOT NULL,
    sequence_number INTEGER NOT NULL CHECK (sequence_number >= 0),
    balance INTEGER NOT NULL CHECK (balance >= 0),
    item_type INTEGER NOT NULL,
    CONSTRAINT no_sub_zero_balance CHECK (balance >= 0)
);

-- For ownership validation
CREATE INDEX idx_latest_key ON latest USING hash (key)

-- Exclusion constraint on the generated key
ALTER TABLE latest ADD CONSTRAINT exc_latest_key 
EXCLUDE USING hash (
    key WITH =
);

-- To track prunable stacks
CREATE TABLE stacks (
    stack_uuid BYTEA NOT NULL,
    account_ids TEXT[] NOT NULL DEFAULT '{}'
);

-- B-tree partial index to track the prunable stacks, keeps o(1) on non-empty stack insertion
CREATE INDEX idx_stacks_empty_accounts ON stacks (stack_uuid)
WHERE account_ids = '{}';

-- For player inventories
CREATE TABLE inventories (
    account_id TEXT NOT NULL,
    stack_uuids BYTEA[] NOT NULL DEFAULT '{}'
);

CREATE INDEX idx_inventories_account_id_hash ON inventories USING hash (account_id);

ALTER TABLE inventories ADD CONSTRAINT exc_account_id
EXCLUDE USING hash (
    account_id WITH =
);

-- Ledger table for transaction history
CREATE TABLE ledger (
    account_id TEXT NOT NULL,
    stack_uuid BYTEA NOT NULL,
    sequence_number INTEGER NOT NULL CHECK (sequence_number >= 0),
    qty INTEGER NOT NULL,
    balance INTEGER NOT NULL CHECK (balance >= 0),
    item_type INTEGER NOT NULL,
    CONSTRAINT no_sub_zero_balance CHECK (balance >= 0)
);

-- Exclusion constraint for composite uniqueness
ALTER TABLE ledger ADD CONSTRAINT exc_ledger_composite
EXCLUDE USING hash (
    account_id WITH =,
    stack_uuid WITH =,
    sequence_number WITH =
);
