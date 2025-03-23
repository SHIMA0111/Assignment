pub(super) fn estimate_tx_size(input_count: usize, 
                               output_count: usize, 
                               has_segwit: bool) -> u64 {
    
    // version(4) + input(1) + output(1) + lock_time(4)
    let mut size = 10_u64;
    
    if has_segwit {
        size += 2;
    }
    
    // Input size
    // Legacy input: About 148 bytes
    // Segwit input: About non-witness 41 bytes and witness about 68 bytes
    size += if has_segwit {
        input_count as u64 * (41 + 68)
    } else {
        input_count as u64 * 148
    };
    
    // Output size: About 34 bytes
    size += output_count as u64 * 34;
    
    size
}