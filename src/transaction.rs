pub mod tx_out;
pub mod tx_in;
pub mod upspent_tx_out;
use sha256::digest;
use crate::transaction::{
    tx_in::TxIn,
    tx_out::TxOut,
    upspent_tx_out::UnpsentTxOut,
};
use crate::dsa::{
    public_key_from,
    sign,
};

pub struct Transaction {
    id: String,
    tx_in_list: Vec<TxIn>,
    tx_out_list: Vec<TxOut>,
}

impl Transaction {
    pub fn new(tx_in_list: Vec<TxIn>, tx_out_list: Vec<TxOut>) -> Self {
        let id = Self::calculate_transaction_id(tx_in_list.clone(), tx_out_list.clone());
        Transaction {
            id,
            tx_in_list,
            tx_out_list,
        }
    }

    pub fn transaction_id(&self) -> String {
        Self::calculate_transaction_id(self.tx_in_list.clone(), self.tx_out_list.clone())
    }

    pub fn calculate_transaction_id(tx_in_list: Vec<TxIn>, tx_out_list: Vec<TxOut>) -> String {
        let tx_in_content = tx_in_list.iter()
            .map(|tx_in| format!("{}{}", tx_in.tx_out_id(), tx_in.tx_out_index()))
            .reduce(|a, b| format!("{}{}", a, b))
            .unwrap();

        let tx_out_content = tx_out_list.iter()
            .map(|tx_out| format!("{}{}", tx_out.address(), tx_out.amount()))
            .reduce(|a, b| format!("{}{}", a, b))
            .unwrap();

        digest(format!("{}{}", tx_in_content, tx_out_content).to_string())
    }

    pub fn sign_tx_in(&self,
        tx_in_index: u32,
        private_key: String,
        unspent_tx_out_list: Vec<UnpsentTxOut>,
    ) -> Result<String, String> {
        let tx_in = self.tx_in_list[tx_in_index as usize].clone();
        let data_to_sign = self.id.clone();
        if let Some(referenced_unspent_tx_out) = unspent_tx_out_list
            .iter()
            .find(|unspent_tx_out| {
                unspent_tx_out.tx_out_id() == tx_in.tx_out_id()
                && unspent_tx_out.tx_out_index() == tx_in.tx_out_index()
            })
        {
            let referenced_address = referenced_unspent_tx_out.address();
            if referenced_address != public_key_from(private_key.clone()).unwrap_or("".to_string()) {
                return Err("Invalid private key".to_string())
            }
        } else {
            return Err("Could not referenced tx_out".to_string())
        }

        sign(private_key, data_to_sign)
    }
}
