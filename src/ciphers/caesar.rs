use anyhow::{anyhow, Result};

pub fn rot13(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'a'..='m' | 'A'..='M' => ((c as u8) + 13) as char,
            'n'..='z' | 'N'..='Z' => ((c as u8) - 13) as char,
            _ => c,
        })
        .collect()
}

pub fn caesar(s: &str, shift: Option<u8>) -> Result<String> {
    match shift {
        Some(n) => {
            let result: String = s
                .chars()
                .map(|c| match c {
                    'a'..='z' => (((c as u8 - 97 + n) % 26) + 97) as char,
                    'A'..='Z' => (((c as u8 - 65 + n) % 26) + 65) as char,
                    _ => c,
                })
                .collect();
            Ok(result)
        }
        None => {
            let results: String = (0..26)
                .map(|i| {
                    let result: String = s
                        .chars()
                        .map(|c| match c {
                            'a'..='z' => (((c as u8 - 97 + i) % 26) + 97) as char,
                            'A'..='Z' => (((c as u8 - 65 + i) % 26) + 65) as char,
                            _ => c,
                        })
                        .collect();
                    format!("[{:02}] {}\n", i, result)
                })
                .collect();
            Ok(results.trim().to_string())
        }
    }
}

pub fn vigenere(s: &str, key: &str, decrypt: bool) -> Result<String> {
    let key: String = key
        .chars()
        .filter(|&c| c.is_ascii_alphabetic())
        .collect::<String>()
        .to_ascii_lowercase();

    if key.is_empty() {
        return Err(anyhow!("Key must contain at least one letter"));
    }

    let key_len = key.len();
    let mut index = 0;

    let result: String = s
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let first = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let shift = key.as_bytes()[index % key_len] - b'a';
                index += 1;
                let c_val = c as u8;
                let shifted = if decrypt {
                    (c_val + 26 - first - shift) % 26
                } else {
                    (c_val - first + shift) % 26
                };
                (first + shifted) as char
            } else {
                c
            }
        })
        .collect();

    Ok(result)
}

pub fn rot47(s: &str) -> String {
    s.chars()
        .map(|c| {
            let code = c as u8;
            if (33..=126).contains(&code) {
                ((code - 33 + 47) % 94 + 33) as char
            } else {
                c
            }
        })
        .collect()
}

pub fn rot18(s: &str) -> String {
    rot13(s)
        .chars()
        .map(|c| match c {
            '0'..='4' => ((c as u8) + 5) as char,
            '5'..='9' => ((c as u8) - 5) as char,
            _ => c,
        })
        .collect()
}
