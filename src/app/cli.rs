use crate::{
    analysis::{extract, manipulation},
    base::base,
    ciphers::{caesar, general::general, morse, xor},
    hasher::hasher,
    utils::services,
};
use pico_args::Arguments;

#[derive(Debug)]
pub enum Command {
    Caesar(String),
    Rot13(String),
    Vigenere(String, String),
    Morse(String),
    HexXHex(String, String),
    StrXStr(String, String),
    StrXByte(String),
    HexXByte(String),
    Base64(String),
    Base32(String),
    Octal(String),
    Hexadecimal(String),
    Binary(String),
    Lower(String),
    Upper(String),
    RemoveWhitespace(String),
    Reverse(String),
    Length(String),
    A1Z26(String),
    Atbash(String),
    Ascii(String),
    Bacon(String),
    Email(String),
    Phone(String),
    Ipv4(String),
    Dna(String),
    Playfair(String, String),
    Railfence(String),
    UrlEncode(String),
    UrlDecode(String),
    FactorDb(String),
    Md5(String),
    Sha1(String),
    Sha256(String),
    Sha512(String),
    Help,
    Version,
    Unknown(String),
}

impl Command {
    pub fn run(&self) {
        match self {
            Command::Caesar(text) => println!("{}", caesar::caesar(text).trim()),
            Command::Rot13(text) => println!("{}", caesar::rot13(text).trim()),
            Command::Vigenere(text, key) => println!("{}", caesar::vigenere(text, key).trim()),
            Command::Morse(text) => println!("{}", morse::morse(text).trim()),
            Command::HexXHex(hex1, hex2) => println!("{}", xor::hex_x_hex(hex1, hex2).trim()),
            Command::StrXStr(str1, str2) => println!("{}", xor::str_x_str(str1, str2).trim()),
            Command::StrXByte(text) => println!("{}", xor::str_x_byte(text).trim()),
            Command::HexXByte(text) => println!("{}", xor::hex_x_byte(text).trim()),
            Command::Base64(text) => println!("{}", base::b64(text).trim()),
            Command::Base32(text) => println!("{}", base::b32(text).trim()),
            Command::Octal(text) => println!("{}", base::octal(text).trim()),
            Command::Hexadecimal(text) => println!("{}", base::hexadecimal(text).trim()),
            Command::Binary(text) => println!("{}", base::binary(text).trim()),
            Command::Lower(text) => println!("{}", manipulation::lower(text).trim()),
            Command::Upper(text) => println!("{}", manipulation::upper(text).trim()),
            Command::RemoveWhitespace(text) => {
                println!("{}", manipulation::remove_whitespace(text).trim())
            }
            Command::Reverse(text) => println!("{}", manipulation::reverse(text).trim()),
            Command::Length(text) => println!("{}", manipulation::length(text).trim()),
            Command::A1Z26(text) => println!("{}", general::a1z26(text).trim()),
            Command::Atbash(text) => println!("{}", general::atbash(text).trim()),
            Command::Ascii(text) => println!("{}", general::ascii(text).trim()),
            Command::Bacon(text) => println!("{}", general::bacon(text).trim()),
            Command::Email(text) => println!("{}", extract::extractor("email", text).trim()),
            Command::Phone(text) => println!("{}", extract::extractor("phone", text).trim()),
            Command::Ipv4(text) => println!("{}", extract::extractor("ip", text).trim()),
            Command::Dna(text) => println!("{}", general::dna(text).trim()),
            Command::Playfair(text, key) => println!("{}", general::playfair(text, key).trim()),
            Command::Railfence(text) => println!("{}", general::railfence(text).trim()),
            Command::UrlEncode(text) => println!("{}", general::url_encode(text).trim()),
            Command::UrlDecode(text) => println!("{}", general::url_decode(text).trim()),
            Command::FactorDb(number) => println!("{}", services::factordb(number).trim()),
            Command::Md5(input) => println!("{}", hasher::md5(input).trim()),
            Command::Sha1(input) => println!("{}", hasher::sha1(input).trim()),
            Command::Sha256(input) => println!("{}", hasher::sha256(input).trim()),
            Command::Sha512(input) => println!("{}", hasher::sha512(input).trim()),
            Command::Help => {}
            Command::Version => {}
            Command::Unknown(cmd) => {
                eprintln!("Error: Unknown subcommand '{}'", cmd);
            }
        }
    }
}

pub fn parse_args(args: &mut Arguments) -> Result<Command, String> {
    if args.contains(["-h", "--help"]) {
        return Ok(Command::Help);
    }
    if args.contains(["-v", "--version"]) {
        return Ok(Command::Version);
    }

    let subcommand = match args.subcommand() {
        Ok(Some(cmd)) => cmd,
        _ => return Ok(Command::Help),
    };

    let cmd = match subcommand.as_str() {
        "caesar" | "c" => Command::Caesar(
            args.free_from_str()
                .map_err(|e| format!("Missing argument for caesar: {}", e))?,
        ),
        "rot13" | "rot" => Command::Rot13(
            args.free_from_str()
                .map_err(|e| format!("Missing argument for rot13: {}", e))?,
        ),
        "vigenere" | "vig" => Command::Vigenere(
            args.free_from_str()
                .map_err(|e| format!("Missing text argument for vigenere: {}", e))?,
            args.free_from_str()
                .map_err(|e| format!("Missing key argument for vigenere: {}", e))?,
        ),
        "morse" => Command::Morse(
            args.free_from_str()
                .map_err(|e| format!("Missing argument for morse: {}", e))?,
        ),
        "hxh" => Command::HexXHex(
            args.free_from_str()
                .map_err(|e| format!("Missing hex1 argument for hxh: {}", e))?,
            args.free_from_str()
                .map_err(|e| format!("Missing hex2 argument for hxh: {}", e))?,
        ),
        "sxs" => Command::StrXStr(
            args.free_from_str()
                .map_err(|e| format!("Missing str1 argument for sxs: {}", e))?,
            args.free_from_str()
                .map_err(|e| format!("Missing str2 argument for sxs: {}", e))?,
        ),
        "sxb" | "bxs" => Command::StrXByte(
            args.free_from_str()
                .map_err(|e| format!("Missing argument for sxb: {}", e))?,
        ),
        "hxb" | "bxh" => Command::HexXByte(
            args.free_from_str()
                .map_err(|e| format!("Missing argument for hxb: {}", e))?,
        ),
        "b64" => Command::Base64(
            args.free_from_str()
                .map_err(|e| format!("Missing argument for b64: {}", e))?,
        ),
        "b32" => Command::Base32(
            args.free_from_str()
                .map_err(|e| format!("Missing argument for b32: {}", e))?,
        ),
        "octal" | "oct" => Command::Octal(
            args.free_from_str()
                .map_err(|e| format!("Missing argument for octal: {}", e))?,
        ),
        "hex" | "hexa" | "b16" => Command::Hexadecimal(
            args.free_from_str()
                .map_err(|e| format!("Missing argument for hex: {}", e))?,
        ),
        "binary" | "bin" => Command::Binary(
            args.free_from_str()
                .map_err(|e| format!("Missing argument for binary: {}", e))?,
        ),
        "lower" => Command::Lower(
            args.free_from_str()
                .map_err(|e| format!("Missing argument for lower: {}", e))?,
        ),
        "upper" => Command::Upper(
            args.free_from_str()
                .map_err(|e| format!("Missing argument for upper: {}", e))?,
        ),
        "remove-whitespace" | "rw" => Command::RemoveWhitespace(
            args.free_from_str()
                .map_err(|e| format!("Missing argument for remove-whitespace: {}", e))?,
        ),
        "reverse" | "rev" => Command::Reverse(
            args.free_from_str()
                .map_err(|e| format!("Missing argument for reverse: {}", e))?,
        ),
        "length" | "len" => Command::Length(
            args.free_from_str()
                .map_err(|e| format!("Missing argument for length: {}", e))?,
        ),
        "a1z26" | "az" => Command::A1Z26(
            args.free_from_str()
                .map_err(|e| format!("Missing argument for a1z26: {}", e))?,
        ),
        "atbash" => Command::Atbash(
            args.free_from_str()
                .map_err(|e| format!("Missing argument for atbash: {}", e))?,
        ),
        "ascii" => Command::Ascii(
            args.free_from_str()
                .map_err(|e| format!("Missing argument for ascii: {}", e))?,
        ),
        "bacon" => Command::Bacon(
            args.free_from_str()
                .map_err(|e| format!("Missing argument for bacon: {}", e))?,
        ),
        "email" | "emails" | "mails" | "mail" => Command::Email(
            args.free_from_str()
                .map_err(|e| format!("Missing argument for email: {}", e))?,
        ),
        "phone" | "phones" | "mobile" | "number" => Command::Phone(
            args.free_from_str()
                .map_err(|e| format!("Missing argument for phone: {}", e))?,
        ),
        "ipv4" | "ip" | "ips" => Command::Ipv4(
            args.free_from_str()
                .map_err(|e| format!("Missing argument for ipv4: {}", e))?,
        ),
        "dna" => Command::Dna(
            args.free_from_str()
                .map_err(|e| format!("Missing argument for dna: {}", e))?,
        ),
        "playfair" | "pf" => Command::Playfair(
            args.free_from_str()
                .map_err(|e| format!("Missing text argument for playfair: {}", e))?,
            args.free_from_str()
                .map_err(|e| format!("Missing key argument for playfair: {}", e))?,
        ),
        "railfence" => Command::Railfence(
            args.free_from_str()
                .map_err(|e| format!("Missing argument for railfence: {}", e))?,
        ),
        "urle" | "ue" | "urlenc" | "urlencode" => Command::UrlEncode(
            args.free_from_str()
                .map_err(|e| format!("Missing argument for urle: {}", e))?,
        ),
        "urld" | "ud" | "urldec" | "urldecode" => Command::UrlDecode(
            args.free_from_str()
                .map_err(|e| format!("Missing argument for urld: {}", e))?,
        ),
        "fdb" | "factor" | "factordb" => Command::FactorDb(
            args.free_from_str()
                .map_err(|e| format!("Missing argument for fdb: {}", e))?,
        ),
        "md5" => Command::Md5(
            args.free_from_str()
                .map_err(|e| format!("Missing argument for md5: {}", e))?,
        ),
        "sha1" => Command::Sha1(
            args.free_from_str()
                .map_err(|e| format!("Missing argument for sha1: {}", e))?,
        ),
        "sha256" => Command::Sha256(
            args.free_from_str()
                .map_err(|e| format!("Missing argument for sha256: {}", e))?,
        ),
        "sha512" => Command::Sha512(
            args.free_from_str()
                .map_err(|e| format!("Missing argument for sha512: {}", e))?,
        ),
        "help" => Command::Help,
        _ => Command::Unknown(subcommand.to_string()),
    };

    Ok(cmd)
}
