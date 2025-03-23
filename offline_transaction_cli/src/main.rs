use anyhow::Result;
use offline_transaction::Platform;

fn main() -> Result<()> {
    let platform = Platform::from_json("/Users/seigooshima/git/assignment_transaction/test_data/input3.json")?;
    let mut transaction = platform.generate_unsigned_transaction()?;
    transaction.sign_transaction()?;
    let hex_transaction = transaction.get_raw_transaction();
    println!("{}", hex_transaction);
    Ok(())
}
