use serde::Serialize;
use crate::{
    block::Block,
    transaction::Transaction,
    database::sqlite::{
        SQLiteBlock,
        SQLiteTransaction,
        SQLiteTxIn,
        SQLiteTxOut,
    },
};

pub const BLOCK_GENERATION_INTERVAL: u32 = 10;
pub const DIFFICULTY_ADJUSTMENT_INTERVAL: u32 = 10;

#[derive(Clone, Serialize)]
pub struct BlockChain(Vec<Block>);

impl BlockChain {
    pub fn generate() -> Self {
        BlockChain(vec![Block::get_genesis()])
    }

    pub fn from_sqlite_blocks(
        sqlite_blocks: Vec<SQLiteBlock>,
        sqlite_transactions: Vec<SQLiteTransaction>,
        sqlite_tx_ins: Vec<SQLiteTxIn>,
        sqlite_tx_outs: Vec<SQLiteTxOut>,
    ) -> Result<Self, &'static str> {
        let mut block_chain = BlockChain(vec![]);
        block_chain.0 = sqlite_blocks
            .iter()
            .map(|b| {
                Block::from_sqlite_block(
                    b.clone(),
                    sqlite_transactions.clone(),
                    sqlite_tx_ins.clone(),
                    sqlite_tx_outs.clone(),
                )
            })
            .collect();

        match block_chain.is_valid_chain() {
            Ok(()) => Ok(block_chain),
            Err(message) => Err(message),
        }
    }

    pub fn latest_block(&self) -> Block {
        self.0[self.0.len() - 1].clone()
    }

    pub fn is_valid_chain(&self) -> Result<(), &'static str> {
        if self.0[0].hash() != Block::get_genesis().hash() {
            return Err("Invalid genesis block")
        }
        for index in 1..self.0.len() {
            match self.0[index - 1].is_valid_next_block(self.0[index].clone()) {
                Ok(()) => {},
                Err(message) => {
                    return Err(message)
                }
            };
        }
        Ok(())
    }

    pub fn append_new_block(&mut self, data: Vec<Transaction>) -> Block {
        let new_block = self.latest_block().find_block(data, self.adjusted_difficulty());
        self.0.push(new_block.clone());

        new_block
    }

    fn adjusted_difficulty(&self) -> u32 {
        let latest_block = self.latest_block();
        if latest_block.index() % DIFFICULTY_ADJUSTMENT_INTERVAL != 0 || latest_block.index() == 0 {
            return latest_block.difficulty()
        }

        let previous_adjustment_block = self.0[self.0.len() - DIFFICULTY_ADJUSTMENT_INTERVAL as usize].clone();
        let time_expected = BLOCK_GENERATION_INTERVAL * DIFFICULTY_ADJUSTMENT_INTERVAL;
        let time_taken = latest_block.timestamp() - previous_adjustment_block.timestamp();

        if time_taken < time_expected / 2 {
            return previous_adjustment_block.difficulty() + 1
        } else if time_taken > time_expected * 2 {
            return previous_adjustment_block.difficulty() - 1
        } else {
            return previous_adjustment_block.difficulty()
        }
    }

    pub fn replace_chain(&mut self, new_chain: BlockChain) -> Result<(), String>{
        if new_chain.is_valid_chain().is_err() || new_chain.0.len() <= self.0.len() {
            return Err("Invalid chain received".to_string())
        }

        self.0 = new_chain.0;
        return Ok(())
    }
}
