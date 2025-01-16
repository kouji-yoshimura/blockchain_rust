use std::fmt;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct TxOut {
    address: String,
    amount: u32,
}

impl TxOut {
    pub fn address(&self) -> String { self.address.clone() }
    pub fn amount(&self) -> u32 { self.amount }

    pub fn new(address: String, amount: u32) -> Self {
        TxOut { address, amount }
    }
}

impl fmt::Display for TxOut {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TxOut{{address:{},amount:{}}}",
            self.address(),
            self.amount(),
        )
    }
}
