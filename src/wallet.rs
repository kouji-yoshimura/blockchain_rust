use std::fs;
use crate::mylib::ecdsa::{
    generate_private_key,
    public_key_from_private_key,
};

const PRIVATE_KEY_PATH: &str= "wallet/private_key";

pub struct Wallet;

impl Wallet {
    pub fn initialize() -> Result<(), String> {
        match fs::exists(PRIVATE_KEY_PATH) {
            Err(e) => return Err(e.to_string()),
            Ok(is_exists) => {
                if is_exists {
                    return Ok(())
                }
            }
        };

        if let Err(e) = fs::write(PRIVATE_KEY_PATH, generate_private_key()) {
            return Err(e.to_string())
        };

        Ok(())
    }

    pub fn public_key() -> Result<String, String> {
        let private_key = match fs::read_to_string(PRIVATE_KEY_PATH) {
            Ok(v) => v,
            Err(e) => return Err(e.to_string()),
        };
        public_key_from_private_key(private_key)
    }
}
