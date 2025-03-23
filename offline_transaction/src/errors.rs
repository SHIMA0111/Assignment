use thiserror::Error;


/// A specialized Result type for offline transaction operations
///
/// This type alias is used throughout the crate to provide a consistent error
/// handling mechanism for operations that may fail during offline transaction
/// processing.
///
/// # Type Parameters
///
/// * `T` - The type of the successful result value
///
/// # Examples
///
/// ```
/// use offline_transaction::errors::OfflineTransactionResult;
///
/// fn process_data() -> OfflineTransactionResult<()> {
///     // Operation that might fail
///     Ok(())
/// }
/// ```
pub type OfflineTransactionResult<T> = Result<T, OfflineTransactionError>;


/// Represents errors that can occur during offline transaction operations
///
/// This enum encompasses various error cases that might arise during file operations,
/// parsing, and other offline transaction-related tasks. Each variant provides
/// specific information about what went wrong and where.
///
/// # Variants
///
/// * `FileNotFound` - The specified file could not be found at the given path
/// * `InvalidFile` - The file exists but is not of the expected type
/// * `FileOperationError` - An error occurred during file operations (read/write)
/// * `ParseError` - Failed to parse data from one format to another
/// * `UnsupportedError` - The provided input is not supported for the given component
/// * `HasherError` - An error occurred during hashing operations
#[derive(Error, Debug)]
pub enum OfflineTransactionError {
    #[error("File not found: {path}")]
    FileNotFound {
        path: String,
    },
    #[error("Invalid file: {path}, expected {expected_type} file")]
    InvalidFile {
        path: String,
        expected_type: String,
    },
    #[error("File {operation} failed: {reason}")]
    FileOperationError {
        operation: String,
        reason: String,
    },
    #[error("Failed to parse from {from} to {to}: {reason}")]
    ParseError {
        from: String,
        to: String,
        reason: String,
    },
    #[error("Unsupported {component} error: {input}, expected one of [{expected}]")]
    UnsupportedError {
        component: String,
        input: String,
        expected: String,
    },
    #[error("Hasher error: {0}")]
    HasherError(String),
}