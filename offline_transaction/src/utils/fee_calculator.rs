use bitcoin::{TxIn, TxOut};
use crate::errors::OfflineTransactionResult;


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
                             fee_rate: u64) -> OfflineTransactionResult<u64> {
    let base_and_input_size_calc = |input: &TxIn| -> u64 {
        // version: 4 + input: 1 + output: 1 + lock_time: 4
        let mut tx_size: u64 = 10;
        if input.script_sig.is_p2wpkh() {
            // HRP: bc 2 words
            tx_size += 2;
            // P2WPKH: signature: about 72 bytes + pubkey: about 33 bytes
            tx_size += (72 + 33) * inputs.len() as u64;
        } else {
            // P2PKH's input is around 148 bytes.
            tx_size += 148 * inputs.len() as u64;
        }
        
        tx_size
    };
    
    let base_and_input_size = inputs.iter()
        .map(|input| base_and_input_size_calc(input))
        .sum::<u64>();
    
    // P2PKH's output is around 34 and P2WPKH's output is around 31 so it treats as 34.
    let output_size = outputs.len() as u64 * 34;
    
    Ok((base_and_input_size + output_size) * fee_rate)
}