use anyhow::{Context, Result};
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
        decode_or_encode(s, |s| base64::decode(s).ok(), |s| base64::encode(s))
    } else {
        Ok(base64::encode(s))
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

fn b62_decode(s: &str) -> Option<Vec<u8>> {
    let mut num = 0u128;
    for c in s.bytes() {
        let val = match c {
            b'0'..=b'9' => (c - b'0') as u128,
            b'A'..=b'Z' => (c - b'A' + 10) as u128,
            b'a'..=b'z' => (c - b'a' + 36) as u128,
            _ => return None,
        };
        num = num.checked_mul(62)?;
        num = num.checked_add(val)?;
    }
    if num == 0 {
        return Some(vec![0]);
    }
    let mut result = Vec::new();
    while num > 0 {
        result.push((num % 256) as u8);
        num /= 256;
    }
    result.reverse();
    Some(result)
}

pub fn b62(s: &str) -> Result<String> {
    let s = s.trim();
    let looks_like_b62 = !s.is_empty() && s.chars().all(is_b62_char);
    if looks_like_b62 {
        decode_or_encode(s, b62_decode, |s| {
            let bytes = s.as_bytes();
            let mut num = 0u128;
            for &b in bytes {
                num = num * 256 + b as u128;
            }
            if num == 0 {
                return "0".to_string();
            }
            const CHARSET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
            let mut result = Vec::new();
            while num > 0 {
                result.push(CHARSET[(num % 62) as usize]);
                num /= 62;
            }
            result.reverse();
            String::from_utf8(result).unwrap()
        })
    } else {
        Ok(b62_encode(s))
    }
}

fn b62_encode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut num = 0u128;
    for &b in bytes {
        num = num * 256 + b as u128;
    }
    if num == 0 {
        return "0".to_string();
    }
    const CHARSET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let mut result = Vec::new();
    while num > 0 {
        result.push(CHARSET[(num % 62) as usize]);
        num /= 62;
    }
    result.reverse();
    String::from_utf8(result).unwrap()
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
            for i in (0..n).rev() {
                encoded[i] = char::from_u32((val % 85) + 33).unwrap();
                let _ = val / 85;
            }
            // Recalculate properly
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
        Ok(s.chars()
            .fold(String::new(), |acc, i| format!("{acc}{:x} ", u32::from(i)))
            .trim()
            .to_string())
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
        Ok(s.chars()
            .fold(String::new(), |acc, i| {
                format!("{acc}{:03o} ", u32::from(i))
            })
            .trim()
            .to_string())
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
        Ok(s.chars()
            .fold(String::new(), |acc, i| {
                format!("{acc}{:08b} ", u32::from(i))
            })
            .trim()
            .to_string())
    }
}
