pub mod sqlite;
use crate::block_chain::BlockChain;
use crate::block::Block as BlockEntity;
use sqlx;

#[allow(async_fn_in_trait)]
pub trait Database {
    async fn find_block_chain(&self) -> Result<BlockChain, sqlx::Error>;
    async fn save_block(&self, block: BlockEntity) -> Result<(), sqlx::Error>;
}

#[derive(Clone, Debug)]
pub struct SQLiteBlock {
    pub block_index: i64,
    pub hash: String,
    pub previous_hash: String,
    pub generate_timestamp: i64,
    pub data: String,
    pub difficulty: i64,
    pub nonce: i64,
}

