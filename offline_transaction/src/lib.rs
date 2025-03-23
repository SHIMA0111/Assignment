use std::fs::File;
use std::io::Read;
use std::path::Path;
use crate::errors::OfflineTransactionError::{FileNotFound, FileOperationError, ParseError, UnsupportedError};
use crate::errors::OfflineTransactionResult;
use crate::types::bitcoin::BitcoinTransactionData;
use crate::types::Network;

pub mod errors;
pub mod types;
pub mod transaction;
mod utils;

/// An enum representing different blockchain platforms for offline transaction creation.
/// Currently, this implementation only supports Bitcoin transactions, but it's designed
/// to be extensible for other blockchain platforms in the future.
///
/// # Variants
///
/// * `Bitcoin` - Handles Bitcoin transaction data and operations, containing a [`BitcoinTransactionData`]
///   structure with all necessary information for creating and signing Bitcoin transactions.
///
/// # Note
///
/// While the enum is designed to support multiple platforms, currently only Bitcoin
/// is implemented. Future versions may add support for other cryptocurrencies.
#[derive(Debug)]
pub enum Platform {
    Bitcoin(BitcoinTransactionData),
}

impl Platform {
    /// Creates a Platform instance from a JSON file
    ///
    /// # Arguments
    ///
    /// * `json_path` - Path to the JSON file containing transaction data
    ///
    /// Specified JSON needs to have "network" key.
    /// 
    /// # Returns
    ///
    /// * `OfflineTransactionResult<Self>` - Returns a Platform instance if successful, or an error if:
    ///   - File is not found
    ///   - File cannot be read
    ///   - JSON parsing fails
    ///   - Network type is unsupported (currently only "bitcoin" is supported)
    pub fn from_json<JP: AsRef<Path>>(json_path: JP) -> OfflineTransactionResult<Self> {
        let json_path = json_path.as_ref();
        if !(json_path.exists() && json_path.is_file()) {
            return Err(FileNotFound {
                path: format!("{}", json_path.display())
            });
        }
        
        let mut file = File::open(json_path)
            .map_err(|e| FileOperationError {
                operation: "open".to_string(),
                reason: e.to_string(),
            })?;
        
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| FileOperationError {
                operation: "read".to_string(),
                reason: e.to_string(),
            })?;
        
        Self::json_parser(&contents)
    }
    
    fn json_parser(json_str: &str) -> OfflineTransactionResult<Self> {
        let network_config: Network = serde_json::from_str(json_str)
            .map_err(|_| ParseError {
                from: "Json".to_string(),
                to: "Network".to_string(),
                reason: "input json needs to include 'network' key".to_string()
            })?;

        match network_config.network.to_lowercase().trim() {
            "bitcoin" => {
                let transaction_data: BitcoinTransactionData = serde_json::from_str(json_str)
                    .map_err(|e| ParseError {
                        from: "Json".to_string(),
                        to: "BitcoinTransactionData".to_string(),
                        reason: e.to_string()
                    })?;

                Ok(Platform::Bitcoin(transaction_data))
            },
            _ => {
                Err(UnsupportedError {
                    component: "network".to_string(),
                    input: network_config.network,
                    expected: "bitcoin".to_string(),
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_from_json() {
        let json_path = Path::new("tests/data/sample.json");
        let result = Platform::from_json(json_path);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_from_json_error() {
        let json_path = Path::new("tests/data/sample_error.json");
        let result = Platform::from_json(json_path);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_json_parser() {
        let json_str = r#"{"network": "bitcoin", "inputs": [], "outputs": [], "changeAddress": "", "privateKey": "", "feeRate": 1}"#;
        let result = Platform::json_parser(json_str);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_json_parser_error() {
        let json_str = r#"{"network": "ethereum", "inputs": [], "outputs": [], "changeAddress": "", "privateKey": "", "feeRate": 1}"#;
        let result = Platform::json_parser(json_str);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_json_parser_error_2() {
        let json_str = r#"{"network": "bitcoin", "test": "test"}"#;
        let result = Platform::json_parser(json_str);
        assert!(result.is_err());
    }
}
