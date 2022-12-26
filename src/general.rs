use crate::constants::{ALPHABETS, ALT_PHABETS, BACON1, BACON2, DNA};

// TODO: on the way, does not work.
pub fn dna(s: &str) -> String {
    let mappings = &DNA;

    s.split_whitespace()
        .collect::<String>()
        .chars()
        .collect::<Vec<char>>()
        .chunks(3)
        .map(|i| mappings.get(&*String::from_iter(i)).unwrap())
        .collect()
}

// NOTE: on the way, doesnt work now.
pub fn railfence(s: &str) -> String {
    let news: String = s.to_lowercase().split_whitespace().collect();
    let nice = news
        .chars()
        .collect::<Vec<char>>()
        .chunks(3)
        .map(|i| i.iter().collect::<String>())
        .collect::<Vec<String>>();
    println!("{nice:?}");
    "noice".to_owned()
}

// TODO: Cleanify
pub fn bacon(s: &str) -> String {
    let string = s.to_lowercase().replace("0", "a").replace("1", "b");
    let str_take1: String = string
        .split_whitespace()
        .map(|word| {
            word.chars()
                .collect::<Vec<char>>()
                .chunks(5)
                .map(|c| {
                    let idx = BACON1
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
                    let idx = BACON2
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
