use crate::{
    analysis::{extract, manipulation},
    base::base,
    ciphers::{caesar, general::general, morse, xor},
    hasher::hasher,
    utils::services,
};
use clap::{Command, CommandFactory, Parser, Subcommand};
use clap_complete::Shell;

#[derive(Parser, Debug)]
#[command(
    name = "ice",
    about = "A simple CTF tool store",
    version,
    author = "tamton-aquib <aquibjavedt007@gmail.com>"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(aliases = ["c"])]
    Caesar {
        text: String,
    },
    #[command(aliases = ["rot"])]
    Rot13 {
        text: String,
    },
    #[command(aliases = ["vig"])]
    Vigenere {
        text: String,
        key: String,
    },
    Morse {
        text: String,
    },
    #[command(name = "hxh")]
    HexXHex {
        hex1: String,
        hex2: String,
    },
    #[command(name = "sxs")]
    StrXStr {
        str1: String,
        str2: String,
    },
    #[command(name = "sxb", aliases = ["bxs"])]
    StrXByte {
        text: String,
    },
    #[command(name = "hxb", aliases = ["bxh"])]
    HexXByte {
        text: String,
    },
    #[command(name = "b64")]
    Base64 {
        text: String,
    },
    #[command(name = "b32")]
    Base32 {
        text: String,
    },
    #[command(aliases = ["oct"])]
    Octal {
        text: String,
    },
    #[command(name = "hex", aliases = ["hexa", "b16"])]
    Hexadecimal {
        text: String,
    },
    #[command(aliases = ["bin"])]
    Binary {
        text: String,
    },
    Lower {
        text: String,
    },
    Upper {
        text: String,
    },
    #[command(name = "remove-whitespace", aliases = ["rw"])]
    RemoveWhitespace {
        text: String,
    },
    #[command(aliases = ["rev"])]
    Reverse {
        text: String,
    },
    #[command(aliases = ["len"])]
    Length {
        text: String,
    },
    #[command(aliases = ["az"])]
    A1Z26 {
        text: String,
    },
    Atbash {
        text: String,
    },
    Ascii {
        text: String,
    },
    Bacon {
        text: String,
    },
    #[command(aliases = ["emails", "mails", "mail"])]
    Email {
        text: String,
    },
    #[command(aliases = ["phones", "mobile", "number"])]
    Phone {
        text: String,
    },
    #[command(name = "ipv4", aliases = ["ip", "ips"])]
    Ipv4 {
        text: String,
    },
    Dna {
        text: String,
    },
    #[command(aliases = ["pf"])]
    Playfair {
        text: String,
        key: String,
    },
    Railfence {
        text: String,
    },
    #[command(name = "urle", aliases = ["ue", "urlenc", "urlencode"])]
    UrlEncode {
        text: String,
    },
    #[command(name = "urld", aliases = ["ud", "urldec", "urldecode"])]
    UrlDecode {
        text: String,
    },
    #[command(name = "fdb", aliases = ["factor", "factordb"])]
    FactorDb {
        number: String,
    },
    Md5 {
        input: String,
    },
    Sha1 {
        input: String,
    },
    Sha256 {
        input: String,
    },
    Sha512 {
        input: String,
    },
    #[command(name = "completion")]
    Completion {
        #[arg(value_enum, default_value_t = Shell::Bash)]
        shell: Shell,
    },
}

pub fn build_cli() -> Command {
    Cli::command()
}

impl Commands {
    pub fn run(&self) {
        match self {
            Commands::Caesar { text } => println!("{}", caesar::caesar(text).trim()),
            Commands::Rot13 { text } => println!("{}", caesar::rot13(text).trim()),
            Commands::Vigenere { text, key } => println!("{}", caesar::vigenere(text, key).trim()),
            Commands::Morse { text } => println!("{}", morse::morse(text).trim()),
            Commands::HexXHex { hex1, hex2 } => println!("{}", xor::hex_x_hex(hex1, hex2).trim()),
            Commands::StrXStr { str1, str2 } => println!("{}", xor::str_x_str(str1, str2).trim()),
            Commands::StrXByte { text } => println!("{}", xor::str_x_byte(text).trim()),
            Commands::HexXByte { text } => println!("{}", xor::hex_x_byte(text).trim()),
            Commands::Base64 { text } => println!("{}", base::b64(text).trim()),
            Commands::Base32 { text } => println!("{}", base::b32(text).trim()),
            Commands::Octal { text } => println!("{}", base::octal(text).trim()),
            Commands::Hexadecimal { text } => println!("{}", base::hexadecimal(text).trim()),
            Commands::Binary { text } => println!("{}", base::binary(text).trim()),
            Commands::Lower { text } => println!("{}", manipulation::lower(text).trim()),
            Commands::Upper { text } => println!("{}", manipulation::upper(text).trim()),
            Commands::RemoveWhitespace { text } => {
                println!("{}", manipulation::remove_whitespace(text).trim())
            }
            Commands::Reverse { text } => println!("{}", manipulation::reverse(text).trim()),
            Commands::Length { text } => println!("{}", manipulation::length(text).trim()),
            Commands::A1Z26 { text } => println!("{}", general::a1z26(text).trim()),
            Commands::Atbash { text } => println!("{}", general::atbash(text).trim()),
            Commands::Ascii { text } => println!("{}", general::ascii(text).trim()),
            Commands::Bacon { text } => println!("{}", general::bacon(text).trim()),
            Commands::Email { text } => println!("{}", extract::extractor("email", text).trim()),
            Commands::Phone { text } => println!("{}", extract::extractor("phone", text).trim()),
            Commands::Ipv4 { text } => println!("{}", extract::extractor("ip", text).trim()),
            Commands::Dna { text } => println!("{}", general::dna(text).trim()),
            Commands::Playfair { text, key } => println!("{}", general::playfair(text, key).trim()),
            Commands::Railfence { text } => println!("{}", general::railfence(text).trim()),
            Commands::UrlEncode { text } => println!("{}", general::url_encode(text).trim()),
            Commands::UrlDecode { text } => println!("{}", general::url_decode(text).trim()),
            Commands::FactorDb { number } => println!("{}", services::factordb(number).trim()),
            Commands::Md5 { input } => println!("{}", hasher::md5(input).trim()),
            Commands::Sha1 { input } => println!("{}", hasher::sha1(input).trim()),
            Commands::Sha256 { input } => println!("{}", hasher::sha256(input).trim()),
            Commands::Sha512 { input } => println!("{}", hasher::sha512(input).trim()),
            Commands::Completion { shell } => {
                let mut cmd = build_cli();
                let name = cmd.get_name().to_string();
                clap_complete::generate(*shell, &mut cmd, name, &mut std::io::stdout());
            }
        }
    }
}
