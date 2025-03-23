use offline_transaction::Platform;
use std::path::Path;

#[test]
fn test_platform_from_json_success() {
    // Test that a valid JSON file is correctly parsed
    let json_path = Path::new("../test_data/input.json");
    let platform = Platform::from_json(json_path);
    
    assert!(platform.is_ok(), "Platform::from_json should succeed with a valid JSON file");
    
    // Verify that the platform is a Bitcoin platform
    match platform.unwrap() {
        Platform::Bitcoin(_) => {
            // Success - the platform is a Bitcoin platform
        }
    }
}

#[test]
fn test_platform_from_json_file_not_found() {
    // Test that an error is returned when the file is not found
    let json_path = Path::new("non_existent_file.json");
    let platform = Platform::from_json(json_path);
    
    assert!(platform.is_err(), "Platform::from_json should fail with a non-existent file");
    
    // Verify that the error is a FileNotFound error
    match platform.unwrap_err() {
        offline_transaction::errors::OfflineTransactionError::FileNotFound { .. } => {
            // Success - the error is a FileNotFound error
        }
        err => {
            panic!("Expected FileNotFound error, got: {:?}", err);
        }
    }
}

#[test]
fn test_platform_from_json_invalid_network() {
    // Create a temporary file with an invalid network
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("invalid_network.json");
    
    std::fs::write(
        &file_path,
        r#"{
            "network": "ethereum",
            "inputs": [],
            "outputs": [],
            "changeAddress": "address",
            "privateKey": "key",
            "feeRate": 1
        }"#,
    )
    .unwrap();
    
    let platform = Platform::from_json(&file_path);
    
    assert!(platform.is_err(), "Platform::from_json should fail with an invalid network");
    
    // Verify that the error is an UnsupportedError
    match platform.unwrap_err() {
        offline_transaction::errors::OfflineTransactionError::UnsupportedError { .. } => {
            // Success - the error is an UnsupportedError
        }
        err => {
            panic!("Expected UnsupportedError, got: {:?}", err);
        }
    }
}

#[test]
fn test_platform_from_json_invalid_json() {
    // Create a temporary file with invalid JSON
    let temp_dir = tempfile::tempdir().unwrap();
    let file_path = temp_dir.path().join("invalid_json.json");
    
    std::fs::write(
        &file_path,
        r#"{
            "network": "bitcoin",
            "inputs": [
                {
                    "txid": "invalid",
                    "vout": "not_a_number", // This should be a number
                    "amount": 0.1,
                    "address": "address"
                }
            ],
            "outputs": [],
            "changeAddress": "address",
            "privateKey": "key",
            "feeRate": 1
        }"#,
    )
    .unwrap();
    
    let platform = Platform::from_json(&file_path);
    
    assert!(platform.is_err(), "Platform::from_json should fail with invalid JSON");
    
    // Verify that the error is a ParseError
    match platform.unwrap_err() {
        offline_transaction::errors::OfflineTransactionError::ParseError { .. } => {
            // Success - the error is a ParseError
        }
        err => {
            panic!("Expected ParseError, got: {:?}", err);
        }
    }
}