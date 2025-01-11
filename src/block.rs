use std::time::{SystemTime, UNIX_EPOCH};
use sha256::digest;
use serde::Serialize;
use to_binary::BinaryString;
use crate::database::SQLiteBlock;

#[derive(Debug, Clone, Serialize)]
pub struct Block {
    index: u32,
    hash: String,
    previous_hash: String,
    timestamp: u32,
    data: String,
    difficulty: u32,
    nonce: u32,
}

impl Block {
    pub fn index(&self) -> u32 { self.index }
    pub fn hash(&self) -> String { self.hash.clone() }
    pub fn previous_hash(&self) -> String { self.previous_hash.clone() }
    pub fn timestamp(&self) -> u32 { self.timestamp }
    pub fn data(&self) -> String { self.data.clone() }
    pub fn difficulty(&self) -> u32 { self.difficulty }
    pub fn nonce(&self) -> u32 { self.nonce }

    fn new(
        index: u32,
        hash: Option<String>,
        previous_hash: String,
        timestamp: u32,
        data: String,
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
            index,
            hash,
            previous_hash,
            timestamp,
            data,
            difficulty,
            nonce,
        }
    }

    pub fn from_sqlite_block(block: SQLiteBlock) -> Self {
        Block {
            index: block.block_index as u32,
            hash: block.hash,
            previous_hash: block.previous_hash,
            timestamp: block.generate_timestamp as u32,
            data: block.data,
            difficulty: block.difficulty as u32,
            nonce: block.nonce as u32,
        }
    }

    pub fn get_genesis() -> Block {
        Block::new(
            0,
            Some("0abf7ee41adf312cc7c44707914ed44324ebca86bc6010668a91dcaa2b7a8b53".to_string()),
            "".to_string(),
            1736325055,
            "my genesis block".to_string(),
            4,
            22
        )
    }

    pub fn find_genesis_block(data: String, difficulty: u32) -> Self {
        let mut nonce: u32 = 0;
        loop {
            let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            let hash = Self::calculate_hash(
                0,
                "".to_string(),
                timestamp as u32,
                data.clone(),
                difficulty,
                nonce,
            );

            if Self::is_matches_difficulty_hash(hash.clone(), difficulty) {
                return Block::new(
                    0,
                    Some(hash),
                    "".to_string(),
                    timestamp as u32,
                    data,
                    difficulty,
                    nonce,
                )
            }

            nonce += 1;
        }
    }

    pub fn find_block(&self, data: String, difficulty: u32) -> Self {
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
        data: String,
        difficulty: u32,
        nonce: u32,
    ) -> String {
        digest(format!("{}{}{}{}{}{}",
            index,
            previous_hash,
            timestamp,
            data,
            difficulty,
            nonce,
        ).to_string())
    }
}
