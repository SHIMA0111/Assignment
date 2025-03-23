use std::fs::File;
use std::io::{Read, Write};
use anyhow::anyhow;
use bitcoin::consensus::encode;
use clap::Parser;
use crate::cli::Args;
use crate::signing_service::create_signed_transaction;
use crate::types::TransactionConfig;

mod cli;
mod types;
mod errors;
mod signing_service;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    
    let input_json = args.input();
    if !(input_json.exists() && input_json.is_file()) {
        return Err(anyhow!("Input file does not exist!"))
    }
    
    let mut file = File::open(input_json)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    let params: TransactionConfig = serde_json::from_str(&contents)?;
    
    let tx = create_signed_transaction(&params)?;
    let tx_hex = hex::encode(encode::serialize(&tx));
    
    if let Some(output_path) = args.output() {
        let mut output_file = File::create(output_path)?;
        output_file.write_fmt(format_args!("{}", tx_hex))?;
        println!("Output transaction to {}", output_path.display());
    }
    else {
        println!("Signed transaction: {}", tx_hex);
    }
    
    println!("Transaction ID: {}", tx.compute_txid());
    
    Ok(())
}
