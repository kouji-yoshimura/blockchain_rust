use std::time::{SystemTime, UNIX_EPOCH};
use sha256::digest;
use serde::Serialize;
use to_binary::BinaryString;
use uuid7;
use crate::{
    database::sqlite::{
        SQLiteBlock,
        SQLiteTransaction,
        SQLiteTxIn,
        SQLiteTxOut,
    },
    transaction::{
        Transaction,
        tx_in::TxIn,
        tx_out::TxOut,
    },
};

#[derive(Clone, Serialize)]
pub struct Block {
    block_id: String,
    block_chain_id: String,
    index: u32,
    hash: String,
    previous_hash: String,
    timestamp: u32,
    data: Vec<Transaction>,
    difficulty: u32,
    nonce: u32,
}

impl Block {
    pub fn block_id(&self) -> String { self.block_id.clone() }
    pub fn block_chain_id(&self) -> String { self.block_chain_id.clone() }
    pub fn index(&self) -> u32 { self.index }
    pub fn hash(&self) -> String { self.hash.clone() }
    pub fn previous_hash(&self) -> String { self.previous_hash.clone() }
    pub fn timestamp(&self) -> u32 { self.timestamp }
    pub fn data(&self) -> Vec<Transaction> { self.data.clone() }
    pub fn difficulty(&self) -> u32 { self.difficulty }
    pub fn nonce(&self) -> u32 { self.nonce }

    fn new(
        block_id: String,
        block_chain_id: String,
        index: u32,
        hash: Option<String>,
        previous_hash: String,
        timestamp: u32,
        data: Vec<Transaction>,
        difficulty: u32,
        nonce: u32,
    ) -> Self {
        let hash = hash.unwrap_or(
            Self::calculate_hash(
                index,
                previous_hash.clone(),
                timestamp,
                data.clone(),
                difficulty,
                nonce,
            )
        );
        Block {
            block_id,
            block_chain_id,
            index,
            hash,
            previous_hash,
            timestamp,
            data,
            difficulty,
            nonce,
        }
    }

    pub fn from_sqlite_block(
        sqlite_block: SQLiteBlock,
        sqlite_transactions: Vec<SQLiteTransaction>,
        sqlite_tx_ins: Vec<SQLiteTxIn>,
        sqlite_tx_outs: Vec<SQLiteTxOut>,
    ) -> Self {
        Block {
            block_id: sqlite_block.block_id.unwrap(),
            block_chain_id: sqlite_block.block_chain_id,
            index: sqlite_block.block_index as u32,
            hash: sqlite_block.hash,
            previous_hash: sqlite_block.previous_hash,
            timestamp: sqlite_block.generate_timestamp as u32,
            data: sqlite_transactions.iter()
                .map(|transaction| {
                    Transaction::new(
                        transaction.transaction_id.clone(),
                        sqlite_tx_ins.iter()
                            .filter(|tx_in| {
                                tx_in.transaction_id == transaction.transaction_id.clone().unwrap()
                            })
                            .map(|tx_in| {
                                TxIn::new(
                                    tx_in.tx_out_id.clone(),
                                    tx_in.tx_out_index as usize,
                                    tx_in.signature.clone(),
                                )
                            })
                            .collect(),
                        sqlite_tx_outs.iter()
                            .filter(|tx_out| {
                                tx_out.transaction_id == transaction.transaction_id.clone().unwrap()
                            })
                            .map(|tx_out| {
                                TxOut::new(
                                    tx_out.address.clone(),
                                    tx_out.amount as u32,
                                )
                            })
                            .collect(),
                    )
                })
                .collect(),
            difficulty: sqlite_block.difficulty as u32,
            nonce: sqlite_block.nonce as u32,
        }
        // Block {
        //     block_id: block.block_id.unwrap(),
        //     block_chain_id: block.block_chain_id,
        //     index: block.block_index as u32,
        //     hash: block.hash,
        //     previous_hash: block.previous_hash,
        //     timestamp: block.generate_timestamp as u32,
        //     data: vec!(),
        //     difficulty: block.difficulty as u32,
        //     nonce: block.nonce as u32,
        // }
    }

    pub fn get_genesis() -> Block {
        Block::new(
            "83e97ead-2fad-4efa-84da-9f51cac0cd82".to_string(),
            "53fc3be4-7143-490e-bf26-f61027a04f48".to_string(),
            0,
            Some("0abf7ee41adf312cc7c44707914ed44324ebca86bc6010668a91dcaa2b7a8b53".to_string()),
            "".to_string(),
            1736325055,
            vec!(),
            4,
            22
        )
    }

    // pub fn find_genesis_block(data: Vec<Transaction>, difficulty: u32) -> Self {
    //     let mut nonce: u32 = 0;
    //     loop {
    //         let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    //         let hash = Self::calculate_hash(
    //             0,
    //             "".to_string(),
    //             timestamp as u32,
    //             data.clone(),
    //             difficulty,
    //             nonce,
    //         );
    //
    //         if Self::is_matches_difficulty_hash(hash.clone(), difficulty) {
    //             return Block::new(
    //                 0,
    //                 Some(hash),
    //                 "".to_string(),
    //                 timestamp as u32,
    //                 data,
    //                 difficulty,
    //                 nonce,
    //             )
    //         }
    //
    //         nonce += 1;
    //     }
    // }

    pub fn find_block(&self, data: Vec<Transaction>, difficulty: u32) -> Self {
        let mut nonce: u32 = 0;
        loop {
            let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            let hash = Self::calculate_hash(
                self.index + 1,
                self.hash.clone(),
                timestamp as u32,
                data.clone(),
                difficulty,
                nonce,
            );

            if Self::is_matches_difficulty_hash(hash.clone(), difficulty) {
                return Block::new(
                    uuid7::uuid7().to_string(),
                    self.block_chain_id(),
                    self.index + 1,
                    Some(hash),
                    self.hash.clone(),
                    timestamp as u32,
                    data,
                    difficulty,
                    nonce,
                )
            }

            nonce += 1;
        }
    }

    fn is_matches_difficulty_hash(hash: String, difficulty: u32) -> bool {
        let hex = BinaryString::from_hex(hash.as_str()).unwrap();
        for (i, char) in hex.0.chars().enumerate() {
            if i + 1 > difficulty as usize {
                break;
            }
            if char != '0' {
                return false
            }
        }
        return true
    }

    pub fn is_valid_next_block(&self, next_block: Block) -> Result<(), &'static str> {
        if self.index() + 1 != next_block.index() {
            return Err("Invalid index")
        }
        if self.hash() != next_block.previous_hash() {
            return Err("Invalid previous hash")
        }
        if next_block.calculate_hash_for_block() != next_block.hash() {
            return Err("Invalid hash")
        }
        if self.timestamp() - 60 > next_block.timestamp() || next_block.timestamp() - 60 > SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u32 {
            return Err("Invalid timestamp")
        }
        Ok(())
    }

    fn calculate_hash_for_block(&self) -> String {
        Self::calculate_hash(
            self.index,
            self.previous_hash.clone(),
            self.timestamp,
            self.data.clone(),
            self.difficulty,
            self.nonce,
        )
    }

    fn calculate_hash(
        index: u32,
        previous_hash: String,
        timestamp: u32,
        data: Vec<Transaction>,
        difficulty: u32,
        nonce: u32,
    ) -> String {
        digest(format!("{}{}{}{}{}{}",
            index,
            previous_hash,
            timestamp,
            data.iter()
                .map(|v| format!("{}", v))
                .reduce(|a, b| format!("{}{}", a, b))
                .unwrap(),
            difficulty,
            nonce,
        ).to_string())
    }
}
