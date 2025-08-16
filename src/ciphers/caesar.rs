/// Classic rot13.
/// Returns a single `String`.
/// * `query`: &str (query string)
pub fn rot13(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'a'..='m' | 'A'..='M' => ((c as u8) + 13) as char,
            'n'..='z' | 'N'..='Z' => ((c as u8) - 13) as char,
            _ => c,
        })
        .collect()
}

/// Caesar cipher for 0-25(a-z)
/// * `query`: &str (query string)
pub fn caesar(s: &str) -> String {
    (0..26)
        .map(|i| {
            let result = s.chars().map(|c| match c {
                'a'..='z' => (((c as u8 - 97 + i) % 26) + 97) as char,
                'A'..='Z' => (((c as u8 - 65 + i) % 26) + 65) as char,
                _ => c,
            });

            format!("[{:02}] {}\n", i, result.collect::<String>())
        })
        .collect()
}

// NOTE: taken from https://github.com/TheAlgorithms/Rust
/// Vigenere cipher encrypt.
/// Returns encrypted string.
/// * `s`: &str (query string)
/// * `key`: &str (key to encrypt)
pub fn vigenere(s: &str, key: &str) -> String {
    let key = key
        .chars()
        .filter(|&c| c.is_ascii_alphabetic())
        .collect::<String>()
        .to_ascii_lowercase();

    let key_len = key.len();
    let mut index = 0;

    s.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let first = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let shift = key.as_bytes()[index % key_len] - b'a';
                index += 1;

                (first + (c as u8 + shift - first) % 26) as char
            } else {
                c
            }
        })
        .collect()
}
