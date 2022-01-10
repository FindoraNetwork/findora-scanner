CREATE TABLE block (
    block_id VARCHAR(255) NOT NULL,
    height BIGINT NOT NULL,
    time TIMESTAMP NOT NULL,
    app_hash VARCHAR(255) NOT NULL,
    proposer VARCHAR(255) NOT NULL,
    PRIMARY KEY(height)
);

CREATE TABLE transaction(
    txid VARCHAR(255) NOT NULL,
    ty INT NOT NULL,
    value JSONB NOT NULL,
    code INT NOT NULL,
    log TEXT,
    PRIMARY KEY(txid)
);

CREATE TABLE validators(
    address VARCHAR(255) NOT NULL,
    pubkey_type INT NOT NULL,
    pubkey VARCHAR(255) NOT NULL,
    PRIMARY KEY(address)
);

CREATE TABLE block_generation(
    height BIGINT NOT NULL,
    address VARCHAR(255) NOT NULL,
    power BIGINT NOT NULL,
    priority BIGINT,
    signature VARCHAR(255),
    time TIMESTAMP,
    PRIMARY KEY(height, address)
);

