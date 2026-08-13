use anyhow::{anyhow, Context, Result};
use crate::app::constants::{ALPHABETS, ALT_PHABETS, BACON1, BACON2, DNA1, DNA2};
use crate::utils::utils;
use std::sync::LazyLock;

pub fn url_encode(s: &str) -> Result<String> {
    Ok(urlencoding::encode(s).into())
}

pub fn url_decode(s: &str) -> Result<String> {
    urlencoding::decode(s)
        .map(|s| s.into_owned())
        .context("Failed to URL-decode string")
}

pub fn playfair(s: &str, k: &str, decrypt: bool) -> Result<String> {
    let matrix = build_playfair_matrix(k);
    let digraphs = preprocess_playfair(s);

    let result: String = digraphs
        .iter()
        .map(|pair| {
            if decrypt {
                decrypt_pair(&matrix, pair)
            } else {
                encrypt_pair(&matrix, pair)
            }
        })
        .collect();

    Ok(result)
}

fn build_playfair_matrix(key: &str) -> Vec<Vec<char>> {
    let mut seen: Vec<char> = Vec::new();
    let key: String = key
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| {
            let c = c.to_ascii_lowercase();
            if c == 'j' { 'i' } else { c }
        })
        .collect();

    for c in key.chars() {
        if !seen.contains(&c) {
            seen.push(c);
        }
    }

    for c in 'a'..='z' {
        if c == 'j' {
            continue;
        }
        if !seen.contains(&c) {
            seen.push(c);
        }
    }

    seen.chunks(5).map(|chunk| chunk.to_vec()).collect()
}

fn find_position(matrix: &[Vec<char>], c: char) -> (usize, usize) {
    let c = if c == 'j' { 'i' } else { c };
    for (row, row_vec) in matrix.iter().enumerate() {
        for (col, &val) in row_vec.iter().enumerate() {
            if val == c {
                return (row, col);
            }
        }
    }
    unreachable!()
}

fn preprocess_playfair(s: &str) -> Vec<String> {
    let s: String = s
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| {
            let c = c.to_ascii_lowercase();
            if c == 'j' { 'i' } else { c }
        })
        .collect();

    let mut digraphs: Vec<String> = Vec::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if i + 1 == chars.len() {
            digraphs.push(format!("{}x", chars[i]));
            break;
        }
        if chars[i] == chars[i + 1] {
            digraphs.push(format!("{}x", chars[i]));
            i += 1;
        } else {
            digraphs.push(format!("{}{}", chars[i], chars[i + 1]));
            i += 2;
        }
    }

    digraphs
}

fn encrypt_pair(matrix: &[Vec<char>], pair: &str) -> String {
    let chars: Vec<char> = pair.chars().collect();
    let (r1, c1) = find_position(matrix, chars[0]);
    let (r2, c2) = find_position(matrix, chars[1]);

    if r1 == r2 {
        format!(
            "{}{}",
            matrix[r1][(c1 + 1) % 5],
            matrix[r2][(c2 + 1) % 5],
        )
    } else if c1 == c2 {
        format!(
            "{}{}",
            matrix[(r1 + 1) % 5][c1],
            matrix[(r2 + 1) % 5][c2],
        )
    } else {
        format!("{}{}", matrix[r1][c2], matrix[r2][c1])
    }
}

fn decrypt_pair(matrix: &[Vec<char>], pair: &str) -> String {
    let chars: Vec<char> = pair.chars().collect();
    let (r1, c1) = find_position(matrix, chars[0]);
    let (r2, c2) = find_position(matrix, chars[1]);

    if r1 == r2 {
        format!(
            "{}{}",
            matrix[r1][(c1 + 4) % 5],
            matrix[r2][(c2 + 4) % 5],
        )
    } else if c1 == c2 {
        format!(
            "{}{}",
            matrix[(r1 + 4) % 5][c1],
            matrix[(r2 + 4) % 5][c2],
        )
    } else {
        format!("{}{}", matrix[r1][c2], matrix[r2][c1])
    }
}

pub fn dna(s: &str) -> Result<String> {
    let is_dna = s.chars().all(|c| "ACGTacgt".contains(c));

    if is_dna {
        let mut results = Vec::new();

        let bits: String = s
            .to_uppercase()
            .chars()
            .filter(|c| "ACGT".contains(*c))
            .map(|c| *DNA2.get(&c).unwrap_or(&""))
            .collect();

        if bits.len() >= 8 {
            let padded = format!("{:0<width$}", bits, width = bits.len().div_ceil(8) * 8);
            let decoded: String = padded
                .as_bytes()
                .chunks(8)
                .map(|chunk| {
                    let byte_str = std::str::from_utf8(chunk).unwrap_or("00000000");
                    u8::from_str_radix(byte_str, 2).unwrap_or(0) as char
                })
                .collect();
            results.push(format!(
                "[{}]: {}",
                crate::utils::color::green("2-bit Decode"),
                decoded
            ));
        }

        if s.len() >= 3 {
            let codons: String = s
                .to_uppercase()
                .chars()
                .filter(|c| "ACGT".contains(*c))
                .collect();
            let codon_result: String = codons
                .as_bytes()
                .chunks(3)
                .filter(|chunk| chunk.len() == 3)
                .map(|chunk| {
                    let key = std::str::from_utf8(chunk).unwrap_or("");
                    *DNA1.get(key).unwrap_or(&'?')
                })
                .collect();
            if !codon_result.is_empty() {
                results.push(format!(
                    "[{}]: {}",
                    crate::utils::color::green("Codon Decode"),
                    codon_result
                ));
            }
        }

        if results.is_empty() {
            Ok("(no decode results)".to_string())
        } else {
            Ok(results.join("\n"))
        }
    } else {
        let bits: String = s
            .bytes()
            .flat_map(|b| {
                (0..8)
                    .rev()
                    .map(move |i| if (b >> i) & 1 == 1 { '1' } else { '0' })
            })
            .collect();

        let encoded: String = bits
            .as_bytes()
            .chunks(2)
            .map(|pair| match std::str::from_utf8(pair) {
                Ok("00") => 'A',
                Ok("01") => 'G',
                Ok("10") => 'C',
                Ok("11") => 'T',
                _ => '?',
            })
            .collect();

        Ok(format!(
            "[{}]: {}",
            crate::utils::color::green("DNA Encode"),
            encoded
        ))
    }
}

pub fn railfence(s: &str, rails: Option<usize>) -> Result<String> {
    match rails {
        Some(n) if n > 1 => Ok(railfence_decrypt(s, n)),
        Some(_) => Err(anyhow!("Rail count must be greater than 1")),
        None => {
            eprintln!(
                "{}",
                crate::utils::color::yellow(
                    "Warning: No rail count provided. Brute-forcing all possible counts..."
                )
            );
            let results: Vec<String> = (2..s.len())
                .map(|r| {
                    let decrypted = railfence_decrypt(s, r);
                    format!("[rails={}] {}\n", r, decrypted)
                })
                .collect();
            Ok(results.into_iter().collect::<String>().trim().to_string())
        }
    }
}

fn railfence_decrypt(s: &str, rails: usize) -> String {
    if rails <= 1 || rails >= s.len() {
        return s.to_string();
    }

    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();

    let mut pattern: Vec<Vec<usize>> = vec![Vec::new(); rails];
    let mut row = 0;
    let mut down = true;

    for i in 0..len {
        pattern[row].push(i);
        if down {
            if row == rails - 1 {
                down = false;
                row = row.saturating_sub(1);
            } else {
                row += 1;
            }
        } else if row == 0 {
            down = true;
            row += 1;
        } else {
            row -= 1;
        }
    }

    let mut result = vec![' '; len];
    let mut idx = 0;
    for rail_chars in &pattern {
        for &pos in rail_chars {
            result[pos] = chars[idx];
            idx += 1;
        }
    }

    result.into_iter().collect()
}

pub fn bacon(s: &str) -> Result<String> {
    let clean_str = s.replace("0", "a").replace("1", "b").to_lowercase();

    let str_take1: String = utils::chunkify(clean_str.as_str())
        .chunks(5)
        .map(|word| {
            let idx = BACON1
                .iter()
                .position(|&i| i == String::from_iter(word))
                .unwrap_or(100);
            ALPHABETS.chars().nth(idx).unwrap_or('?')
        })
        .collect();

    let str_take2: String = utils::chunkify(clean_str.as_str())
        .chunks(5)
        .map(|word| {
            let idx = BACON2
                .iter()
                .position(|&i| i == String::from_iter(word))
                .unwrap_or(100);
            ALT_PHABETS.chars().nth(idx).unwrap_or('?')
        })
        .collect();

    Ok(format!("1: {}\n2: {}", str_take1, str_take2))
}

static REV_ALPHABET: LazyLock<String> = LazyLock::new(|| ALPHABETS.chars().rev().collect());

pub fn atbash(s: &str) -> Result<String> {
    Ok(s.to_lowercase()
        .chars()
        .map(|c| {
            let idx = ALPHABETS.find(c).unwrap_or(27);
            REV_ALPHABET.chars().nth(idx).unwrap_or(' ')
        })
        .collect())
}

pub fn ascii(s: &str) -> Result<String> {
    if s.chars().all(|x| "0123456789 ".contains(x)) {
        let result: String = s
            .split_whitespace()
            .map(|n| {
                n.parse::<u8>()
                    .map(|v| v as char)
                    .unwrap_or('?')
            })
            .collect();
        Ok(result)
    } else {
        Ok(s.chars().map(|i| format!("{}", i as u8)).collect())
    }
}

pub fn a1z26(s: &str) -> Result<String> {
    let result: String = s
        .replace("-", " ")
        .split_whitespace()
        .map(|num| {
            let n: usize = num.parse().unwrap_or(0);
            if (1..=26).contains(&n) {
                ALPHABETS.chars().nth(n - 1).unwrap()
            } else {
                '?'
            }
        })
        .collect();
    Ok(result)
}

pub fn affine(s: &str, a: i32, b: i32, decrypt: bool) -> Result<String> {
    if a % 2 == 0 || a % 13 == 0 {
        return Err(anyhow!("Key 'a' must be coprime with 26 (must not be even or divisible by 13)"));
    }

    let mmi = mod_inverse(a, 26).ok_or_else(|| anyhow!("Key 'a' has no modular inverse mod 26"))?;

    let result: String = s
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                let x = (c as u8 - 97) as i32;
                let val = if decrypt {
                    (mmi * (x - b)).rem_euclid(26)
                } else {
                    (a * x + b) % 26
                };
                ((val as u8) + 97) as char
            } else if c.is_ascii_uppercase() {
                let x = (c as u8 - 65) as i32;
                let val = if decrypt {
                    (mmi * (x - b)).rem_euclid(26)
                } else {
                    (a * x + b) % 26
                };
                ((val as u8) + 65) as char
            } else {
                c
            }
        })
        .collect();

    Ok(result)
}

fn mod_inverse(a: i32, m: i32) -> Option<i32> {
    let (mut old_r, mut r) = (a, m);
    let (mut old_s, mut s) = (1i32, 0i32);

    while r != 0 {
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_s, s) = (s, old_s - quotient * s);
    }

    if old_r != 1 {
        return None;
    }

    Some(old_s.rem_euclid(m))
}

pub fn bifid(s: &str, key: &str, decrypt: bool) -> Result<String> {
    let alphabet: Vec<char> = "ABCDEFGHIKLMNOPQRSTUVWXYZ".chars().collect();
    let mut square: Vec<char> = Vec::new();

    for c in key.chars() {
        let c = c.to_ascii_uppercase();
        if c == 'J' {
            continue;
        }
        if c.is_ascii_alphabetic() && !square.contains(&c) {
            square.push(c);
        }
    }

    for &c in &alphabet {
        if !square.contains(&c) {
            square.push(c);
        }
    }

    let s: String = s
        .to_uppercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| if c == 'J' { 'I' } else { c })
        .collect();

    let mut rows: Vec<usize> = Vec::new();
    let mut cols: Vec<usize> = Vec::new();

    for c in s.chars() {
        if let Some(pos) = square.iter().position(|&x| x == c) {
            rows.push(pos / 5);
            cols.push(pos % 5);
        }
    }

    if decrypt {
        let combined: Vec<usize> = [rows, cols].concat();
        let mid = combined.len() / 2;
        let new_rows = &combined[..mid];
        let new_cols = &combined[mid..];

        let result: String = new_rows
            .iter()
            .zip(new_cols.iter())
            .map(|(&r, &c)| square[r * 5 + c])
            .collect();
        Ok(result)
    } else {
        let combined: Vec<usize> = [rows, cols].concat();
        let mid = combined.len() / 2;
        let new_rows = &combined[..mid];
        let new_cols = &combined[mid..];

        let result: String = new_rows
            .iter()
            .zip(new_cols.iter())
            .map(|(&r, &c)| square[r * 5 + c])
            .collect();
        Ok(result)
    }
}

pub fn substitution(s: &str, key: &str, decrypt: bool) -> Result<String> {
    if key.len() < 26 {
        return Err(anyhow!("Key must be at least 26 characters long"));
    }

    let key: Vec<char> = key
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    if key.len() < 26 {
        return Err(anyhow!("Key must contain at least 26 letters"));
    }

    if decrypt {
        let mut reverse = ['?'; 26];
        for (i, &c) in key.iter().enumerate() {
            if c.is_ascii_lowercase() {
                reverse[(c as u8 - b'a') as usize] = (b'a' + i as u8) as char;
            }
        }
        let result: String = s
            .chars()
            .map(|c| {
                if c.is_ascii_lowercase() {
                    reverse[(c as u8 - b'a') as usize]
                } else if c.is_ascii_uppercase() {
                    let r = reverse[(c as u8 - b'A') as usize];
                    r.to_ascii_uppercase()
                } else {
                    c
                }
            })
            .collect();
        Ok(result)
    } else {
        let result: String = s
            .chars()
            .map(|c| {
                if c.is_ascii_lowercase() {
                    key[(c as u8 - b'a') as usize]
                } else if c.is_ascii_uppercase() {
                    key[(c as u8 - b'A') as usize].to_ascii_uppercase()
                } else {
                    c
                }
            })
            .collect();
        Ok(result)
    }
}

static KEYBOARD_ROWS: [&str; 4] = [
    "1234567890-=",
    "qwertyuiop[]\\",
    "asdfghjkl;'",
    "zxcvbnm,./",
];

fn keyboard_neighbors() -> std::collections::HashMap<char, (Option<char>, Option<char>)> {
    let mut map = std::collections::HashMap::new();
    for row in KEYBOARD_ROWS.iter() {
        let chars: Vec<char> = row.chars().collect();
        for (i, &c) in chars.iter().enumerate() {
            let left = if i > 0 { Some(chars[i - 1]) } else { None };
            let right = chars.get(i + 1).copied();
            map.insert(c, (left, right));
        }
    }
    map
}

pub fn keyboard(s: &str, dir: Option<i8>) -> Result<String> {
    if let Some(d) = dir {
        if d.abs() != 1 {
            return Err(anyhow!("--dir must be 1 (move keys left) or -1 (move keys right)"));
        }
    }
    let map = keyboard_neighbors();
    let apply = |d: i8| -> String {
        s.chars()
            .map(|c| {
                let lower = c.to_ascii_lowercase();
                match map.get(&lower) {
                    Some(&(left, right)) => {
                        let shifted = if d > 0 {
                            left.unwrap_or(lower)
                        } else {
                            right.unwrap_or(lower)
                        };
                        if c.is_ascii_uppercase() {
                            shifted.to_ascii_uppercase()
                        } else {
                            shifted
                        }
                    }
                    None => c,
                }
            })
            .collect()
    };
    match dir {
        Some(d) => Ok(apply(d)),
        None => Ok(format!(
            "[{}]\n{}\n\n[{}]\n{}",
            crate::utils::color::cyan("hands shifted right -> move each key left"),
            apply(1),
            crate::utils::color::cyan("hands shifted left -> move each key right"),
            apply(-1)
        )),
    }
}
