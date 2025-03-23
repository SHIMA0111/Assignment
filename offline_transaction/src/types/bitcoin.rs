use serde::{Deserialize, Serialize};


/// Represents the data required for creating a Bitcoin transaction
///
/// This struct contains all necessary information to construct a Bitcoin transaction,
/// including inputs, outputs, change address, private key for signing, and fee rate.
///
/// # Fields
///
/// * `inputs` - Vector of transaction inputs containing previous transaction references and amounts
/// * `outputs` - Vector of transaction outputs specifying destination addresses and amounts
/// * `change_address` - Address where remaining funds (after outputs and fees) will be sent
/// * `private_key` - Private key used to sign the transaction
/// * `fee_rate` - Fee rate in satoshis per byte for transaction fee calculation
#[derive(Debug, Serialize, Deserialize)]
pub struct BitcoinTransactionData {
    inputs: Vec<BitcoinTransactionInput>,
    outputs: Vec<BitcoinTransactionOutput>,
    #[serde(rename = "changeAddress")]
    change_address: String,
    #[serde(rename = "privateKey")]
    private_key: String,
    #[serde(rename = "feeRate")]
    fee_rate: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct BitcoinTransactionInput {
    txid: String,
    vout: u32,
    amount: f64,
    address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct BitcoinTransactionOutput {
    address: String,
    amount: f64,
}

impl BitcoinTransactionData {
    /// Creates a new BitcoinTransactionData instance with default values
    ///
    /// Initializes a new transaction data structure with empty inputs and outputs,
    /// and sets the source address as the change address. The fee rate is set to
    /// a default value of 1 satoshi per byte.
    ///
    /// # Arguments
    ///
    /// * `source_address` - The address to send any change amount back to or UTXO source address
    /// * `private_key` - The private key used for signing the transaction
    ///
    /// # Returns
    ///
    /// A new BitcoinTransactionData instance with the specified parameters
    pub fn new(&self, source_address: &str, private_key: &str) -> Self {
        Self {
            inputs: Vec::new(),
            outputs: Vec::new(),
            change_address: source_address.to_string(),
            private_key: private_key.to_string(),
            fee_rate: 1,
        }
    }
    

    /// Adds a new input to the transaction
    ///
    /// This method adds a new transaction input to the inputs vector. The input's source address
    /// is automatically set to the change address specified during initialization.
    /// Uses the builder pattern to allow method chaining.
    ///
    /// # Arguments
    ///
    /// * `txid` - The transaction ID of the UTXO to spend
    /// * `vout` - The output index in the referenced transaction
    /// * `amount` - The amount in BTC contained in this input
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to self to enable method chaining
    pub fn push_input(&mut self, txid: &str, vout: u32, amount: f64) -> &mut Self {
        let new_input = BitcoinTransactionInput {
            txid: txid.to_string(),
            vout,
            amount,
            address: self.change_address.clone(),
        };
        self.inputs.push(new_input);
        
        self
    }
    

    /// Adds a new output to the transaction
    ///
    /// This method adds a new transaction output to the outputs vector.
    /// Uses the builder pattern to allow method chaining.
    ///
    /// # Arguments
    ///
    /// * `address` - The destination address for this output
    /// * `amount` - The amount in BTC to send to this address
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to self to enable method chaining
    pub fn push_output(&mut self, address: &str, amount: f64) -> &mut Self {
        let new_output = BitcoinTransactionOutput {
            address: address.to_string(),
            amount,
        };
        self.outputs.push(new_output);
        
        self
    }
    

    /// Updates the transaction fee rate
    ///
    /// This is an optional method to modify the fee rate used for transaction fee calculation.
    /// If not set, the default fee rate of 1 satoshi per byte will be used.
    /// Uses the builder pattern to allow method chaining.
    ///
    /// # Arguments
    ///
    /// * `fee_rate` - The new fee rate in satoshis per byte
    ///
    /// # Returns
    ///
    /// Returns a mutable reference to self to enable method chaining
    pub fn set_fee_rate(&mut self, fee_rate: u64) -> &mut Self {
        self.fee_rate = fee_rate;
        
        self
    }
    
    pub(crate) fn inputs(&self) -> &Vec<BitcoinTransactionInput> {
        &self.inputs
    }
    
    pub(crate) fn outputs(&self) -> &Vec<BitcoinTransactionOutput> {
        &self.outputs
    }

    pub(crate) fn change_address(&self) -> &str {
        &self.change_address
    }

    pub(crate) fn private_key(&self) -> &str {
        &self.private_key
    }
    
    pub(crate) fn fee_rate(&self) -> u64 {
        self.fee_rate
    }
}

impl BitcoinTransactionInput {
    pub(crate) fn txid(&self) -> &str {
        &self.txid
    }
    
    pub(crate) fn vout(&self) -> u32 {
        self.vout
    }
    
    pub(crate) fn amount(&self) -> f64 {
        self.amount
    }
    
    pub(crate) fn address(&self) -> &str {
        &self.address
    }
}

impl BitcoinTransactionOutput {
    pub(crate) fn address(&self) -> &str {
        &self.address
    }
    
    pub(crate) fn amount(&self) -> f64 {
        self.amount
    }
}
