// TODO: maybe add a scoring system to efficiently get correct ones.
// This is kinda complicated (diff inputs + diff output formats)

pub fn hex_x_hex(a: &str, b: &str) -> String {
    // let a_decoded = a.bytes();
    // let b_decoded = b.bytes();
    let a_decoded = hex::decode(a);
    let b_decoded = hex::decode(b);

    let bruh = a_decoded
        .iter()
        .zip(b_decoded.iter())
        .map(|(x1, x2)| x1 ^ x2)
        .collect::<Vec<u8>>();
    hex::encode(bruh)
    // hex::encode(bruh)
}

pub fn str_x_str(a: &str, b: &str) -> String {
    hex::encode(
        a.chars()
            .zip(b.chars().cycle())
            .map(|(i, j)| ((i as u8) ^ (j as u8)) as char)
            .collect::<String>(),
    )
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
