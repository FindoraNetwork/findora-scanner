drop table native_txs;
drop table evm_txs;
drop table created_assets;
drop table issued_assets;
drop table stakings;
drop table unstakings;
drop table rewards;

-- Native Transfer
create table native_txs(
    tx varchar(64) not null,
    block varchar(64) not null,
    sender varchar(62) not null,
    receiver varchar(62) not null,
    amount bigint not null,
    height bigint not null,
    timestamp bigint not null,
    ty integer not null
);
create index nt_tx_index on native_txs(tx);
create index nt_block_index on native_txs(block);
create index nt_sender_index on native_txs(sender);
create index unt_receiver_index on native_txs(receiver);
create index nt_height_index on native_txs(height);
create index nt_time_index on native_txs(timestamp);

-- EVM Transfer
create table evm_txs(
    tx varchar(64) not null,
    block varchar(64) not null,
    evm_tx varchar(66) not null,
    sender varchar(42) not null,
    receiver varchar(42) not null,
    nonce bigint not null,
    amount bigint not null,
    gas_price bigint not null,
    gas_limit bigint not null,
    height bigint not null,
    timestamp bigint not null,
    ty integer
);
create index et_tx_index on evm_txs(tx);
create index et_block_index on evm_txs(block);
create index et_evm_tx_index on evm_txs(evm_tx);
create index et_sender_index on evm_txs(sender);
create index et_receiver_index on evm_txs(receiver);
create index et_height_index on evm_txs(height);
create index et_time_index on evm_txs(timestamp);

-- DefineAsset
create table created_assets(
    code varchar(44) not null,
    creator varchar(62) not null,
    created_at varchar(64) not null,
    decimal integer not null,
    max_units integer not null,
    transferable boolean not null,
    updatable boolean not null,
    transfer_multisig_rules jsonb
);
create index ca_code_index on created_assets(code);
create index ca_issuer_index on created_assets(creator);
create index ca_created_at_index on created_assets(created_at);

-- IssueAsset
create table issued_assets(
    asset_type varchar(44) not null,
    issuer varchar(62) not null,
    issued_at varchar(64) not null,
    amount bigint not null
);
create index ia_type_index on issued_assets(asset_type);
create index ia_issuer_index on issued_assets(issuer);
create index ia_issued_at_index on issued_assets(issued_at);

-- Delegation
create table stakings(
    tx varchar(64) not null,
    sender varchar(62) not null,
    amount bigint not null,
    target_validator varchar(40) not null,
    new_validator varchar(62)
);
create index stk_tx_index on stakings(tx);
create index stk_sender_index on stakings(sender);
create index stk_validator_index on stakings(target_validator);

-- UnDelegation
create table unstakings(
    tx varchar(64) not null,
    sender varchar(62) not null,
    amount bigint not null,
    target_validator varchar(40) not null,
    new_validator varchar(62)
);
create index unstk_tx_index on unstakings(tx);
create index unstk_sender_index on unstakings(sender);
create index unstk_validator_index on unstakings(target_validator);

-- Claim
create table rewards(
    tx varchar(64) not null,
    sender varchar(62) not null,
    amount bigint not null
);
create index rd_tx_index on rewards(tx);
create index rd_sender_index on rewards(sender);