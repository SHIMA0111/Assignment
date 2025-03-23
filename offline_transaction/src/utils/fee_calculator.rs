use bitcoin::{Amount, Transaction, TxIn, TxOut};
use bitcoin::absolute::LockTime;
use bitcoin::transaction::Version;
use crate::errors::OfflineTransactionResult;
use crate::utils::str_to_address::str_to_address_unchecked;


/// Calculates transaction fee based on inputs, outputs, change address and fee rate
///
/// This is an internal function that estimates the total transaction fee by calculating the virtual size
/// of the transaction and multiplying it by the fee rate.
///
/// # Arguments
///
/// * `inputs` - Vector of transaction inputs
/// * `outputs` - Vector of transaction outputs 
/// * `change_address` - Address for returning change
/// * `fee_rate` - Fee rate in satoshis per virtual byte
///
/// # Returns
///
/// * `OfflineTransactionResult<u64>` - Calculated fee in satoshis or an error if address parsing fails
pub(crate) fn fee_calculator(inputs: &[TxIn],
                             outputs: &[TxOut],
                             change_address: &str,
                             fee_rate: u64) -> OfflineTransactionResult<u64> {
    let change_address_script_pubkey = str_to_address_unchecked(change_address)?
        .script_pubkey();
    let mut tmp_outputs = outputs.to_vec();
    tmp_outputs.push(
        TxOut {
            value: Amount::from_sat(0),
            script_pubkey: change_address_script_pubkey,
        }
    );
    let tx_size = Transaction {
        version: Version::TWO,
        lock_time: LockTime::ZERO,
        input: inputs.to_vec(),
        output: tmp_outputs,
    }.vsize() as u64;
    
    Ok(tx_size * fee_rate)
}