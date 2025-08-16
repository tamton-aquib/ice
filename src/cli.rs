use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Subcommands,
}

#[derive(Subcommand)]
pub enum Subcommands {
    /// Caesar cipher
    #[clap(aliases = &["ceasar", "c"])]
    Caesar { text: String },
    /// Rot13 cipher
    #[clap(alias = "rot")]
    Rot13 { text: String },
    /// Vigenere cipher
    #[clap(alias = "vig")]
    Vigenere { text: String, key: String },
    /// Morse code
    Morse { text: String },
    /// XOR hex with hex
    Hxh { hex1: String, hex2: String },
    /// XOR string with string
    Sxs { str1: String, str2: String },
    /// XOR string with byte
    #[clap(alias = "bxs")]
    Sxb { text: String },
    /// XOR hex with byte
    #[clap(alias = "bxh")]
    Hxb { text: String },
    /// Base64 encoding/decoding
    B64 { text: String },
    /// Base32 encoding/decoding
    B32 { text: String },
    /// Octal encoding/decoding
    #[clap(alias = "oct")]
    Octal { text: String },
    /// Hexadecimal encoding/decoding
    #[clap(aliases = &["hexa", "b16"])]
    Hex { text: String },
    /// Binary encoding/decoding
    #[clap(alias = "bin")]
    Binary { text: String },
    /// Convert to lowercase
    Lower { text: String },
    /// Convert to uppercase
    Upper { text: String },
    /// Remove whitespace
    #[clap(alias = "rw")]
    RemoveWhitespace { text: String },
    /// Reverse text
    #[clap(alias = "rev")]
    Reverse { text: String },
    /// Get text length
    #[clap(alias = "len")]
    Length { text: String },
    /// A1Z26 cipher
    #[clap(alias = "az")]
    A1z26 { text: String },
    /// Atbash cipher
    Atbash { text: String },
    /// ASCII encoding/decoding
    Ascii { text: String },
    /// Baconian cipher
    Bacon { text: String },
    /// Extract emails
    #[clap(aliases = &["emails", "mails", "mail"])]
    Email { text: String },
    /// Extract phone numbers
    #[clap(aliases = &["phones", "mobile", "number"])]
    Phone { text: String },
    /// Extract IPv4 addresses
    #[clap(aliases = &["ip", "ips"])]
    Ipv4 { text: String },
    /// DNA encoding/decoding
    #[clap(alias = "dna")]
    Dna { text: String },
    /// Playfair cipher
    #[clap(alias = "pf")]
    Playfair { text: String, key: String },
    /// Railfence cipher
    Railfence { text: String },
    /// URL encode
    #[clap(aliases = &["ue", "urlenc", "urlencode"])]
    Urle { text: String },
    /// URL decode
    #[clap(aliases = &["ud", "urldec", "urldecode"])]
    Urld { text: String },
    /// FactorDB lookup
    #[clap(aliases = &["factor", "factordb"])]
    Fdb { number: String },
}
