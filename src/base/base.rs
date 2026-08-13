use anyhow::{Context, Result};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use crate::utils::utils;

fn is_b64_char(c: char) -> bool {
    matches!(c, 'A'..='Z' | 'a'..='z' | '0'..='9' | '+' | '/' | '=' | ' ')
}

fn is_b32_char(c: char) -> bool {
    matches!(c, 'A'..='Z' | 'a'..='z' | '2'..='7' | '=' | ' ')
}

fn decode_or_encode<D>(s: &str, decode: D, encode: fn(&str) -> String) -> Result<String>
where
    D: FnOnce(&str) -> Option<Vec<u8>>,
{
    match decode(s.trim()) {
        Some(n) => {
            let decoded = String::from_utf8_lossy(&n);
            let result = decoded.into_owned();
            if !result.contains('\u{FFFD}') {
                return Ok(result);
            }
            Ok(encode(s))
        }
        None => Ok(encode(s)),
    }
}

pub fn b64(s: &str) -> Result<String> {
    let looks_like_b64 = !s.trim().is_empty() && s.trim().chars().all(is_b64_char);
    if looks_like_b64 {
        decode_or_encode(s, |s| STANDARD.decode(s).ok(), |s| STANDARD.encode(s))
    } else {
        Ok(STANDARD.encode(s))
    }
}

pub fn b32(s: &str) -> Result<String> {
    let looks_like_b32 = !s.trim().is_empty() && s.trim().chars().all(is_b32_char);
    if looks_like_b32 {
        decode_or_encode(s, |s| base32::decode(base32::Alphabet::RFC4648 { padding: true }, s), |s| {
            base32::encode(base32::Alphabet::RFC4648 { padding: true }, s.as_bytes())
        })
    } else {
        Ok(base32::encode(
            base32::Alphabet::RFC4648 { padding: true },
            s.as_bytes(),
        ))
    }
}

pub fn b45(s: &str) -> Result<String> {
    let s = s.trim();
    let looks_like = !s.is_empty() && s.chars().all(is_b45_char);
    if looks_like {
        decode_or_encode(s, |s| base45::decode(s).ok(), |s| base45::encode(s.as_bytes()))
    } else {
        Ok(base45::encode(s.as_bytes()))
    }
}

pub fn b58(s: &str) -> Result<String> {
    let s = s.trim();
    let looks_like = !s.is_empty() && s.chars().all(is_b58_char);
    if looks_like {
        decode_or_encode(s, |s| bs58::decode(s).into_vec().ok(), |s| bs58::encode(s).into_string())
    } else {
        Ok(bs58::encode(s).into_string())
    }
}

fn is_b45_char(c: char) -> bool {
    matches!(c, '0'..='9' | 'A'..='Z' | ' ' | '$' | '%' | '*' | '+' | '-' | '.' | '/' | ':')
}

fn is_b58_char(c: char) -> bool {
    matches!(c, '1'..='9' | 'A'..='Z' | 'a'..='z')
}

fn is_b62_char(c: char) -> bool {
    matches!(c, '0'..='9' | 'A'..='Z' | 'a'..='z')
}

fn b62_decode_str(s: &str) -> Option<Vec<u8>> {
    let s = s.trim();
    if s.is_empty() {
        return Some(Vec::new());
    }
    let mut out = Vec::with_capacity(s.len() * 7 / 10 + 1);
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let group_len = (bytes.len() - i).min(10);
        let byte_len = match group_len {
            2 => 1,
            3 => 2,
            5 => 3,
            6 => 4,
            7 => 5,
            9 => 6,
            10 => 7,
            _ => return None,
        };
        let mut val: u64 = 0;
        for &b in &bytes[i..i + group_len] {
            val = match b {
                b'0'..=b'9' => val * 62 + (b - b'0') as u64,
                b'A'..=b'Z' => val * 62 + (b - b'A' + 10) as u64,
                b'a'..=b'z' => val * 62 + (b - b'a' + 36) as u64,
                _ => return None,
            };
        }
        if byte_len < 8 && val >= (1u64 << (byte_len * 8)) {
            return None;
        }
        let be = val.to_be_bytes();
        out.extend_from_slice(&be[8 - byte_len..]);
        i += group_len;
    }
    Some(out)
}

const B62_CHARSET: &[u8] =
    b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

fn b62_width(bytes: usize) -> usize {
    match bytes {
        1 => 2,
        2 => 3,
        3 => 5,
        4 => 6,
        5 => 7,
        6 => 9,
        7 => 10,
        _ => 0,
    }
}

fn b62_encode_bytes(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return String::new();
    }
    let mut out = String::with_capacity(bytes.len() + bytes.len() / 2);
    for chunk in bytes.chunks(7) {
        let mut val: u64 = 0;
        for &b in chunk {
            val = val * 256 + b as u64;
        }
        let width = b62_width(chunk.len());
        let mut digits = vec![b'0'; width];
        for i in (0..width).rev() {
            digits[i] = B62_CHARSET[(val % 62) as usize];
            val /= 62;
        }
        out.push_str(std::str::from_utf8(&digits).unwrap());
    }
    out
}

fn b62_encode_str(s: &str) -> String {
    b62_encode_bytes(s.as_bytes())
}

pub fn b62(s: &str) -> Result<String> {
    let s = s.trim();
    let looks_like_b62 = !s.is_empty() && s.chars().all(is_b62_char);
    if looks_like_b62 {
        decode_or_encode(s, b62_decode_str, b62_encode_str)
    } else {
        Ok(b62_encode_str(s))
    }
}

pub fn b85(s: &str) -> Result<String> {
    let is_b85 = s.starts_with("<~") && s.ends_with("~>");

    if is_b85 {
        let inner = &s[2..s.len() - 2];
        let inner = inner.replace(" ", "").replace("\n", "").replace("\r", "");
        let decoded = ascii85_decode(&inner)?;
        Ok(String::from_utf8_lossy(&decoded).into_owned())
    } else {
        let encoded = ascii85_encode(s.as_bytes());
        Ok(format!("<~{}~>", encoded))
    }
}

fn ascii85_encode(data: &[u8]) -> String {
    let mut result = String::new();
    for chunk in data.chunks(4) {
        let mut buf = [0u8; 4];
        for (i, &b) in chunk.iter().enumerate() {
            buf[i] = b;
        }
        let val = u32::from_be_bytes(buf);
        if val == 0 && chunk.len() == 4 {
            result.push('z');
        } else {
            let mut encoded = ['!'; 5];
            let n = chunk.len() + 1;
            let mut v = val;
            for i in (0..5).rev() {
                encoded[i] = char::from_u32((v % 85) + 33).unwrap();
                v /= 85;
            }
            for &c in &encoded[..n] {
                result.push(c);
            }
        }
    }
    result
}

fn ascii85_decode(data: &str) -> Result<Vec<u8>> {
    let mut result = Vec::new();
    let mut buf = Vec::new();

    for c in data.chars() {
        if c == 'z' {
            if !buf.is_empty() {
                return Err(anyhow::anyhow!("Invalid 'z' in Ascii85 data"));
            }
            result.extend_from_slice(&[0, 0, 0, 0]);
        } else if c == '~' {
            break;
        } else if c.is_ascii_whitespace() {
            continue;
        } else {
            let val = (c as u8).wrapping_sub(33);
            if val > 84 {
                return Err(anyhow::anyhow!("Invalid Ascii85 character: {}", c));
            }
            buf.push(val);
            if buf.len() == 5 {
                let num = buf.iter().fold(0u32, |acc, &v| acc * 85 + v as u32);
                result.extend_from_slice(&num.to_be_bytes());
                buf.clear();
            }
        }
    }

    if !buf.is_empty() {
        while buf.len() < 5 {
            buf.push(84);
        }
        let num = buf.iter().fold(0u32, |acc, &v| acc * 85 + v as u32);
        let bytes = num.to_be_bytes();
        let n = buf.len() - 1;
        result.extend_from_slice(&bytes[..n]);
    }

    Ok(result)
}

pub fn hexadecimal(s: &str) -> Result<String> {
    if utils::is_hex_repr(s) {
        let trimmed = s.trim();
        let bytes = hex::decode(trimmed).context("Invalid hex string")?;
        Ok(String::from_utf8_lossy(&bytes).into_owned())
    } else {
        let mut out = String::with_capacity(s.len() * 3);
        for c in s.chars() {
            out.push_str(&format!("{:x} ", u32::from(c)));
        }
        Ok(out.trim_end().to_string())
    }
}

pub fn octal(s: &str) -> Result<String> {
    if s.chars().all(|x| "01234567 ".contains(x)) {
        let chars: String = s
            .split_whitespace()
            .map(|c| {
                u8::from_str_radix(c, 8)
                    .map(|v| v as char)
                    .unwrap_or('?')
            })
            .collect();
        Ok(chars)
    } else {
        let mut out = String::with_capacity(s.len() * 4);
        for c in s.chars() {
            out.push_str(&format!("{:03o} ", u32::from(c)));
        }
        Ok(out.trim_end().to_string())
    }
}

pub fn binary(s: &str) -> Result<String> {
    if s.chars().all(|x| ['0', '1', ' '].contains(&x)) {
        let chars: String = utils::chunkify(s)
            .chunks(8)
            .map(|c| {
                u8::from_str_radix(&String::from_iter(c), 2)
                    .map(|v| v as char)
                    .unwrap_or('?')
            })
            .collect();
        Ok(chars)
    } else {
        let mut out = String::with_capacity(s.len() * 9);
        for c in s.chars() {
            out.push_str(&format!("{:08b} ", u32::from(c)));
        }
        Ok(out.trim_end().to_string())
    }
}
