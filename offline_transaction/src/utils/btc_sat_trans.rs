/// Converts Bitcoin (BTC) amount to Satoshis
///
/// # Arguments
///
/// * `btc` - Amount in Bitcoin to convert
///
/// # Returns
///
/// * Number of Satoshis as u64 (1 BTC = 100,000,000 Satoshis)
pub(crate) fn btc_to_sat(btc: f64) -> u64 {
    (btc * 100_000_000.0) as u64
}
