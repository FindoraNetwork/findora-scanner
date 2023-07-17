drop table native_txs;
drop table evm_txs;
drop table created_assets;
drop table issued_assets;
drop table stakings;
drop table unstakings;
drop table rewards;
drop table tx_types;
drop table n2e;

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
    sender varchar(62) not null,
    receiver varchar(62) not null,
    asset varchar(44) not null,
    amount varchar(32) not null,
    height bigint not null,
    timestamp bigint not null,
    primary key (tx,receiver,amount,asset)
);
create index nt_block_index on native_txs(block);
create index nt_sender_index on native_txs(sender);
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
    code varchar(44) not null,
    issuer varchar(62) not null,
    issued_at_tx varchar(64) not null,
    amount varchar(89) not null,
    timestamp bigint not null,
    height bigint not null,
    primary key (code)
);
create index ia_issuer_index on issued_assets(issuer);
create index ia_issued_at_index on issued_assets(issued_at_tx);
create index ia_time_index on issued_assets(timestamp);
create index ia_height_index on issued_assets(height);

-- Delegation
create table stakings(
    tx varchar(64) not null,
    block varchar(64) not null,
    sender varchar(62) not null,
    amount bigint not null,
    validator varchar(40) not null,
    new_validator varchar(62),
    timestamp bigint not null,
    height bigint not null,
    primary key (tx)
);
create index stk_sender_index on stakings(sender);
create index stk_validator_index on stakings(validator);
create index stk_time_index on stakings(timestamp);
create index stk_height_index on stakings(height);

-- UnDelegation
create table unstakings(
    tx varchar(64) not null,
    block varchar(64) not null,
    sender varchar(62) not null,
    amount bigint not null,
    target_validator varchar(40) not null,
    new_delegator varchar(62),
    height bigint not null,
    timestamp bigint not null,
    primary key (tx)
);
create index unstk_sender_index on unstakings(sender);
create index unstk_validator_index on unstakings(target_validator);
create index unstk_time_index on unstakings(timestamp);
create index unstk_height_index on unstakings(height);

-- Claim
create table rewards(
    tx varchar(64) not null,
    block varchar(64) not null,
    sender varchar(62) not null,
    amount bigint not null,
    height bigint not null,
    timestamp bigint not null,
    primary key (tx)
);
create index rd_sender_index on rewards(sender);
create index rd_time_index on rewards(timestamp);
create index rd_height_index on rewards(height);

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
    primary key (tx)
);
create index n2e_sender_index on n2e(sender);
create index n2e_receiver_index on n2e(receiver);
create index n2e_time_index on n2e(timestamp);
create index n2e_height_index on n2e(height);
