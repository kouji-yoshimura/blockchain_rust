pub mod sqlite;
use crate::block_chain::BlockChain;
use crate::block::Block;
use sqlx;

#[allow(async_fn_in_trait)]
pub trait Database {
    async fn find_block_chain(&self) -> Result<BlockChain, sqlx::Error>;
    async fn save_block(&self, block: Block) -> Result<(), sqlx::Error>;
}
