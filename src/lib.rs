// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    // todo!()
    if raw_tx_hex.len() < 8 {
        return Err("Transaction data too short".to_string());
    }
    match hex::decode(&raw_tx_hex[..8]) {
        Ok(bytes) => {
            if bytes.len() < 4 {
                return Err("Decoded bytes are too short".to_string());
            }
            let version = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
            Ok(version)
        }
        Err(_) => Err("Hex decode error".to_string()),
    }
}
