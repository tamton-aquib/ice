// TODO: maybe add a scoring system to efficiently get correct ones.

/// XOR between a string and another string.
/// Works on strings or bytes, returns `String`.
/// * `a`: &str (first string)
/// * `b`: &str (second string)
pub fn str_x_str(a: &str, b: &str) -> String {
    if b > a {
        let (_a, _b) = (b, a);
    }

    a.chars()
        .zip(b.chars().cycle())
        .map(|(i, j)| ((i as u8) ^ (j as u8)) as char)
        .collect()
}

// TODO: not working for some reason
/// Single byte xoring.
/// Tries on bytes 0-255
/// * `s`: &str (query string)
pub fn str_x_byte(s: &str) -> String {
    (0..=255)
        .filter(|i| {
            let ans = hex::decode(s)
                .unwrap()
                .iter()
                .map(|b| (b ^ i) as char)
                .collect::<String>();
            ans.is_ascii()
        })
        .map(|n| n as char)
        .collect()
}

// TODO: to be completed!
/// XOR between 2 hex strings.
/// Decodes and performs str_x_str.
/// * `a`: &str(hex)
/// * `b`: &str(hex)
pub fn hex_x_hex(_a: &str, _b: &str) -> String {
    unimplemented!();
}
