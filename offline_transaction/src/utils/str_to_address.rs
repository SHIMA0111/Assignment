use std::str::FromStr;
use bitcoin::Address;
use crate::errors::OfflineTransactionError::ParseError;
use crate::errors::OfflineTransactionResult;

/// Converts a string to a Bitcoin address
///
/// This is an internal function that attempts to parse a string into a Bitcoin address.
/// If the parsing fails, it returns a ParseError with detailed information about the failure.
///
/// # Arguments
///
/// * `address` - The string to be converted to a Bitcoin address
///
/// # Returns
///
/// * `OfflineTransactionResult<Address>` - The parsed Bitcoin address or an error if parsing fails
pub(crate) fn str_to_address_unchecked(address: &str) -> OfflineTransactionResult<Address> {
    let address = Address::from_str(address)
        .map_err(|e| ParseError {
            from: "string".to_string(),
            to: "Address".to_string(),
            reason: e.to_string()
        })?
        .assume_checked();
    
    Ok(address)
}