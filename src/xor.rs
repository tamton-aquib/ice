// TODO: maybe add a scoring system to efficiently get correct ones.
// This is kinda complicated (diff inputs + diff output formats)

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

// This is the only func here that returns ascii strings
pub fn str_x_byte(s: &str) -> String {
    (0..=255)
        .map(|i| {
            format!(
                "{}\n",
                hex::decode(s)
                    .expect("Hex cant be decoded!")
                    .iter()
                    .map(|b| (b ^ i) as char)
                    .collect::<String>(),
            )
        })
        .filter(|i| i.is_ascii())
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
