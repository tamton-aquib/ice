use crate::app::constants::{ALPHABETS, ALT_PHABETS, BACON1, BACON2, DNA1, DNA2};
use crate::base::base;
use crate::utils::utils::Chunkify;

pub fn url_encode(s: &str) -> String {
    urlencoding::encode(s).into()
}

pub fn url_decode(s: &str) -> String {
    urlencoding::decode(s)
        .expect("Error decoding the url string!")
        .into()
}

// TODO: a pain to do
pub fn playfair(s: &str, k: &str) -> String {
    let mut matrix: Vec<char> = Vec::new();

    for c in k.split_whitespace().collect::<String>().chars() {
        if !matrix.contains(&c) {
            matrix.push(c);
        }
    }

    for i in "abcdefghiklmnopqrstuvwxyz".chars() {
        if !matrix.contains(&i) {
            matrix.push(i);
        }
    }

    let _converted_matrix: Vec<char> = matrix
        .chunks(5)
        .map(|i| {
            println!("{i:?}");
            'a'
        })
        .collect();

    let _alr = s
        .chunkify()
        .chunks(2)
        .map(|c| {
            println!("{} {}", c[0], if c.len() == 1 { 'X' } else { c[1] });
            // println!("{}", c[1]);
            'a'
        })
        .collect::<Vec<char>>();

    "noice".to_string()
}

// TODO: on the way, does not work.
pub fn dna(s: &str) -> String {
    let str_take1: String = s
        .chunkify()
        .chunks(3)
        .map(|i| {
            DNA1.get(&*String::from_iter(i))
                .expect(&format!("Chunk '{:?}' not found in the list", i))
        })
        .collect();
    let str_take2: String = s
        .to_uppercase()
        .chunkify()
        .chunks(1)
        .map(|c| *DNA2.get(&c[0]).unwrap())
        .collect();

    let nice = base::binary(&str_take2);
    println!("Noice: {nice:?}");

    format!("1: {}\n2: {}", str_take1, str_take2)
}

// NOTE: on the way, doesnt work now.
pub fn railfence(s: &str) -> String {
    let nice = s
        .chunkify()
        .chunks(3)
        .map(|i| i.iter().collect::<String>())
        .collect::<Vec<String>>();
    println!("{nice:?}");
    "noice".to_owned()
}

// TODO: Cleanify the hacks
pub fn bacon(s: &str) -> String {
    let clean_str = s.replace("0", "a").replace("1", "b").to_lowercase();

    let str_take1: String = clean_str
        .chunkify()
        .chunks(5)
        .map(|word| {
            let idx = BACON1
                .iter()
                .position(|&i| i == String::from_iter(word))
                .unwrap_or(100);
            ALPHABETS.chars().nth(idx).unwrap_or('?')
        })
        .collect();

    let str_take2: String = clean_str
        .chunkify()
        .chunks(5)
        .map(|word| {
            let idx = BACON2
                .iter()
                .position(|&i| i == String::from_iter(word))
                .unwrap_or(100);
            ALT_PHABETS.chars().nth(idx).unwrap_or('?')
        })
        .collect();

    format!("1: {}\n2: {}", str_take1, str_take2)
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
