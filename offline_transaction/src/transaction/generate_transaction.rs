use std::str::FromStr;
use bitcoin::{Amount, OutPoint, ScriptBuf, Sequence, Transaction, TxIn, TxOut, Txid, Witness};
use bitcoin::absolute::LockTime;
use bitcoin::transaction::Version;
use crate::errors::OfflineTransactionError::ParseError;
use crate::errors::OfflineTransactionResult;
use crate::Platform;
use crate::transaction::TransactionData;
use crate::utils::btc_sat_trans::btc_to_sat;
use crate::utils::fee_calculator::fee_calculator;
use crate::utils::str_to_address::str_to_address_unchecked;

impl Platform {
    /// Generates an unsigned transaction based on the platform parameters
    ///
    /// For example, in Bitcoin, this method creates a new unsigned transaction with the following steps:
    /// 1. Converts input transactions to Bitcoin transaction inputs (TxIn)
    /// 2. Converts output specifications to Bitcoin transaction outputs (TxOut)
    /// 3. Calculates the transaction fee based on the fee rate
    /// 4. Adds a change output if the remaining amount is greater than the dust threshold (546 satoshis)
    ///
    /// # Returns
    ///
    /// Returns a Result containing either:
    /// * `TransactionData` - Contains the unsigned transaction and associated data
    /// * `OfflineTransactionError` - If there's an error during transaction generation
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * Input transaction IDs cannot be parsed
    /// * Addresses cannot be parsed
    /// * Fee calculation fails
    pub fn generate_unsigned_transaction(&self) -> OfflineTransactionResult<TransactionData> {
        match self {
            Platform::Bitcoin(params) => {
                let mut tx_inputs = Vec::new();
                let mut input_details = Vec::new();
                let mut total_input_amount: u64 = 0;

                for input in params.inputs() {
                    let txid = Txid::from_str(input.txid())
                        .map_err(|e| ParseError {
                            from: "string".to_string(),
                            to: "Txid".to_string(),
                            reason: e.to_string()
                        })?;
                    let outpoint = OutPoint::new(txid, input.vout());

                    tx_inputs.push(TxIn {
                        previous_output: outpoint,
                        script_sig: ScriptBuf::new(),
                        // Currently, limitation of the relative lock time is unsupported.
                        sequence: Sequence::MAX,
                        witness: Witness::new(),
                    });

                    let input_amound_sat = btc_to_sat(input.amount());
                    let address = str_to_address_unchecked(input.address())?;

                    input_details.push((input_amound_sat, address.script_pubkey()));
                    total_input_amount += input_amound_sat;
                }

                let mut tx_outputs = Vec::new();
                let mut total_output_amount: u64 = 0;

                for output in params.outputs() {
                    let address = str_to_address_unchecked(output.address())?;

                    let output_amount_sat = btc_to_sat(output.amount());

                    tx_outputs.push(TxOut {
                        value: Amount::from_sat(output_amount_sat),
                        script_pubkey: address.script_pubkey(),
                    });

                    total_output_amount += output_amount_sat;
                }

                let transaction_fee = fee_calculator(
                    &tx_inputs,
                    &tx_outputs,
                    params.change_address(),
                    params.fee_rate())?;
                let change_amount = total_input_amount - total_output_amount - transaction_fee;

                // Under 546 satoshi, the change will be treated as dust.
                if change_amount > 546 {
                    let change_address = str_to_address_unchecked(params.change_address())?;
                    tx_outputs.push(TxOut {
                        value: Amount::from_sat(change_amount),
                        script_pubkey: change_address.script_pubkey(),
                    });
                }

                let tx = Transaction {
                    version: Version::TWO,
                    // Lock time unsupported
                    lock_time: LockTime::ZERO,
                    input: tx_inputs,
                    output: tx_outputs,
                };

                Ok(TransactionData::BitcoinTransaction {
                    raw_transaction: tx,
                    signed: false,
                    input_data: input_details,
                    private_key: params.private_key().to_string(),
                })
            }
        }
    }
}