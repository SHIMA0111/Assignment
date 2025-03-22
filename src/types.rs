use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct TransactionInput {
    /// Previous transaction ID
    txid: String,
    /// Index of the output use
    vout: u32,
    /// Input amount of UTXO
    amount: String,
    /// Address of the UTXO
    address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct TransactionOutput {
    /// Address of the distance
    address: String,
    /// Remittance amount
    amount: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct TransactionConfig {
    inputs: Vec<TransactionInput>,
    outputs: Vec<TransactionOutput>,
    #[serde(rename = "changeAddress")]
    change_address: String,
    #[serde(rename = "privateKey")]
    private_key: String,
    #[serde(rename = "feeRate")]
    fee_rate: u64,
}

impl TransactionInput {
    pub(crate) fn txid(&self) -> &str {
        &self.txid
    }
    
    pub(crate) fn vout(&self) -> u32 {
        self.vout
    }
    
    pub(crate) fn amount(&self) -> &str {
        &self.amount
    }
    
    pub(crate) fn address(&self) -> &str {
        &self.address
    }
}

impl TransactionOutput {
    pub(crate) fn address(&self) -> &str {
        &self.address
    }
    
    pub(crate) fn amount(&self) -> &str {
        &self.amount
    }
}

impl TransactionConfig {
    pub(crate) fn inputs(&self) -> &Vec<TransactionInput> {
        &self.inputs
    }
    
    pub(crate) fn outputs(&self) -> &Vec<TransactionOutput> {
        &self.outputs
    }
    
    pub(crate) fn private_key(&self) -> &str {
        self.private_key.as_str()
    }
    
    pub(crate) fn fee_rate(&self) -> u64 {
        self.fee_rate
    }
    
    pub(crate) fn change_address(&self) -> &str {
        self.change_address.as_str()
    }
}
