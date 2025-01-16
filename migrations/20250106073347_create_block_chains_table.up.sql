CREATE TABLE blocks (
  block_id CHAR(36) PRIMARY KEY,
  block_chain_id CHAR(36) NOT NULL,
  block_index INTEGER NOT NULL,
  hash CHAR(64) NOT NULL,
  previous_hash CHAR(64) NOT NULL,
  generate_timestamp INTEGER NOT NULL,
  difficulty INTEGER NOT NULL,
  nonce INTEGER NOT NULL
);

CREATE TABLE transactions (
  transaction_id CHAR(64) PRIMARY KEY,
  block_id CHAR(36) NOT NULL
);

CREATE TABLE tx_ins (
  tx_in_id CHAR(36) PRIMARY KEY,
  transaction_id CHAR(64) NOT NULL,
  tx_out_id CHAR(64) NOT NULL,
  tx_out_index INTEGER NOT NULL,
  signature CHAR(64) NOT NULL
);

CREATE TABLE tx_outs (
  tx_out_id CHAR(36) PRIMARY KEY,
  transaction_id CHAR(64) NOT NULL,
  address CHAR(64) NOT NULL,
  amount INTEGER NOT NULL
);

