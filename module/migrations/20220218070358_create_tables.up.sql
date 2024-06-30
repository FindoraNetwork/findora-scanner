create table block (
    block_hash varchar(64) not null,
    height bigint not null,
    size bigint not null,
    tx_count bigint not null,
    time timestamp not null,
    app_hash varchar(64) not null,
    proposer varchar(64) not null,
    block_data jsonb not null,
    primary key (block_hash)
);
create index block_height_index on block (height);
create index blk_time_index on block(time);
create index block_proposer_index on block (proposer);

create table transaction (
     tx_hash varchar(64) not null,
     block_hash varchar(64) not null,
     height bigint not null,
     timestamp bigint not null,
     code bigint not null,
     ty integer not null,
     ty_sub integer not null,
     sender varchar(64) not null,
     receiver jsonb not null,
     log text,
     origin text not null,
     result jsonb not null,
     value jsonb not null,
     primary key (tx_hash)
);
create index tx_block_index on transaction (block_hash);
create index tx_height_index on transaction (height);
create index tx_time_index on transaction (timestamp);
create index tx_sender_index on transaction(sender);

create table e2n (
    tx_hash varchar(64) not null,
    block_hash varchar(64) not null,
    sender varchar(64) not null,
    receiver varchar(64) not null,
    asset varchar(64) not null,
    amount varchar(48) not null,
    decimal integer not null,
    height bigint not null,
    timestamp bigint not null,
    value jsonb not null,
    Primary Key (tx_hash)
);
create index e2n_sender_index on e2n (sender);
create index e2n_receiver_index on e2n (receiver);
create index e2n_tx_hash_index on e2n(tx_hash);
create index e2n_block_hash_index on e2n(block_hash);
create index e2n_height_index on e2n(height);

create table validators (
    address varchar(64) not null,
    pubkey_type int not null,
    pubkey varchar(64) not null,
    primary key (address)
);

create table block_generation (
    height bigint not null,
    address varchar(64) not null,
    power bigint not null,
    priority bigint,
    signature varchar(128),
    time timestamp,
    primary key (height, address)
);
create index bg_height_addr_index on block_generation(height, address);
create index bg_sig_index on block_generation(signature);

create table last_height (
    tip varchar(8) not null,
    height bigint not null,
    primary key(tip)
);

create table e2n_last_height (
    tip varchar(8) not null,
    height bigint not null,
    primary key (tip)
);

create table native_addrs(
    id bigserial primary key,
    tx varchar(64) not null,
    address varchar(64) not null,
    timestamp bigint not null
);
create index idx_ntvaddr on native_addrs(address);

create table evm_addrs(
    id bigserial primary key,
    tx varchar(64) not null,
    address varchar(64) not null,
    timestamp bigint not null
);
create index idx_evmaddr on evm_addrs(address);

create table assets(
    asset varchar(64) not null,
    tx varchar(64) not null,
    block varchar(64) not null,
    issuer varchar(64) not null,
    height bigint not null,
    timestamp bigint not null,
    ty integer not null,
    content jsonb not null,
    primary key (asset,tx,ty)
);
create index ast_blk_index on assets(block);
create index ast_issuer_index on assets(issuer);
create index ast_height_index on assets(height);
create index ast_tm_index on assets(timestamp);

create table delegations(
    tx varchar(64) not null,
    block varchar(64) not null,
    sender varchar(64) not null,
    amount bigint not null,
    validator varchar(64) not null,
    new_validator varchar(64),
    timestamp bigint not null,
    height bigint not null,
    content jsonb not null,
    primary key (tx)
);
create index dlg_sender_index on delegations(sender);
create index dlg_validator_index on delegations(validator);
create index dlg_time_index on delegations(timestamp);
create index dlg_height_index on delegations(height);


create table undelegations(
    tx varchar(64) not null,
    block varchar(64) not null,
    sender varchar(64) not null,
    amount bigint not null,
    target_validator varchar(64) not null,
    new_delegator varchar(64),
    height bigint not null,
    timestamp bigint not null,
    content jsonb not null,
    primary key (tx)
);
create index ud_sender_index on undelegations(sender);
create index ud_validator_index on undelegations(target_validator);
create index ud_time_index on undelegations(timestamp);
create index ud_height_index on undelegations(height);

create table claims(
    tx varchar(64) not null,
    block varchar(64) not null,
    sender varchar(64) not null,
    amount bigint not null,
    height bigint not null,
    timestamp bigint not null,
    content jsonb not null,
    primary key (tx)
);
create index clm_sender_index on claims(sender);
create index clm_time_index on claims(timestamp);
create index clm_height_index on claims(height);

create table n2e(
    tx varchar(64) not null,
    block varchar(64) not null,
    sender varchar(64) not null,
    receiver varchar(64) not null,
    asset varchar(64) not null,
    amount varchar(48) not null,
    height bigint not null,
    timestamp bigint not null,
    content jsonb not null,
    primary key (tx)
);
create index n2e_sender_index on n2e(sender);
create index n2e_receiver_index on n2e(receiver);
create index n2e_time_index on n2e(timestamp);
create index n2e_height_index on n2e(height);

create table prices(
    name varchar(8) not null,
    price varchar(16) not null,
    primary key (name)
);

create table market(
    name varchar(8) not null,
    val jsonb not null,
    primary key (name)
);
