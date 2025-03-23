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


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_btc_to_sat() {
        assert_eq!(btc_to_sat(1.0), 100_000_000);
    }
    
    #[test]
    fn test_btc_to_sat_2() {
        assert_eq!(btc_to_sat(0.00000001), 1);
    }
    
    #[test]
    fn test_btc_to_sat_3() {
        assert_eq!(btc_to_sat(0.15), 15_000_000);
    }
}
