use bitcoin::{Amount, EcdsaSighashType, PrivateKey, PublicKey, ScriptBuf, Witness};
use bitcoin::hashes::Hash;
use bitcoin::script::PushBytesBuf;
use bitcoin::secp256k1::{Message, Secp256k1};
use bitcoin::sighash::SighashCache;
use crate::errors::OfflineTransactionError::{HasherError, ParseError, UnsupportedError};
use crate::errors::OfflineTransactionResult;
use crate::transaction::TransactionData;

impl TransactionData {
    /// Signs the transaction using the appropriate cryptographic algorithm for the platform
    ///
    /// This method handles the digital signature process for transaction inputs by:
    /// 1. Initializing the cryptographic context
    /// 2. Converting the private key from its string format
    /// 3. Deriving the corresponding public key
    /// 4. For each transaction input:
    ///    - Generates the signature hash based on input type
    ///    - Creates digital signature using the private key
    ///    - Adds the signature data to the transaction
    ///    - Sets up any required script or witness data
    ///
    /// # Returns
    ///
    /// * `OfflineTransactionResult<()>` - Ok(()) if signing succeeds, or an error if:
    ///   - Private key format is invalid
    ///   - Signature generation fails
    ///   - Input script type is unsupported
    pub fn sign_transaction(&mut self) -> OfflineTransactionResult<()> {
        let secp = Secp256k1::new();
        
        match self {
            TransactionData::BitcoinTransaction {
                raw_transaction,
                input_data, 
                private_key,
                ..
            } => {
                let hash_type = EcdsaSighashType::All;
                let private_key = PrivateKey::from_wif(private_key)
                    .map_err(|e| ParseError {
                        from: "WIF string".to_string(),
                        to: "PrivateKey".to_string(),
                        reason: e.to_string()
                    })?;
                let public_key = PublicKey::from_private_key(&secp, &private_key);
                
                
                for (idx, (amount, script_pubkey)) in input_data.iter().enumerate() {
                    if script_pubkey.is_p2pkh() {
                        let sighash = SighashCache::new(&mut *raw_transaction)
                            .legacy_signature_hash(
                                idx, script_pubkey, hash_type.to_u32()
                            )
                            .map_err(|e| HasherError(e.to_string()))?;
                        let message = Message::from_digest(sighash.to_byte_array());
                        let sig = secp.sign_ecdsa(&message, &private_key.inner);

                        let mut serialized_signature = sig.serialize_der().to_vec();
                        serialized_signature.push(hash_type as u8);
                        let serialized_signature_bytes = PushBytesBuf::try_from(serialized_signature)
                            .map_err(|e| ParseError {
                                from: "serialized_signature".to_string(),
                                to: "PushBytesBuf".to_string(),
                                reason: e.to_string()
                            })?;

                        let script_sig = ScriptBuf::builder()
                            .push_slice(&serialized_signature_bytes)
                            .push_key(&public_key)
                            .into_script();

                        raw_transaction.input[idx].script_sig = script_sig;
                    }
                    else if script_pubkey.is_p2wpkh() {
                        let sighash = SighashCache::new(&mut *raw_transaction)
                            .p2wpkh_signature_hash(idx, script_pubkey, Amount::from_sat(*amount), hash_type)
                            .map_err(|e| HasherError(e.to_string()))?;

                        let message = Message::from_digest(sighash.to_byte_array());
                        let sig = secp.sign_ecdsa(&message, &private_key.inner);

                        let mut serialized_signature = sig.serialize_der().to_vec();
                        serialized_signature.push(hash_type as u8);

                        let mut witness = Witness::new();
                        witness.push(serialized_signature);
                        witness.push(public_key.to_bytes());

                        let input = &mut raw_transaction.input[idx];
                        input.witness = witness;
                        input.script_sig = ScriptBuf::new();
                    }
                    else {
                        return Err(UnsupportedError {
                            component: "script_pubey type".to_string(),
                            input: "unknown".to_string(),
                            expected: "p2pkh, p2wpkh".to_string(),
                        });
                    }
                }
            }
        }
        
        self.signed();
        Ok(())
    }
}