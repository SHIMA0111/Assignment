mod utils;

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;
use clap::Parser;
use offline_transaction::errors::OfflineTransactionError;
use offline_transaction::Platform;
use crate::utils::display_offline_transaction_error;

#[derive(Parser, Debug)]
#[command(name = "offline_transaction", author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, help = "Path of the input JSON file")]
    input: PathBuf,
    
    #[clap(short, long, help = "Path of the output JSON file (Optional)")]   
    output: Option<PathBuf>,
}

fn main() {
    let cli_args = Cli::parse();
    
    let input_json_path = cli_args.input;
    let output_path = cli_args.output;
    
    let platform = Platform::from_json(input_json_path)
        .unwrap_or_else(|e| {
            display_offline_transaction_error(e);
            exit(1);
        });
    let raw_transaction = platform.generate_unsigned_transaction()
        .unwrap_or_else(|e| {
            display_offline_transaction_error(e);
            exit(1);       
        })
        .sign_transaction()
        .unwrap_or_else(|e| {
            display_offline_transaction_error(e);
            exit(1);
        })
        .get_raw_transaction();
    
    if let Some(output_path) = output_path {
        let mut output_file = File::create(&output_path)
            .map_err(|e| OfflineTransactionError::FileOperationError {
                operation: "open".to_string(),
                reason: e.to_string(),
            })
            .unwrap_or_else(|e| {
                display_offline_transaction_error(e);
                exit(1);
            });
        output_file.write_fmt(format_args!("{}", raw_transaction))
            .map_err(|e| OfflineTransactionError::FileOperationError {
                operation: "write".to_string(),
                reason: e.to_string(),
            })
            .unwrap_or_else(|e| {
                display_offline_transaction_error(e);
                exit(1);
            });
        println!("Transaction written to {}", output_path.as_path().display());
    } else {
        println!("{}", raw_transaction);
    }
}
