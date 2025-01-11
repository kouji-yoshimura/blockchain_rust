use crate::transaction::unspent_tx_out::UnspentTxOut;
use crate::dsa::verify;

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

    pub fn is_valid(&self, transaction: super::Transaction, unspent_tx_out_list: Vec<UnspentTxOut>) -> Result<(), String> {
        let referenced_unspent_tx_out = match unspent_tx_out_list
            .iter()
            .find(|unspent_tx_out| {
                unspent_tx_out.tx_out_id() == self.tx_out_id()
                && unspent_tx_out.tx_out_index() == self.tx_out_index()
            })
        {
            Some(v) => v,
            None => return Err("Failed to find referenced UnspentTxOut".to_string()),
        };

        verify(referenced_unspent_tx_out.address(), transaction.id(), self.signature())
    }
}
