/// Classic rot13.
/// Returns a single `String`.
/// * `query`: &str (query string)
pub fn rot13(query: &str) -> String {
    query
        .chars()
        .map(|c| match c {
            'a'..='m' | 'A'..='M' => ((c as u8) + 13) as char,
            'n'..='z' | 'N'..='Z' => ((c as u8) - 13) as char,
            _ => c,
        })
        .collect::<String>()
}

/// Caesar cipher for 0-25(a-z)
/// Returns a vector of Strings (25 no.)
/// * `a`: &str (query string)
pub fn caesar(query: &str) -> Vec<String> {
    (0..26)
        .map(|i| {
            let result = query.chars().map(|c| match c {
                'a'..='z' => (((c as u8 - 97 + i) % 26) + 97) as char,
                'A'..='Z' => (((c as u8 - 65 + i) % 26) + 65) as char,
                _ => c,
            });

            format!("[{:02}] {}", i, result.collect::<String>())
        })
        .collect::<Vec<String>>()
}

// TODO: to be completed!
/// Vigenere cipher encrypt.
/// Returns encrypted string.
/// * `s`: &str (query string)
/// * `key`: &str (key to encrypt)
pub fn vigenere(s: &str, key: &str) -> String {
    s.chars()
        .zip(key.chars().cycle())
        .map(|(x, y)| (((x as u8) + (y as u8)) % 26) as char)
        .collect::<String>()
}
