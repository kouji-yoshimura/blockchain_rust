pub fn encode_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|n| format!("{:02X}", n)).collect::<String>()
}

pub fn decode_hex(hex: String) -> Vec<u8> {
    (0..hex.as_str().len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect::<Vec<u8>>()
}
