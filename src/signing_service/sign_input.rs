use bitcoin::{Amount, EcdsaSighashType, PrivateKey, PublicKey, ScriptBuf, Transaction, Witness};
use bitcoin::hashes::Hash;
use bitcoin::script::{PushBytesBuf};
use bitcoin::secp256k1::{All, Message, Secp256k1};
use bitcoin::sighash::SighashCache;

pub(crate) fn sign_input(
    tx: &mut Transaction,
    input_index: usize,
    amount: u64,
    script_pubkey: &ScriptBuf,
    private_key: &PrivateKey,
    public_key: &PublicKey,
    secp: &Secp256k1<All>,
) -> anyhow::Result<()> {
    let hash_ty = EcdsaSighashType::All;
    
    // Legacy P2PKH
    if script_pubkey.is_p2pkh() {
        println!("Legacy P2PKH");
        let sighash = SighashCache::new(&mut *tx)
            .legacy_signature_hash(
                input_index,
                script_pubkey, 
                hash_ty.to_u32()
            )?;
        let message = Message::from_digest(sighash.to_byte_array());
        let sig = secp.sign_ecdsa(&message, &private_key.inner);
        
        let mut sig_serialized = sig.serialize_der().to_vec();
        sig_serialized.push(hash_ty as u8);
        let sig_bytes = PushBytesBuf::try_from(sig_serialized)?;
        
        let script_sig = ScriptBuf::builder()
            .push_slice(&sig_bytes)
            .push_key(public_key)
            .into_script();
        
        tx.input[input_index].script_sig = script_sig;
    }
    // Native SegWit
    else if script_pubkey.is_p2wpkh() {
        let sighash = SighashCache::new(&mut *tx)
            .p2wpkh_signature_hash(input_index, &script_pubkey, Amount::from_sat(amount), hash_ty)?;
        
        let message = Message::from_digest(sighash.to_byte_array());
        let sig = secp.sign_ecdsa(&message, &private_key.inner);
        
        let mut sig_serialized = sig.serialize_der().to_vec();
        sig_serialized.push(hash_ty as u8);
        
        let mut witness = Witness::new();
        witness.push(sig_serialized);
        witness.push(public_key.to_bytes());
        tx.input[input_index].witness = witness;
        
        // In Native Segwit, script_sig is empty
        tx.input[input_index].script_sig = ScriptBuf::new();
    }
    else {
        return Err(anyhow::anyhow!("Unsupported script_pubkey type"));
    }
    
    Ok(())
}