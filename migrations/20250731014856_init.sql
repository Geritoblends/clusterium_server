-- Add migration script here

-- Consumed table with hash-based exclusion constraint
CREATE TABLE consumed (
    stack_uuid BIGINT NOT NULL
);

-- Hash index for lookups
-- CREATE INDEX idx_consumed_stack_uuid ON consumed USING hash (stack_uuid);
-- Por el momento no le veo utilidad, pero podría tenerla

-- Exclusion constraint for uniqueness using hash
ALTER TABLE consumed ADD CONSTRAINT exc_consumed_stack_uuid 
EXCLUDE USING hash (stack_uuid WITH =);

-- Latest balances table
CREATE TABLE latest (
    account_id TEXT NOT NULL,
    stack_uuid BIGINT NOT NULL,
    sequence_number INTEGER NOT NULL CHECK (sequence_number >= 0),
    balance INTEGER NOT NULL CHECK (balance >= 0),
    item_type INTEGER NOT NULL,
    CONSTRAINT no_sub_zero_balance CHECK (balance >= 0)
);

-- For ownership validation
CREATE INDEX idx_latest_key ON latest USING hash (account_id, stack_uuid)

-- Exclusion constraint on the generated key
ALTER TABLE latest ADD CONSTRAINT exc_latest_key 
EXCLUDE USING hash (
    account_id WITH =,
    stack_uuid WITH =
);

-- Hash index for account_id lookups (partial index excluding zero balances)
CREATE INDEX idx_latest_account_id ON latest USING hash (account_id) 
WHERE balance > 0;

-- Ledger table for transaction history
CREATE TABLE ledger (
    account_id TEXT NOT NULL,
    stack_uuid BIGINT NOT NULL,
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

