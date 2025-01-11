#[derive(Clone)]
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

