use sqlx::sqlite::SqlitePool;
use uuid7;
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
            SQLiteBlock,
            r#"
                SELECT
                    block_id,
                    block_chain_id,
                    block_index,
                    hash,
                    previous_hash,
                    generate_timestamp,
                    difficulty,
                    nonce
                  FROM blocks
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let transactions = sqlx::query_as!(
            SQLiteTransaction,
            r#"
                SELECT
                    transaction_id,
                    block_id
                  FROM transactions
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let tx_ins = sqlx::query_as!(
            SQLiteTxIn,
            r#"
                SELECT
                    tx_in_id,
                    transaction_id,
                    tx_out_id,
                    tx_out_index,
                    signature
                  FROM tx_ins
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let tx_outs = sqlx::query_as!(
            SQLiteTxOut,
            r#"
                SELECT
                    tx_out_id,
                    transaction_id,
                    address,
                    amount
                  FROM tx_outs
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let block_chain = BlockChain::from_sqlite_blocks(
            blocks,
            transactions,
            tx_ins,
            tx_outs,
        ).unwrap();

        Ok(block_chain)
    }

    async fn save_block(&self, block: Block) -> Result<(), sqlx::Error> {
        let block_id = block.block_id();
        let block_chain_id = block.block_chain_id();
        let index = block.index() as i64;
        let hash = block.hash();
        let previous_hash = block.previous_hash();
        let timestamp = block.timestamp() as i64;
        let difficulty = block.difficulty() as i64;
        let nonce = block.nonce() as i64;
        sqlx::query!(
            r#"
                INSERT INTO blocks (
                    block_id,
                    block_chain_id,
                    block_index,
                    hash,
                    previous_hash,
                    generate_timestamp,
                    difficulty,
                    nonce
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            block_id,
            block_chain_id,
            index,
            hash,
            previous_hash,
            timestamp,
            difficulty,
            nonce,
        )
        .execute(&self.pool)
        .await?;

        for transaction in block.data().iter() {
            let transaction_id = transaction.id();
            sqlx::query!(
                r#"
                    INSERT INTO transactions(
                        transaction_id,
                        block_id
                    ) VALUES (?, ?)
                "#,
                transaction_id,
                block_id,
            )
            .execute(&self.pool)
            .await?;

            for tx_in in transaction.tx_in_list().iter() {
                let tx_in_id = uuid7::uuid7().to_string(); // FIXME
                let tx_out_id = tx_in.tx_out_id();
                let tx_out_index = tx_in.tx_out_index() as i64;
                let signature = tx_in.signature();
                sqlx::query!(
                    r#"
                        INSERT INTO tx_ins(
                            tx_in_id,
                            transaction_id,
                            tx_out_id,
                            tx_out_index,
                            signature
                        ) VALUES (?, ?, ?, ?, ?)
                    "#,
                    tx_in_id,
                    transaction_id,
                    tx_out_id,
                    tx_out_index,
                    signature,
                )
                .execute(&self.pool)
                .await?;
            }
            for tx_out in transaction.tx_out_list().iter() {
                let tx_out_id = uuid7::uuid7().to_string(); // FIXME
                let address = tx_out.address();
                let amount = tx_out.amount();
                sqlx::query!(
                    r#"
                        INSERT INTO tx_outs(
                            tx_out_id,
                            transaction_id,
                            address,
                            amount
                        ) VALUES (?, ?, ?, ?)
                    "#,
                    tx_out_id,
                    transaction_id,
                    address,
                    amount,
                )
                .execute(&self.pool)
                .await?;
            }
        }

        // let block = SQLiteBlock {
        //     block_id: block.block_id(),
        //     block_chain_id: block.block_chain_id(),
        //     block_index: block.index() as i64,
        //     hash: block.hash(),
        //     previous_hash: block.previous_hash(),
        //     generate_timestamp: block.timestamp() as i64,
        //     data: block.data().iter().map(|transaction| {
        //         SQLiteTransaction {
        //             transaction_id: transaction.transaction_id(),
        //             // block_id: transaction.block_id() as i64,
        //             block_id: block.block_id(),
        //             tx_ins: transaction.tx_in_list().iter().map(|tx_in| {
        //                 SQLiteTxIn {
        //                     tx_in_id: "".to_string(),
        //                     transaction_id: transaction.transaction_id(),
        //                     tx_out_id: tx_in.tx_out_id(),
        //                     tx_out_index: tx_in.tx_out_index() as i64,
        //                     signature: tx_in.signature(),
        //                 }
        //             }).collect(),
        //             tx_outs: transaction.tx_out_list().iter().map(|tx_out| {
        //                 SQLiteTxOut {
        //                     tx_out_id: "".to_string(),
        //                     transaction_id: transaction.transaction_id(),
        //                     address: tx_out.address(),
        //                     amount: tx_out.amount() as i64,
        //                 }
        //             }).collect(),
        //         }
        //     }).collect(),
        //     difficulty: block.difficulty() as i64,
        //     nonce: block.nonce() as i64,
        // };

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct SQLiteBlock {
    pub block_id: Option<String>,
    pub block_chain_id: String,
    pub block_index: i64,
    pub hash: String,
    pub previous_hash: String,
    pub generate_timestamp: i64,
    pub difficulty: i64,
    pub nonce: i64,
}

#[derive(Clone, Debug)]
pub struct SQLiteTransaction {
    pub transaction_id: Option<String>,
    pub block_id: String,
}

#[derive(Clone, Debug)]
pub struct SQLiteTxIn {
    pub tx_in_id: Option<String>,
    pub transaction_id: String,
    pub tx_out_id: String,
    pub tx_out_index: i64,
    pub signature: String,
}

#[derive(Clone, Debug)]
pub struct SQLiteTxOut {
    pub tx_out_id: Option<String>,
    pub transaction_id: String,
    pub address: String,
    pub amount: i64,
}

