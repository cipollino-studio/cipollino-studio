
pub fn rmpv_get<'v>(value: &'v rmpv::Value, key: &str) -> Option<&'v rmpv::Value> {
    let map = value.as_map()?;
    for (map_key, val) in map {
        if map_key.as_str() == Some(key) {
            return Some(val);
        }
    } 
    None
}

pub fn rmpv_encode(data: &rmpv::Value) -> Option<Vec<u8>> {
    let mut bytes = Vec::new();
    rmpv::encode::write_value(&mut bytes, data).ok()?;
    Some(bytes)
}

pub fn rmpv_decode(mut bytes: &[u8]) -> Option<rmpv::Value> {
    rmpv::decode::read_value(&mut bytes).ok()
}