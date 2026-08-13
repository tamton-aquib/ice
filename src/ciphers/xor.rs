use anyhow::{anyhow, Context, Result};

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

pub fn key_x_byte(s: &str) -> Result<String> {
    let trimmed: String = s.split_whitespace().collect();
    let bytes: Vec<u8> = if !trimmed.is_empty()
        && trimmed.len().is_multiple_of(2)
        && trimmed.chars().all(|c| c.is_ascii_hexdigit())
    {
        hex::decode(&trimmed).context("Invalid hex string")?
    } else {
        s.as_bytes().to_vec()
    };

    let max_ks = (40).min(bytes.len() / 4);
    if max_ks < 2 {
        return Err(anyhow!("Input too short for keysize analysis"));
    }

    let mut scored: Vec<(f64, usize)> = (2..=max_ks)
        .map(|ks| {
            let mut dist = 0.0f64;
            let mut pairs = 0usize;
            for chunk in bytes.chunks(ks).collect::<Vec<_>>().windows(2) {
                if chunk[0].len() == ks && chunk[1].len() == ks {
                    dist += hamming(chunk[0], chunk[1]) as f64 / ks as f64;
                    pairs += 1;
                }
            }
            (if pairs > 0 { dist / pairs as f64 } else { f64::MAX }, ks)
        })
        .collect();
    scored.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

    let mut results: Vec<String> = Vec::new();
    for (_, ks) in scored.iter().take(5) {
        let ks = *ks;
        let mut key = Vec::with_capacity(ks);
        for pos in 0..ks {
            let column: Vec<u8> = bytes.iter().skip(pos).step_by(ks).copied().collect();
            let mut best = (f64::NEG_INFINITY, 0u8);
            for k in 0..=255u8 {
                let dec: Vec<u8> = column.iter().map(|&b| b ^ k).collect();
                let sc = english_score(&dec);
                if sc > best.0 {
                    best = (sc, k);
                }
            }
            key.push(best.1);
        }
        let plain: Vec<u8> = bytes
            .iter()
            .zip(key.iter().cycle())
            .map(|(&b, &k)| b ^ k)
            .collect();
        let key_str: String = key.iter().map(|&b| b as char).collect();
        let text = String::from_utf8_lossy(&plain);
        results.push(format!(
            "[score={:.1} {}] key=\"{}\"\n{}",
            english_score(&plain),
            crate::utils::color::cyan(&format!("keysize={}", ks)),
            key_str,
            text
        ));
    }
    Ok(results.join("\n\n"))
}

fn hamming(a: &[u8], b: &[u8]) -> u32 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x ^ y).count_ones())
        .sum()
}

fn english_score(bytes: &[u8]) -> f64 {
    let mut score = 0.0f64;
    for &b in bytes {
        if b.is_ascii_alphabetic() {
            let c = b.to_ascii_lowercase();
            score += match c {
                b'e' => 12.0,
                b't' => 9.1,
                b'a' => 8.2,
                b'o' => 7.5,
                b'i' => 7.0,
                b'n' => 6.7,
                b's' => 6.3,
                b'h' => 6.1,
                b'r' => 6.0,
                b'd' => 4.3,
                b'l' => 4.0,
                b'u' => 2.8,
                b'c' => 2.8,
                b'm' => 2.4,
                b'w' => 2.4,
                b'f' => 2.2,
                b'g' => 2.0,
                b'y' => 2.0,
                b'p' => 1.9,
                b'b' => 1.5,
                b'v' => 0.98,
                b'k' => 0.77,
                b'j' => 0.15,
                b'x' => 0.15,
                b'q' => 0.095,
                b'z' => 0.074,
                _ => 0.0,
            };
        } else if b == b' ' {
            score += 2.5;
        } else if b.is_ascii_control() || b == 0x7f {
            score -= 10.0;
        } else if !b.is_ascii_alphanumeric() {
            score -= 2.0;
        }
    }
    score / bytes.len().max(1) as f64
}
