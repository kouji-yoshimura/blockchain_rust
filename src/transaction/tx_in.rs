use std::fmt;
use serde::Serialize;
use crate::transaction::utxo::UTxO;
use crate::mylib::ecdsa::verify;

#[derive(Clone, Serialize)]
pub struct TxIn {
    tx_out_id: String,
    tx_out_index: usize,
    signature: String,
}

impl TxIn {
    pub fn tx_out_id(&self) -> String { self.tx_out_id.clone() }
    pub fn tx_out_index(&self) -> usize { self.tx_out_index }
    pub fn signature(&self) -> String { self.signature.clone() }

    pub fn new(tx_out_id: String, tx_out_index: usize, signature: String) -> Self {
        TxIn { tx_out_id, tx_out_index, signature }
    }

    pub fn is_valid(&self, transaction: super::Transaction, utxo_list: Vec<UTxO>) -> Result<(), String> {
        let referenced_unspent_tx_out = match utxo_list
            .iter()
            .find(|utxo| {
                utxo.tx_out_id() == self.tx_out_id()
                && utxo.tx_out_index() == self.tx_out_index()
            })
        {
            Some(v) => v,
            None => return Err("Failed to find referenced UTxO".to_string()),
        };

        verify(referenced_unspent_tx_out.address(), transaction.id(), self.signature())
    }
}

impl fmt::Display for TxIn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TxIn{{tx_out_id:{},tx_out_index:{},signature:{}}}", self.tx_out_id(), self.tx_out_index(), self.signature())
    }
}
