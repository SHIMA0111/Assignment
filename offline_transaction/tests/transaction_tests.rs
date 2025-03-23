use offline_transaction::{Platform, transaction::TransactionData};
use std::path::Path;

#[test]
fn test_generate_unsigned_transaction() {
    // Load a platform from a JSON file
    let json_path = Path::new("../test_data/input.json");
    let platform = Platform::from_json(json_path).expect("Failed to load platform from JSON");
    
    // Generate an unsigned transaction
    let transaction_data = platform.generate_unsigned_transaction()
        .expect("Failed to generate unsigned transaction");
    
    // Verify that the transaction is not signed
    match &transaction_data {
        TransactionData::BitcoinTransaction { signed, .. } => {
            assert!(!signed, "Transaction should not be signed initially");
        }
    }
    
    // Verify that the raw transaction can be retrieved
    let raw_tx = transaction_data.get_raw_transaction();
    assert!(!raw_tx.is_empty(), "Raw transaction should not be empty");
}

#[test]
fn test_sign_transaction() {
    // Load a platform from a JSON file
    let json_path = Path::new("../test_data/input.json");
    let platform = Platform::from_json(json_path).expect("Failed to load platform from JSON");
    
    // Generate an unsigned transaction
    let mut transaction_data = platform.generate_unsigned_transaction()
        .expect("Failed to generate unsigned transaction");
    
    // Sign the transaction
    transaction_data.sign_transaction().expect("Failed to sign transaction");
    
    // Verify that the transaction is signed
    match &transaction_data {
        TransactionData::BitcoinTransaction { signed, .. } => {
            assert!(signed, "Transaction should be signed after signing");
        }
    }
    
    // Verify that the raw transaction can be retrieved
    let raw_tx = transaction_data.get_raw_transaction();
    assert!(!raw_tx.is_empty(), "Raw transaction should not be empty");
}

#[test]
fn test_transaction_data_methods() {
    // Load a platform from a JSON file
    let json_path = Path::new("../test_data/input.json");
    let platform = Platform::from_json(json_path).expect("Failed to load platform from JSON");
    
    // Generate an unsigned transaction
    let mut transaction_data = platform.generate_unsigned_transaction()
        .expect("Failed to generate unsigned transaction");
    
    // Test the signed method
    match &transaction_data {
        TransactionData::BitcoinTransaction { signed, .. } => {
            assert!(!signed, "Transaction should not be signed initially");
        }
    }
    
    // Mark the transaction as signed
    transaction_data.signed();
    
    // Verify that the transaction is now marked as signed
    match &transaction_data {
        TransactionData::BitcoinTransaction { signed, .. } => {
            assert!(signed, "Transaction should be signed after calling signed()");
        }
    }
    
    // Test the get_raw_transaction method
    let raw_tx = transaction_data.get_raw_transaction();
    assert!(!raw_tx.is_empty(), "Raw transaction should not be empty");
}

#[test]
fn test_transaction_workflow() {
    // This test verifies the complete workflow from loading a platform to generating and signing a transaction
    
    // Load a platform from a JSON file
    let json_path = Path::new("../test_data/input.json");
    let platform = Platform::from_json(json_path).expect("Failed to load platform from JSON");
    
    // Generate an unsigned transaction
    let mut transaction_data = platform.generate_unsigned_transaction()
        .expect("Failed to generate unsigned transaction");
    
    // Verify that the transaction is not signed
    match &transaction_data {
        TransactionData::BitcoinTransaction { signed, .. } => {
            assert!(!signed, "Transaction should not be signed initially");
        }
    }
    
    // Get the raw unsigned transaction
    let unsigned_tx = transaction_data.get_raw_transaction();
    assert!(!unsigned_tx.is_empty(), "Unsigned transaction should not be empty");
    
    // Sign the transaction
    transaction_data.sign_transaction().expect("Failed to sign transaction");
    
    // Verify that the transaction is signed
    match &transaction_data {
        TransactionData::BitcoinTransaction { signed, .. } => {
            assert!(signed, "Transaction should be signed after signing");
        }
    }
    
    // Get the raw signed transaction
    let signed_tx = transaction_data.get_raw_transaction();
    assert!(!signed_tx.is_empty(), "Signed transaction should not be empty");
    
    // The signed transaction should be different from the unsigned transaction
    assert_ne!(unsigned_tx, signed_tx, "Signed transaction should be different from unsigned transaction");
}