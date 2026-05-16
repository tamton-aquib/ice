use anyhow::Result;
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
    #[command(about = "Caesar cipher — bruteforce all shifts, or encode with --shift", aliases = ["c"])]
    Caesar {
        text: String,
        #[arg(short, long)]
        shift: Option<u8>,
    },
    #[command(about = "ROT13 cipher", aliases = ["rot"])]
    Rot13 {
        text: String,
    },
    #[command(about = "Vigenere cipher — encrypt by default, use --decrypt to decrypt", aliases = ["vig"])]
    Vigenere {
        text: String,
        key: String,
        #[arg(short, long)]
        decrypt: bool,
    },
    #[command(about = "Morse code encode/decode (auto-detects)")]
    Morse {
        text: String,
    },
    #[command(name = "hxh", about = "XOR two hex strings")]
    HexXHex {
        hex1: String,
        hex2: String,
    },
    #[command(name = "sxs", about = "XOR two strings (returns hex)")]
    StrXStr {
        str1: String,
        str2: String,
    },
    #[command(name = "sxb", about = "XOR a string against all single-byte keys", aliases = ["bxs"])]
    StrXByte {
        text: String,
    },
    #[command(name = "hxb", about = "XOR a hex string against all single-byte keys", aliases = ["bxh"])]
    HexXByte {
        text: String,
    },
    #[command(name = "b64", about = "Base64 encode/decode (auto-detects)")]
    Base64 {
        text: String,
    },
    #[command(name = "b32", about = "Base32 encode/decode (auto-detects)")]
    Base32 {
        text: String,
    },
    #[command(name = "b45", about = "Base45 encode/decode (auto-detects)")]
    Base45 {
        text: String,
    },
    #[command(name = "b58", about = "Base58 encode/decode (auto-detects)")]
    Base58 {
        text: String,
    },
    #[command(name = "b62", about = "Base62 encode/decode (auto-detects)")]
    Base62 {
        text: String,
    },
    #[command(name = "b85", about = "Base85 (Ascii85) encode/decode (auto-detects)")]
    Base85 {
        text: String,
    },
    #[command(about = "Octal encode/decode (auto-detects)", aliases = ["oct"])]
    Octal {
        text: String,
    },
    #[command(name = "hex", about = "Hex encode/decode (auto-detects)", aliases = ["hexa", "b16"])]
    Hexadecimal {
        text: String,
    },
    #[command(about = "Binary encode/decode (auto-detects)", aliases = ["bin"])]
    Binary {
        text: String,
    },
    #[command(about = "Convert text to lowercase")]
    Lower {
        text: String,
    },
    #[command(about = "Convert text to uppercase")]
    Upper {
        text: String,
    },
    #[command(name = "remove-whitespace", about = "Remove all whitespace from text", aliases = ["rw"])]
    RemoveWhitespace {
        text: String,
    },
    #[command(about = "Reverse the string", aliases = ["rev"])]
    Reverse {
        text: String,
    },
    #[command(about = "Get the length of the string", aliases = ["len"])]
    Length {
        text: String,
    },
    #[command(about = "A1Z26 cipher — decode numbers to letters", aliases = ["az"])]
    A1Z26 {
        text: String,
    },
    #[command(about = "Atbash cipher")]
    Atbash {
        text: String,
    },
    #[command(about = "Convert text to/from ASCII codes")]
    Ascii {
        text: String,
    },
    #[command(about = "Baconian cipher encode/decode")]
    Bacon {
        text: String,
    },
    #[command(about = "ROT47 cipher (rotates ASCII 33-126)")]
    Rot47 {
        text: String,
    },
    #[command(about = "Extract email addresses from text or a file", aliases = ["emails", "mails", "mail"])]
    Email {
        text: String,
    },
    #[command(about = "Extract phone numbers from text or a file", aliases = ["phones", "mobile", "number"])]
    Phone {
        text: String,
    },
    #[command(name = "ipv4", about = "Extract IPv4 addresses from text or a file", aliases = ["ip", "ips"])]
    Ipv4 {
        text: String,
    },
    #[command(about = "DNA encode/decode (2-bit and codon auto-detect)")]
    Dna {
        text: String,
    },
    #[command(about = "Playfair cipher — encrypt by default, use --decrypt to decrypt", aliases = ["pf"])]
    Playfair {
        text: String,
        key: String,
        #[arg(short, long)]
        decrypt: bool,
    },
    #[command(about = "Rail fence cipher — use --rails N, or omit to bruteforce")]
    Railfence {
        text: String,
        #[arg(short, long)]
        rails: Option<usize>,
    },
    #[command(name = "urle", about = "URL-encode a string", aliases = ["ue", "urlenc", "urlencode"])]
    UrlEncode {
        text: String,
    },
    #[command(name = "urld", about = "URL-decode a string", aliases = ["ud", "urldec", "urldecode"])]
    UrlDecode {
        text: String,
    },
    #[command(name = "fdb", about = "Look up number factors on factordb.com", aliases = ["factor", "factordb"])]
    FactorDb {
        number: String,
    },
    #[command(about = "MD5 hash a string or file")]
    Md5 {
        input: String,
    },
    #[command(about = "SHA1 hash a string or file")]
    Sha1 {
        input: String,
    },
    #[command(about = "SHA256 hash a string or file")]
    Sha256 {
        input: String,
    },
    #[command(about = "SHA512 hash a string or file")]
    Sha512 {
        input: String,
    },
    #[command(name = "jwt", about = "Decode JWT header and payload (no signature verification)")]
    Jwt {
        token: String,
    },
    #[command(name = "completion", about = "Generate shell completion script")]
    Completion {
        #[arg(value_enum, default_value_t = Shell::Bash)]
        shell: Shell,
    },
    #[command(about = "Affine cipher (ax+b mod 26) — --decrypt to decrypt")]
    Affine {
        text: String,
        a: i32,
        b: i32,
        #[arg(short, long)]
        decrypt: bool,
    },
    #[command(about = "Bifid cipher with a Polybius square key — --decrypt to decrypt")]
    Bifid {
        text: String,
        key: String,
        #[arg(short, long)]
        decrypt: bool,
    },
    #[command(about = "Simple substitution cipher with a 26-char key — --decrypt to decrypt")]
    Substitution {
        text: String,
        key: String,
        #[arg(short, long)]
        decrypt: bool,
    },
}

pub fn build_cli() -> Command {
    let cmd = Cli::command();
    cmd.subcommand_help_heading("Subcommands:")
        .help_template("\
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading} {usage}

{subcommands}{after-help}")
}

pub fn print_help() {
    let name = "ice";
    let version = env!("CARGO_PKG_VERSION");
    let about = "A simple CTF tool store";

    let mut out = String::new();
    out.push_str(&format!("{name} {version}\n{about}\n\n"));
    out.push_str("Usage: ice [COMMAND]\n\n");

    let sections = [
        ("Encoding / Decoding", &[
            ("b64",     "Base64 encode/decode (auto-detects)"),
            ("b32",     "Base32 encode/decode (auto-detects)"),
            ("b45",     "Base45 encode/decode (auto-detects)"),
            ("b58",     "Base58 encode/decode (auto-detects)"),
            ("b62",     "Base62 encode/decode (auto-detects)"),
            ("b85",     "Base85 (Ascii85) encode/decode (auto-detects)"),
            ("hex",     "Hex encode/decode (auto-detects)"),
            ("octal",   "Octal encode/decode (auto-detects)"),
            ("binary",  "Binary encode/decode (auto-detects)"),
        ] as &[(&str, &str)]),
        ("Ciphers", &[
            ("caesar",        "Caesar cipher (--shift N to encode)"),
            ("rot13",         "ROT13 cipher"),
            ("vigenere",      "Vigenere cipher (--decrypt to decrypt)"),
            ("morse",         "Morse code encode/decode (auto-detects)"),
            ("atbash",        "Atbash cipher"),
            ("a1z26",         "A1Z26 cipher -- numbers to letters"),
            ("ascii",         "Convert text to/from ASCII codes"),
            ("bacon",         "Baconian cipher encode/decode"),
            ("rot47",         "ROT47 cipher (ASCII 33-126)"),
            ("playfair",      "Playfair cipher (--decrypt to decrypt)"),
            ("railfence",     "Rail fence cipher (--rails N)"),
            ("dna",           "DNA encode/decode (auto-detects)"),
            ("affine",        "Affine cipher ax+b (--decrypt to decrypt)"),
            ("bifid",         "Bifid cipher with a key (--decrypt)"),
            ("substitution",  "Substitution cipher (--decrypt to decrypt)"),
        ]),
        ("XOR", &[
            ("hxh", "XOR two hex strings"),
            ("sxs", "XOR two strings (returns hex)"),
            ("sxb", "XOR string against all single-byte keys"),
            ("hxb", "XOR hex string against all single-byte keys"),
        ]),
        ("Hashing", &[
            ("md5",    "MD5 hash a string or file"),
            ("sha1",   "SHA1 hash a string or file"),
            ("sha256", "SHA256 hash a string or file"),
            ("sha512", "SHA512 hash a string or file"),
        ]),
        ("URL", &[
            ("urle", "URL-encode a string"),
            ("urld", "URL-decode a string"),
        ]),
        ("Extract", &[
            ("email", "Extract email addresses from text or a file"),
            ("phone", "Extract phone numbers from text or a file"),
            ("ipv4",  "Extract IPv4 addresses from text or a file"),
        ]),
        ("Formatting", &[
            ("lower",              "Convert text to lowercase"),
            ("upper",              "Convert text to uppercase"),
            ("remove-whitespace",  "Remove all whitespace from text"),
            ("reverse",            "Reverse the string"),
            ("length",             "Get the length of the string"),
        ]),
        ("Services", &[
            ("fdb", "Look up number factors on factordb.com"),
        ]),
        ("Other", &[
            ("jwt",        "Decode JWT header and payload"),
            ("completion", "Generate shell completion script"),
        ]),
    ];

    for (heading, cmds) in &sections {
        out.push_str(&format!(" {}:\n", heading));
        for (name, desc) in *cmds {
            out.push_str(&format!("   {:<20} {}\n", name, desc));
        }
        out.push('\n');
    }

    out.push_str("Options:\n  -h, --help     Print help\n  -V, --version  Print version\n");
    print!("{out}");
}

impl Commands {
    pub fn run(&self) -> Result<()> {
        match self {
            Commands::Caesar { text, shift } => {
                let result = caesar::caesar(text, *shift)?;
                println!("{}", result.trim());
            }
            Commands::Rot13 { text } => {
                println!("{}", caesar::rot13(text).trim());
            }
            Commands::Vigenere {
                text,
                key,
                decrypt,
            } => {
                let result = caesar::vigenere(text, key, *decrypt)?;
                println!("{}", result.trim());
            }
            Commands::Morse { text } => {
                println!("{}", morse::morse(text).trim());
            }
            Commands::HexXHex { hex1, hex2 } => {
                let result = xor::hex_x_hex(hex1, hex2)?;
                println!("{}", result.trim());
            }
            Commands::StrXStr { str1, str2 } => {
                let result = xor::str_x_str(str1, str2)?;
                println!("{}", result.trim());
            }
            Commands::StrXByte { text } => {
                let result = xor::str_x_byte(text)?;
                println!("{}", result.trim());
            }
            Commands::HexXByte { text } => {
                let result = xor::hex_x_byte(text)?;
                println!("{}", result.trim());
            }
            Commands::Base64 { text } => {
                let result = base::b64(text)?;
                println!("{}", result.trim());
            }
            Commands::Base32 { text } => {
                let result = base::b32(text)?;
                println!("{}", result.trim());
            }
            Commands::Base45 { text } => {
                let result = base::b45(text)?;
                println!("{}", result.trim());
            }
            Commands::Base58 { text } => {
                let result = base::b58(text)?;
                println!("{}", result.trim());
            }
            Commands::Base62 { text } => {
                let result = base::b62(text)?;
                println!("{}", result.trim());
            }
            Commands::Base85 { text } => {
                let result = base::b85(text)?;
                println!("{}", result.trim());
            }
            Commands::Octal { text } => {
                let result = base::octal(text)?;
                println!("{}", result.trim());
            }
            Commands::Hexadecimal { text } => {
                let result = base::hexadecimal(text)?;
                println!("{}", result.trim());
            }
            Commands::Binary { text } => {
                let result = base::binary(text)?;
                println!("{}", result.trim());
            }
            Commands::Lower { text } => {
                println!("{}", manipulation::lower(text).trim());
            }
            Commands::Upper { text } => {
                println!("{}", manipulation::upper(text).trim());
            }
            Commands::RemoveWhitespace { text } => {
                println!("{}", manipulation::remove_whitespace(text).trim());
            }
            Commands::Reverse { text } => {
                println!("{}", manipulation::reverse(text).trim());
            }
            Commands::Length { text } => {
                println!("{}", manipulation::length(text).trim());
            }
            Commands::A1Z26 { text } => {
                let result = general::a1z26(text)?;
                println!("{}", result.trim());
            }
            Commands::Atbash { text } => {
                let result = general::atbash(text)?;
                println!("{}", result.trim());
            }
            Commands::Ascii { text } => {
                let result = general::ascii(text)?;
                println!("{}", result.trim());
            }
            Commands::Bacon { text } => {
                let result = general::bacon(text)?;
                println!("{}", result.trim());
            }
            Commands::Rot47 { text } => {
                println!("{}", caesar::rot47(text).trim());
            }
            Commands::Email { text } => {
                let result = extract::extractor("email", text)?;
                println!("{}", result.trim());
            }
            Commands::Phone { text } => {
                let result = extract::extractor("phone", text)?;
                println!("{}", result.trim());
            }
            Commands::Ipv4 { text } => {
                let result = extract::extractor("ip", text)?;
                println!("{}", result.trim());
            }
            Commands::Dna { text } => {
                let result = general::dna(text)?;
                println!("{}", result.trim());
            }
            Commands::Playfair {
                text,
                key,
                decrypt,
            } => {
                let result = general::playfair(text, key, *decrypt)?;
                println!("{}", result.trim());
            }
            Commands::Railfence { text, rails } => {
                let result = general::railfence(text, *rails)?;
                println!("{}", result.trim());
            }
            Commands::UrlEncode { text } => {
                let result = general::url_encode(text)?;
                println!("{}", result.trim());
            }
            Commands::UrlDecode { text } => {
                let result = general::url_decode(text)?;
                println!("{}", result.trim());
            }
            Commands::FactorDb { number } => {
                let result = services::factordb(number)?;
                println!("{}", result.trim());
            }
            Commands::Md5 { input } => {
                let result = hasher::md5(input)?;
                println!("{}", result.trim());
            }
            Commands::Sha1 { input } => {
                let result = hasher::sha1(input)?;
                println!("{}", result.trim());
            }
            Commands::Sha256 { input } => {
                let result = hasher::sha256(input)?;
                println!("{}", result.trim());
            }
            Commands::Sha512 { input } => {
                let result = hasher::sha512(input)?;
                println!("{}", result.trim());
            }
            Commands::Jwt { token } => {
                let parts: Vec<&str> = token.split('.').collect();
                if parts.len() != 3 {
                    anyhow::bail!("Invalid JWT: expected 3 dot-separated segments");
                }
                let decode_b64url = |input: &str| -> Result<String> {
                    let padded = match input.len() % 4 {
                        2 => format!("{}==", input),
                        3 => format!("{}=", input),
                        _ => input.to_string(),
                    };
                    let standard = padded.replace('-', "+").replace('_', "/");
                    let bytes = base64::decode(&standard)
                        .map_err(|e| anyhow::anyhow!("Base64 decode failed: {}", e))?;
                    String::from_utf8(bytes).map_err(|e| anyhow::anyhow!("Invalid UTF-8: {}", e))
                };
                let header = decode_b64url(parts[0])?;
                let payload = decode_b64url(parts[1])?;
                let header_json: serde_json::Value = serde_json::from_str(&header)?;
                let payload_json: serde_json::Value = serde_json::from_str(&payload)?;
                let output = serde_json::json!({ "header": header_json, "payload": payload_json });
                println!("{}", serde_json::to_string_pretty(&output)?);
            }
            Commands::Completion { shell } => {
                let mut cmd = build_cli();
                let name = cmd.get_name().to_string();
                clap_complete::generate(*shell, &mut cmd, name, &mut std::io::stdout());
            }
            Commands::Affine {
                text,
                a,
                b,
                decrypt,
            } => {
                let result = general::affine(text, *a, *b, *decrypt)?;
                println!("{}", result.trim());
            }
            Commands::Bifid {
                text,
                key,
                decrypt,
            } => {
                let result = general::bifid(text, key, *decrypt)?;
                println!("{}", result.trim());
            }
            Commands::Substitution {
                text,
                key,
                decrypt,
            } => {
                let result = general::substitution(text, key, *decrypt)?;
                println!("{}", result.trim());
            }
        }
        Ok(())
    }
}
