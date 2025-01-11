CREATE TABLE blocks (
  block_index INTEGER PRIMARY KEY,
  hash CHAR(64) NOT NULL,
  previous_hash CHAR(64) NOT NULL,
  generate_timestamp INTEGER NOT NULL,
  data TXT NOT NULL,
  difficulty INTEGER NOT NULL,
  nonce INTEGER NOT NULL
);
