#[derive(Clone)]
pub struct TxIn {
    tx_out_id: String,
    tx_out_index: u32,
    signature: String,
}

impl TxIn {
    pub fn tx_out_id(&self) -> String { self.tx_out_id.clone() }
    pub fn tx_out_index(&self) -> u32 { self.tx_out_index }
    pub fn signature(&self) -> String { self.signature.clone() }

    pub fn new(tx_out_id: String, tx_out_index: u32, signature: String) -> Self {
        TxIn { tx_out_id, tx_out_index, signature }
    }
}
