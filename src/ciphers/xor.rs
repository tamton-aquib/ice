// TODO: maybe add a scoring system to efficiently get correct ones.
// This is kinda complicated (diff inputs + diff output formats)

// TODO: does not work
pub fn hex_x_hex(a: &str, b: &str) -> String {
    let a_decoded = hex::decode(a).expect("first argument is not a valid hex!");
    let b_decoded = hex::decode(b).expect("second argument is not a valid hex!");

    hex::encode(
        a_decoded
            .iter()
            .zip(b_decoded.iter().cycle())
            .map(|(x1, x2)| (x1 ^ x2) as char)
            .collect::<String>(),
    )
}

// TODO: make hxh work to make this work.
pub fn str_x_str(a: &str, b: &str) -> String {
    hex::encode(
        a.chars()
            .zip(b.chars().cycle())
            .map(|(i, j)| ((i as u8) ^ (j as u8)) as char)
            .collect::<String>(),
    )
}

// HACK: Shouldve called the other way around!
pub fn str_x_byte(s: &str) -> String {
    hex_x_byte(&hex::encode(s))
}

pub fn hex_x_byte(s: &str) -> String {
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
        .filter(|i| i.is_ascii() && !i.trim().is_empty())
        .collect()
}
