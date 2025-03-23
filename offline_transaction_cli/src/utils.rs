use colored::Colorize;
use offline_transaction::errors::OfflineTransactionError;

pub(crate) fn display_offline_transaction_error(error: OfflineTransactionError) {
    match &error {
        OfflineTransactionError::FileNotFound {..} => {
            eprintln!("{}: {}", "FileNotFound".bold().red(), error.to_string())
        },
        OfflineTransactionError::FileOperationError {..} => {
            eprintln!("{}: {}", "FileOperationError".bold().red(), error.to_string())
        },
        OfflineTransactionError::InvalidFile {..} => {
            eprintln!("{}: {}", "InvalidFile".bold().red(), error.to_string())
        },
        OfflineTransactionError::ParseError {..} => {
            eprintln!("{}: {}", "ParseError".bold().red(), error.to_string())
        },
        OfflineTransactionError::HasherError(_) => {
            eprintln!("{}: {}", "HasherError".bold().red(), error.to_string())
        },
        OfflineTransactionError::UnsupportedError {..} => {
            eprintln!("{}: {}", "UnsupportedError".bold().red(), error.to_string())
        }
    }
}