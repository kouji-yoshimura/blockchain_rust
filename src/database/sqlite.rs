use sqlx::sqlite::SqlitePool;
use crate::block_chain::BlockChain;
use crate::block::Block;

#[derive(Clone)]
pub struct SqliteDatabase {
    pool: SqlitePool,
}

impl SqliteDatabase {
    pub fn new(pool: SqlitePool) -> Self {
        SqliteDatabase {
            pool,
        }
    }
}

impl super::Database for SqliteDatabase {
    async fn find_block_chain(&self) -> Result<BlockChain, sqlx::Error> {
        let blocks = sqlx::query_as!(
            super::SQLiteBlock,
            r#"
                SELECT
                    block_index,
                    hash,
                    previous_hash,
                    generate_timestamp,
                    data,
                    difficulty,
                    nonce
                  FROM blocks
                  ORDER BY block_index ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let block_chain = BlockChain::from_sqlite_blocks(blocks).unwrap();
        Ok(block_chain)
    }

    async fn save_block(&self, block: Block) -> Result<(), sqlx::Error> {
        let index = block.index() as i64;
        let hash = block.hash();
        let previous_hash = block.previous_hash();
        let timestamp = block.timestamp() as i64;
        let data = block.data();
        let difficulty = block.difficulty();
        let nonce = block.nonce();
        sqlx::query!(
            r#"
                INSERT INTO blocks (
                    block_index,
                    hash,
                    previous_hash,
                    generate_timestamp,
                    data,
                    difficulty,
                    nonce
                ) VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
            index,
            hash,
            previous_hash,
            timestamp,
            data,
            difficulty,
            nonce,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
