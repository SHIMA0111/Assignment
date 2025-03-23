use bitcoin::{ScriptBuf, Transaction};
use bitcoin::consensus::encode;
use colored::Colorize;

mod generate_transaction;
mod sign_transaction;


/// Represents transaction data for different cryptocurrency platforms
///
/// # Variants
///
/// ## BitcoinTransaction
/// Contains data required for Bitcoin transaction processing:
/// * `raw_transaction` - The Bitcoin transaction object
/// * `signed` - Boolean flag indicating if the transaction is signed
/// * `input_data` - Vector of tuples containing amount (in satoshis) and script for each input
/// * `private_key` - Private key used for signing the transaction
pub enum TransactionData {
    BitcoinTransaction {
        raw_transaction: Transaction,
        signed: bool,
        input_data: Vec<(u64, ScriptBuf)>,
        private_key: String,
    }
}

impl TransactionData {
    /// Marks the transaction as signed
    ///
    /// This method updates the internal signed flag to true
    pub(crate) fn signed(&mut self) {
        match self {
            TransactionData::BitcoinTransaction { signed, .. } => *signed = true,
        }
    }

    /// Returns whether the transaction is signed
    ///
    /// This method checks the internal signed flag and returns its value
    ///
    /// # Returns
    ///
    /// * `bool` - True if the transaction is signed, false otherwise
    pub fn is_signed(&self) -> bool {
        match self {
            TransactionData::BitcoinTransaction { signed, .. } => *signed,
        }
    }

    /// Returns the raw transaction as a hexadecimal string
    ///
    /// If the transaction is unsigned, displays a warning message.
    /// The transaction is serialized and encoded as a hexadecimal string.
    ///
    /// # Returns
    ///
    /// * `String` - The transaction data encoded as a hexadecimal string
    pub fn get_raw_transaction(&self) -> String {
        match self {
            TransactionData::BitcoinTransaction {
                raw_transaction, signed, ..} => {
                if !*signed {
                    println!("{}: {}", 
                             "WARNING".bold().yellow(), 
                             "output transaction is unsigned, \
                             if you want to get signed transaction, execute sign_transaction()")
                }
                
                hex::encode(encode::serialize(raw_transaction))
            }
        }
    }
}
