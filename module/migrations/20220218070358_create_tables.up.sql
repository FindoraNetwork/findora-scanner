CREATE TABLE block (
    block_hash VARCHAR(255) NOT NULL,
    height BIGINT NOT NULL,
    size BIGINT NOT NULL,
    tx_count BIGINT NOT NULL,
    time TIMESTAMP NOT NULL,
    app_hash VARCHAR(255) NOT NULL,
    proposer VARCHAR(255) NOT NULL,
    block_data JSONB NOT NULL,
    PRIMARY KEY(height)
);

CREATE TABLE transaction (
    txid VARCHAR(255) NOT NULL,
    block_id VARCHAR(255) NOT NULL,
    ty INT NOT NULL,
    timestamp BIGINT NOT NULL,
    value JSONB NOT NULL,
    code BIGINT NOT NULL,
    log TEXT,
    PRIMARY KEY(txid)
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
