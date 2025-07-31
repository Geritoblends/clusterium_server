-- Add migration script here

-- Enable pgcrypto for UUID and hash functions
CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- Consumed table with hash-based exclusion constraint
CREATE TABLE consumed (
    stack_uuid UUID NOT NULL
);

-- Hash index for lookups
CREATE INDEX idx_consumed_stack_uuid ON consumed USING hash (stack_uuid);

-- Exclusion constraint for uniqueness using hash
ALTER TABLE consumed ADD CONSTRAINT exc_consumed_stack_uuid 
EXCLUDE USING hash (stack_uuid WITH =);

-- Latest balances table
CREATE TABLE latest (
    key TEXT NOT NULL GENERATED ALWAYS AS (
        encode(digest(account_id || stack_uuid::text, 'sha256'), 'hex')
    ) STORED,
    account_id TEXT NOT NULL,
    stack_uuid UUID NOT NULL,
    sequence_number INTEGER NOT NULL CHECK (sequence_number >= 0),
    balance INTEGER NOT NULL CHECK (balance >= 0),
    item_type INTEGER NOT NULL,
    CONSTRAINT no_zero_balance CHECK (balance > 0)
);

-- Hash index for the generated key
CREATE INDEX idx_latest_key ON latest USING hash (key);

-- Exclusion constraint on the generated key
ALTER TABLE latest ADD CONSTRAINT exc_latest_key 
EXCLUDE USING hash (key WITH =);

-- Hash index for account_id lookups (partial index excluding zero balances)
CREATE INDEX idx_latest_account_id ON latest USING hash (account_id) 
WHERE balance > 0;

-- Ledger table for transaction history
CREATE TABLE ledger (
    account_id TEXT NOT NULL,
    stack_uuid UUID NOT NULL,
    sequence_number INTEGER NOT NULL CHECK (sequence_number >= 0),
    qty INTEGER NOT NULL,
    balance INTEGER NOT NULL CHECK (balance >= 0),
    item_type INTEGER NOT NULL
);

-- Exclusion constraint for composite uniqueness
ALTER TABLE ledger ADD CONSTRAINT exc_ledger_composite
EXCLUDE USING hash (
    account_id WITH =,
    stack_uuid WITH =,
    sequence_number WITH =
);
