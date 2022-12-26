use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref BACON1: &'static [&'static str] = &[
        "aaaaa", "aaaab", "aaaba", "aaabb", "aabaa", "aabab", "aabba", "aabbb", "abaaa", "abaab",
        "ababa", "ababb", "abbaa", "abbab", "abbba", "abbbb", "baaaa", "baaab", "baaba", "baabb",
        "babaa", "babab", "babba", "babbb",
    ];
    pub static ref BACON2: &'static [&'static str] = &[
        "aaaaa", "aaaab", "aaaba", "aaabb", "aabaa", "aabab", "aabba", "aabbb", "abaaa", "abaab",
        "ababa", "ababb", "abbaa", "abbab", "abbba", "abbbb", "baaaa", "baaab", "baaba", "baabb",
        "babaa", "babab", "babba", "babbb", "bbaaa", "bbaab",
    ];
    pub static ref ALPHABETS: &'static str = "abcdefghijklmnopqrstuvwxyz";
    pub static ref ALT_PHABETS: &'static str = "abcdefghiklmnopqrstuwxyz";
    pub static ref DNA2: HashMap<char, &'static str> =
        HashMap::from([('A', "00"), ('C', "10"), ('G', "01"), ('T', "11")]);
    pub static ref DNA1: HashMap<&'static str, char> = HashMap::from([
        ("AAA", 'a'),
        ("CAA", 'q'),
        ("GAA", 'G'),
        ("TAA", 'W'),
        ("AAC", 'b'),
        ("CAC", 'r'),
        ("GAC", 'H'),
        ("TAC", 'X'),
        ("AAG", 'c'),
        ("CAG", 's'),
        ("GAG", 'I'),
        ("TAG", 'Y'),
        ("AAT", 'd'),
        ("CAT", 't'),
        ("GAT", 'J'),
        ("TAT", 'Z'),
        ("ACA", 'e'),
        ("CCA", 'u'),
        ("GCA", 'K'),
        ("TCA", '1'),
        ("ACC", 'f'),
        ("CCC", 'v'),
        ("GCC", 'L'),
        ("TCC", '2'),
        ("ACG", 'g'),
        ("CCG", 'w'),
        ("GCG", 'M'),
        ("TCG", '3'),
        ("ACT", 'h'),
        ("CCT", 'x'),
        ("GCT", 'N'),
        ("TCT", '4'),
        ("AGA", 'i'),
        ("CGA", 'y'),
        ("GGA", 'O'),
        ("TGA", '5'),
        ("AGC", 'j'),
        ("CGC", 'z'),
        ("GGC", 'P'),
        ("TGC", '6'),
        ("AGG", 'k'),
        ("CGG", 'A'),
        ("GGG", 'Q'),
        ("TGG", '7'),
        ("AGT", 'l'),
        ("CGT", 'B'),
        ("GGT", 'R'),
        ("TGT", '8'),
        ("ATA", 'm'),
        ("CTA", 'C'),
        ("GTA", 'S'),
        ("TTA", '9'),
        ("ATC", 'n'),
        ("CTC", 'D'),
        ("GTC", 'T'),
        ("TTC", '0'),
        ("ATG", 'o'),
        ("CTG", 'E'),
        ("GTG", 'U'),
        ("TTG", ' '),
        ("ATT", 'p'),
        ("CTT", 'F'),
        ("GTT", 'V'),
        ("TTT", '.'),
    ]);
}