pub mod reader;

pub fn bcd_encode(input: &str, length: usize) -> Vec<u8> {
    let mut bytes = match hex::decode(input) {
        Ok(bytes) => bytes,
        Err(_) => {
            return vec![0; length];
        }
    };
    if length - bytes.len() > 0 {
        bytes.extend(std::iter::repeat(0).take(length - bytes.len()));
    }
    bytes
}

pub fn bcd_decode(input: &[u8]) -> String {
    let index = match input.iter().rposition(|&b| b != 0) {
        Some(index) => index,
        None => return "".to_string(),
    };
    input[..index+1]
        .iter()
        .map(|&b| format!("{:02x}", b))
        .collect()
}
