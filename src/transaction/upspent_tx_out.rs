pub struct UnpsentTxOut {
    tx_out_id: String,
    tx_out_index: u32,
    address: String,
    amount: u32,
}

impl UnpsentTxOut {
    pub fn tx_out_id(&self) -> String { self.tx_out_id.clone() }
    pub fn tx_out_index(&self) -> u32 { self.tx_out_index }
    pub fn address(&self) -> String { self.address.clone() }
    pub fn amount(&self) -> u32 { self.amount }

    pub fn new(
        tx_out_id: String,
        tx_out_index: u32,
        address: String,
        amount: u32,
    ) -> Self {
        UnpsentTxOut {
            tx_out_id,
            tx_out_index,
            address,
            amount,
        }
    }
}
