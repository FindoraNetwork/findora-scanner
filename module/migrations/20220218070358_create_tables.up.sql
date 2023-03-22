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

CREATE TABLE e2n (
    tx_hash VARCHAR(255) NOT NULL,
    block_hash VARCHAR(255) NOT NULL,
    sender VARCHAR(255) NOT NULL,
    receiver VARCHAR(255) NOT NULL,
    asset VARCHAR(255) NOT NULL,
    amount VARCHAR(255) NOT NULL,
    height BIGINT NOT NULL,
    timestamp BIGINT NOT NULL,
    value JSONB NOT NULL,
    Primary Key (tx_hash)
);

CREATE INDEX e2n_sender_index ON e2n (sender);
CREATE INDEX e2n_receiver_index ON e2n (receiver);

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

CREATE TABLE delegations (
    height BIGINT NOT NULL PRIMARY KEY,
    info JSONB
);

CREATE INDEX block_height_index ON block (height);
CREATE INDEX block_time_index ON block (time);
CREATE INDEX block_proposer_index ON block (proposer);
CREATE INDEX tx_block_index ON transaction (block_hash);
CREATE INDEX tx_height_index ON transaction (height);
CREATE INDEX tx_timestamp_index ON transaction (timestamp);
CREATE INDEX tx_ty_index ON transaction (ty);
CREATE INDEX bg_height_address_index ON block_generation(height, address);
CREATE INDEX bg_signature_index ON block_generation(signature);
CREATE INDEX bg_time_index ON block_generation(time);
