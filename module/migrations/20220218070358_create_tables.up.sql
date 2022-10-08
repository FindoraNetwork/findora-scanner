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
     result JSONB NOT NULL,
     value JSONB NOT NULL,
     PRIMARY KEY(tx_hash)
);

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
