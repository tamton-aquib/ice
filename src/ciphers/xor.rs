use anyhow::{Context, Result};

pub fn hex_x_hex(a: &str, b: &str) -> Result<String> {
    let a_decoded = hex::decode(a).context("first argument is not a valid hex string")?;
    let b_decoded = hex::decode(b).context("second argument is not a valid hex string")?;

    let result: Vec<u8> = a_decoded
        .iter()
        .zip(b_decoded.iter().cycle())
        .map(|(x1, x2)| x1 ^ x2)
        .collect();

    Ok(hex::encode(result))
}

pub fn str_x_str(a: &str, b: &str) -> Result<String> {
    let result: Vec<u8> = a
        .bytes()
        .zip(b.bytes().cycle())
        .map(|(i, j)| i ^ j)
        .collect();

    Ok(hex::encode(result))
}

pub fn str_x_byte(s: &str) -> Result<String> {
    hex_x_byte_inner(&hex::encode(s))
}

pub fn hex_x_byte(s: &str) -> Result<String> {
    hex_x_byte_inner(s)
}

fn hex_x_byte_inner(s: &str) -> Result<String> {
    let decoded = hex::decode(s).context("Invalid hex string")?;

    let results: Vec<String> = (0..=255)
        .filter_map(|i| {
            let bytes: Vec<u8> = decoded.iter().map(|b| b ^ i).collect();
            if bytes.iter().all(|&b| b.is_ascii_graphic() || b == b' ') {
                String::from_utf8(bytes).ok()
            } else {
                None
            }
        })
        .collect();

    Ok(results.join("\n"))
}
