use twox_hash::XxHash3_128;
pub fn floor_div(a: i128, b: i128) -> i128 {
    let (q, r) = (a / b, a % b);
    if (r != 0) && ((r > 0) != (b > 0)) {
        q - 1
    } else {
        q
    }
}

fn compute_xyza_hash(x: i128, y: i128, z: i128, a: i32) -> u128 {
    let mut bytes = [0u8; 52];  
    
    bytes[0..16].copy_from_slice(&x.to_le_bytes());
    bytes[16..32].copy_from_slice(&y.to_le_bytes());
    bytes[32..48].copy_from_slice(&z.to_le_bytes());
    bytes[48..52].copy_from_slice(&a.to_le_bytes());
    
    XxHash3_128::oneshot(&bytes)
}

fn compute_account_item_hash(account_id: &str, item_type: i32) -> u128 {
    let account_bytes = account_id.as_bytes();
    let item_type_bytes = item_type.to_le_bytes();
    let mut bytes = Vec::with_capacity(account_bytes.len() + 4);
    bytes.extend_from_slice(account_bytes);
    bytes.extend_from_slice(&item_type_bytes);
    XxHash3_128::oneshot(&bytes)
}

fn compute_consumed_key(x: i128, y: i128, z: i128, a: i32) -> u128 {
    let lootable_blocks_density = 0.1;
    let looted_blocks_estimate = 0.5;
    let actually_looted_avg = lootable_blocks_density / looted_blocks_estimate
    let bloom_filter_tolerable_fp = 0.1;

    let mut bytes = [0u8; 52]

    let region_size: i128 = 64; // ~ 4KB size per bloom filter
    let region_x = floor_div(x, region_size);
    let region_y = floor_div(y, region_size);
    let region_z = floor_div(z, region_size);

    bytes[0..16].copy_from_slice(&region_x.to_le_bytes());
    bytes[16..32].copy_from_slice(&region_y.to_le_bytes());
    bytes[32..48].copy_from_slice(&region_z.to_le_bytes());
    bytes[48..52].copy_from_slice(&a.to_le_bytes());

    XxHash3_128::oneshot(&bytes)
}
