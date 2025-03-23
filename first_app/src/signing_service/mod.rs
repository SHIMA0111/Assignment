use std::str::FromStr;
use bitcoin::{Address, Amount, OutPoint, PrivateKey, PublicKey, ScriptBuf, Sequence, Transaction, TxIn, TxOut, Txid, Witness};
use bitcoin::absolute::LockTime;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::transaction::Version;
use crate::signing_service::estimator::estimate_tx_size;
use crate::signing_service::sign_input::sign_input;
use crate::types::TransactionConfig;

mod estimator;
mod sign_input;

pub(crate) fn create_signed_transaction(params: &TransactionConfig) -> anyhow::Result<Transaction> {
    let secp = Secp256k1::new();
    
    let private_key = PrivateKey::from_wif(params.private_key())?;
    let public_key = PublicKey::from_private_key(&secp, &private_key);
    
    let mut tx_inputs = Vec::new();
    let mut input_details = Vec::new();
    
    for input in params.inputs() {
        let txid = Txid::from_str(input.txid())?;
        let outpoint = OutPoint::new(txid, input.vout());
        
        tx_inputs.push(TxIn {
            previous_output: outpoint,
            script_sig: ScriptBuf::new(),
            sequence: Sequence::MAX,
            witness: Witness::new(),
        });
        
        let amount_btc: f64 = input.amount().parse()?;
        let amount_sat = (amount_btc * 100_000_000.0) as u64;
        
        let address = Address::from_str(input.address())?
            .assume_checked();
        
        input_details.push((amount_sat, address.script_pubkey()));
    }
    
    let mut tx_outputs = Vec::new();
    let mut total_output_amount: u64 = 0u64;
    
    for output in params.outputs() {
        let address = Address::from_str(output.address())?
            .assume_checked();
        
        let amount_btc: f64 = output.amount().parse()?;
        let amount_sat = (amount_btc * 100_000_000.0) as u64;
        
        tx_outputs.push(TxOut {
            value: Amount::from_sat(amount_sat),
            script_pubkey: address.script_pubkey(),
        });
        
        total_output_amount += amount_sat;
    }
    
    let total_input_amount: u64 = input_details.iter().map(|(amount, _)| *amount).sum();
    
    let has_segwit = input_details.iter().any(|(_, script)| script.is_p2wpkh());
    // When the change_amount is over the dust limit, the tx size will be up to the +1 transaction output size. 
    let tx_size = estimate_tx_size(input_details.len(), tx_outputs.len() + 1, has_segwit);
    
    let fee = tx_size * params.fee_rate();
    
    let change_amount = total_input_amount - total_output_amount - fee;
    
    // Bitcoin dust limit is 546 satoshis.
    if change_amount > 546 {
        let change_address = Address::from_str(params.change_address())?
            .assume_checked();
        tx_outputs.push(TxOut {
            value: Amount::from_sat(change_amount),
            script_pubkey: change_address.script_pubkey(),
        });
    }
    
    let mut tx = Transaction {
        version: Version::TWO,
        lock_time: LockTime::ZERO,
        input: tx_inputs,
        output: tx_outputs
    };
    
    for (i, (amount, script_pubkey)) in input_details.iter().enumerate() {
        sign_input(&mut tx, i, *amount, script_pubkey, &private_key, &public_key, &secp)?;
    }
    
    Ok(tx)
}