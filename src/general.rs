// TODO: mostly sub ciphers.
const ALPHABETS: &str = "abcdefghijklmnopqrstuvwxyz";
const ALT_PHABETS: &str = "abcdefghiklmnopqrstuwxyz";

pub fn railfence(s: &str) -> String {
    "noice".to_owned()
}

// TODO: Cleanify
pub fn bacon(s: &str) -> String {
    let take1 = [
        "aaaaa", "aaaab", "aaaba", "aaabb", "aabaa", "aabab", "aabba", "aabbb", "abaaa", "abaab",
        "ababa", "ababb", "abbaa", "abbab", "abbba", "abbbb", "baaaa", "baaab", "baaba", "baabb",
        "babaa", "babab", "babba", "babbb",
    ];

    let take2 = [
        "aaaaa", "aaaab", "aaaba", "aaabb", "aabaa", "aabab", "aabba", "aabbb", "abaaa", "abaab",
        "ababa", "ababb", "abbaa", "abbab", "abbba", "abbbb", "baaaa", "baaab", "baaba", "baabb",
        "babaa", "babab", "babba", "babbb", "bbaaa", "bbaab",
    ];

    let string = s.to_lowercase().replace("0", "a").replace("1", "b");
    let str_take1: String = string
        .split_whitespace()
        .map(|word| {
            word.chars()
                .collect::<Vec<char>>()
                .chunks(5)
                .map(|c| {
                    let idx = take1
                        .iter()
                        .position(|&i| i == String::from_iter(c))
                        .unwrap();
                    ALPHABETS.chars().nth(idx).unwrap()
                })
                .collect::<String>()
        })
        .collect();
    let str_take2: String = string
        .split_whitespace()
        .map(|word| {
            word.chars()
                .collect::<Vec<char>>()
                .chunks(5)
                .map(|c| {
                    let idx = take2
                        .iter()
                        .position(|&i| i == String::from_iter(c))
                        .unwrap();
                    ALT_PHABETS.chars().nth(idx).unwrap()
                })
                .collect::<String>()
        })
        .collect();

    format!("{}\n{}", str_take1, str_take2)
}

// TODO: cleanup maybe
pub fn atbash(s: &str) -> String {
    s.chars()
        .map(|c| {
            let idx = ALPHABETS.find(c).unwrap_or(27);
            match ALPHABETS.chars().rev().nth(idx) {
                Some(v) => v,
                None => ' ',
            }
        })
        .collect()
}

pub fn ascii(s: &str) -> String {
    if s.chars().all(|x| "0123456789 ".contains(x)) {
        s.split_whitespace()
            .map(|n| n.parse::<u8>().expect("Failed when parsing to range 0-255") as char)
            .collect()
    } else {
        s.chars().map(|i| format!("{}", i as u8)).collect()
    }
}

pub fn a1z26(s: &str) -> String {
    s.replace("-", " ")
        .split_whitespace()
        .map(|num| {
            ALPHABETS
                .chars()
                .nth(num.parse::<u8>().unwrap() as usize - 1)
                .unwrap()
        })
        .collect()
}
