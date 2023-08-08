CREATE TABLE block (
    block_hash VARCHAR(255) NOT NULL,
    height BIGINT NOT NULL,
    size BIGINT NOT NULL,
    tx_count BIGINT NOT NULL,
    time TIMESTAMP NOT NULL,
    app_hash VARCHAR(255) NOT NULL,
    proposer VARCHAR(255) NOT NULL,
    block_data JSONB NOT NULL,
    PRIMARY KEY(block_hash)
);
CREATE INDEX block_height_index ON block (height);
CREATE INDEX block_time_index ON block (time);
CREATE INDEX block_proposer_index ON block (proposer);

CREATE TABLE transaction (
     tx_hash VARCHAR(255) NOT NULL,
     block_hash VARCHAR(255) NOT NULL,
     height BIGINT NOT NULL,
     timestamp BIGINT NOT NULL,
     code BIGINT NOT NULL,
     ty INT NOT NULL,
     log TEXT,
     origin TEXT NOT NULL,
     result JSONB NOT NULL,
     value JSONB NOT NULL,
     PRIMARY KEY(tx_hash)
);
CREATE INDEX tx_block_index ON transaction (block_hash);
CREATE INDEX tx_height_index ON transaction (height);
CREATE INDEX tx_timestamp_index ON transaction (timestamp);

CREATE TABLE e2n (
    tx_hash VARCHAR(255) NOT NULL,
    block_hash VARCHAR(255) NOT NULL,
    sender VARCHAR(255) NOT NULL,
    receiver VARCHAR(255) NOT NULL,
    asset VARCHAR(255) NOT NULL,
    amount VARCHAR(255) NOT NULL,
    decimal integer NOT NULL,
    height BIGINT NOT NULL,
    timestamp BIGINT NOT NULL,
    value JSONB NOT NULL,
    Primary Key (tx_hash)
);

CREATE INDEX e2n_sender_index ON e2n (sender);
CREATE INDEX e2n_receiver_index ON e2n (receiver);
CREATE INDEX e2n_tx_hash_index ON e2n(tx_hash);
CREATE INDEX e2n_block_hash_index ON e2n(block_hash);
CREATE INDEX e2n_height_index ON e2n(height);

CREATE TABLE validators (
    address VARCHAR(255) NOT NULL,
    pubkey_type INT NOT NULL,
    pubkey VARCHAR(255) NOT NULL,
    PRIMARY KEY(address)
);

CREATE TABLE block_generation (
    height BIGINT NOT NULL,
    address VARCHAR(255) NOT NULL,
    power BIGINT NOT NULL,
    priority BIGINT,
    signature VARCHAR(255),
    time TIMESTAMP,
    PRIMARY KEY(height, address)
);
CREATE INDEX bg_height_addr_index ON block_generation(height, address);
CREATE INDEX bg_sig_index ON block_generation(signature);
CREATE INDEX bg_time_index ON block_generation(time);

CREATE TABLE last_height (
    tip VARCHAR(255) NOT NULL,
    height BIGINT NOT NULL,
    PRIMARY KEY(tip)
);

CREATE TABLE e2n_last_height (
    tip VARCHAR(255) NOT NULL,
    height BIGINT NOT NULL,
    PRIMARY KEY(tip)
);

-- Migrate
-- Tx Type
create table tx_types(
    tx varchar(64) not null,
    ty integer not null,
    primary key (tx)
);

-- Native Transfer
create table native_txs(
    tx varchar(64) not null,
    block varchar(64) not null,
    address varchar(62) not null,
    height bigint not null,
    timestamp bigint not null,
    content jsonb not null,
    primary key (tx)
);
create index nt_block_index on native_txs(block);
create index nt_addr_index on native_txs(address);
create index nt_height_index on native_txs(height);
create index nt_time_index on native_txs(timestamp);

-- EVM Transfer
create table evm_txs(
    tx varchar(64) not null,
    block varchar(64) not null,
    evm_tx varchar(66) not null,
    sender varchar(42) not null,
    receiver varchar(42) not null,
    amount varchar(32) not null,
    height bigint not null,
    timestamp bigint not null,
    content jsonb not null,
    primary key (tx)
);
create index et_block_index on evm_txs(block);
create index et_evm_tx_index on evm_txs(evm_tx);
create index et_sender_index on evm_txs(sender);
create index et_receiver_index on evm_txs(receiver);
create index et_height_index on evm_txs(height);
create index et_time_index on evm_txs(timestamp);

-- DefineAsset
create table defined_assets(
    asset varchar(44) not null,
    tx varchar(64) not null,
    block varchar(64) not null,
    issuer varchar(62) not null,
    max_units varchar(32) not null,
    decimal integer not null,
    height bigint not null,
    timestamp bigint not null,
    content jsonb not null,
    primary key (asset)
);
create index da_tx_index on defined_assets(tx);
create index da_block_index on defined_assets(block);
create index da_issuer_index on defined_assets(issuer);
create index da_time_index on defined_assets(timestamp);
create index da_height_index on defined_assets(height);

-- IssueAsset
create table issued_assets(
    asset varchar(44) not null,
    tx varchar(64) not null,
    block varchar(64) not null,
    issuer varchar(62) not null,
    height bigint not null,
    timestamp bigint not null,
    content jsonb not null,
    primary key (asset)
);
create index ia_issuer_index on issued_assets(issuer);
create index ia_tx_index on issued_assets(tx);
create index ia_block_index on issued_assets(block);
create index ia_time_index on issued_assets(timestamp);
create index ia_height_index on issued_assets(height);

-- Delegation
create table delegations(
    tx varchar(64) not null,
    block varchar(64) not null,
    sender varchar(62) not null,
    amount bigint not null,
    validator varchar(40) not null,
    new_validator varchar(62),
    timestamp bigint not null,
    height bigint not null,
    content jsonb not null,
    primary key (tx)
);
create index dlg_sender_index on delegations(sender);
create index dlg_validator_index on delegations(validator);
create index dlg_time_index on delegations(timestamp);
create index dlg_height_index on delegations(height);

-- UnDelegation
create table undelegations(
    tx varchar(64) not null,
    block varchar(64) not null,
    sender varchar(62) not null,
    amount bigint not null,
    target_validator varchar(40) not null,
    new_delegator varchar(62),
    height bigint not null,
    timestamp bigint not null,
    content jsonb not null,
    primary key (tx)
);
create index ud_sender_index on undelegations(sender);
create index ud_validator_index on undelegations(target_validator);
create index ud_time_index on undelegations(timestamp);
create index ud_height_index on undelegations(height);

-- Claim
create table claims(
    tx varchar(64) not null,
    block varchar(64) not null,
    sender varchar(62) not null,
    amount bigint not null,
    height bigint not null,
    timestamp bigint not null,
    content jsonb not null,
    primary key (tx)
);
create index clm_sender_index on claims(sender);
create index clm_time_index on claims(timestamp);
create index clm_height_index on claims(height);

-- Native To EVM
create table n2e(
    tx varchar(64) not null,
    block varchar(64) not null,
    sender varchar(62) not null,
    receiver varchar(42) not null,
    asset varchar(44) not null,
    amount varchar(32) not null,
    height bigint not null,
    timestamp bigint not null,
    content jsonb not null,
    primary key (tx)
);
create index n2e_sender_index on n2e(sender);
create index n2e_receiver_index on n2e(receiver);
create index n2e_time_index on n2e(timestamp);
create index n2e_height_index on n2e(height);
