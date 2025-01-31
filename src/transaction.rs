pub mod tx_out;
pub mod tx_in;
pub mod utxo;
use std::fmt;
use serde::Serialize;
use sha256::digest;
use crate::transaction::{
    tx_in::TxIn,
    tx_out::TxOut,
    utxo::UTxO,
};
use crate::mylib::ecdsa::{
    public_key_from_private_key,
    sign,
};

const COINBASE_AMOUNT: u32 = 50;

#[derive(Clone, Serialize)]
pub struct Transaction {
    id: String,
    tx_in_list: Vec<TxIn>,
    tx_out_list: Vec<TxOut>,
}

impl Transaction {
    pub fn id(&self) -> String { self.id.clone() }
    pub fn tx_in_list(&self) -> Vec<TxIn> { self.tx_in_list.clone() }
    pub fn tx_out_list(&self) -> Vec<TxOut> { self.tx_out_list.clone() }

    pub fn new(id: Option<String>, tx_in_list: Vec<TxIn>, tx_out_list: Vec<TxOut>) -> Self {
        let id = id.unwrap_or(Self::calculate_transaction_id(tx_in_list.clone(), tx_out_list.clone()));
        Transaction {
            id,
            tx_in_list,
            tx_out_list,
        }
    }

    pub fn is_valid(&self) -> Result<(), &'static str> {
        if self.id() != self.transaction_id() {
            return Err("Invalid transaction id")
        }

        Ok(())
    }

    pub fn check_amount_between_in_and_out(&self, utxo_list: Vec<UTxO>) -> Result<(), &'static str> {
        let total_tx_in_amount = self.tx_in_list
            .iter()
            .map(|tx_in| {
                match utxo_list.iter().find(|utxo| {
                    utxo.tx_out_id() == tx_in.tx_out_id()
                    && utxo.tx_out_index() == tx_in.tx_out_index()
                }) {
                    Some(tx_out) => tx_out.amount(),
                    None => 0,
                }
            })
            .reduce(|a, b| a + b);

        let total_tx_out_amount = self.tx_out_list
            .iter()
            .map(|tx_out| tx_out.amount())
            .reduce(|a, b| a + b);

        if total_tx_in_amount != total_tx_out_amount {
            return Err("Total values of TxIn and TxOut do not match")
        }

        Ok(())
    }

    pub fn is_coinbase(&self, block_index: usize) -> Result<(), &'static str> {
        if self.tx_in_list().len() != 1 {
            return Err("One TxIn must be specified in the coinbase transaction")
        }
        if self.tx_in_list()[0].tx_out_index() != block_index {
            return Err("The TxIn index in coinbase tx must be the block height")
        }
        if self.tx_out_list().len() != 1 {
            return Err("Invalid number of TxOutList in coinbase transaction")
        }
        if self.tx_out_list()[0].amount() != COINBASE_AMOUNT {
            return Err("Invalid coinbase amount in coinbase transaction")
        }
        Ok(())
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
        utxo_list: Vec<UTxO>,
    ) -> Result<String, String> {
        let tx_in = self.tx_in_list[tx_in_index as usize].clone();
        let data_to_sign = self.id.clone();

        let referenced_utxo = match utxo_list
            .iter()
            .find(|utxo| {
                utxo.tx_out_id() == tx_in.tx_out_id()
                && utxo.tx_out_index() == tx_in.tx_out_index()
            })
        {
            Some(v) => v,
            None => return Err("Could not referenced tx_out".to_string())
        };

        let referenced_address = referenced_utxo.address();
        if referenced_address != public_key_from_private_key(private_key.clone()).unwrap_or("".to_string()) {
            return Err("Invalid private key".to_string())
        }

        sign(private_key, data_to_sign)
    }

    pub fn new_utxo(&self) -> Vec<UTxO> {
        self.tx_out_list().iter().enumerate().map(|(index, tx_out)| {
            UTxO::new(
                self.id(),
                index,
                tx_out.address(),
                tx_out.amount(),
            )
        })
        .collect::<Vec<UTxO>>()
    }

    pub fn consumed_utxo(&self) -> Vec<UTxO> {
        self.tx_in_list()
            .iter()
            .map(|tx_in| {
                UTxO::new(
                    tx_in.tx_out_id(),
                    tx_in.tx_out_index(),
                    "".to_string(),
                    0,
                )
            })
            .collect::<Vec<UTxO>>()
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Transaction{{{},{},{}}}",
            self.id(),
            self.tx_in_list()
                .iter()
                .map(|v| format!("{}", v))
                .reduce(|a, b| format!("{}{}", a, b))
                .unwrap(),
            self.tx_out_list()
                .iter()
                .map(|v| format!("{}", v))
                .reduce(|a, b| format!("{}{}", a, b))
                .unwrap(),
        )
    }
}
