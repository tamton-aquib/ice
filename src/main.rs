use clap::Parser;
use cli::{Cli, Subcommands};
use ice::{base, caesar, extract, general, manipulation, morse, services, xor};

mod cli;

fn main() {
    let cli = Cli::parse();

    let res = match &cli.command {
        Subcommands::Caesar { text } => caesar::caesar(text),
        Subcommands::Rot13 { text } => caesar::rot13(text),
        Subcommands::Vigenere { text, key } => caesar::vigenere(text, key),
        Subcommands::Morse { text } => morse::morse(text),
        Subcommands::Hxh { hex1, hex2 } => xor::hex_x_hex(hex1, hex2),
        Subcommands::Sxs { str1, str2 } => xor::str_x_str(str1, str2),
        Subcommands::Sxb { text } => xor::str_x_byte(text),
        Subcommands::Hxb { text } => xor::hex_x_byte(text),
        Subcommands::B64 { text } => base::b64(text),
        Subcommands::B32 { text } => base::b32(text),
        Subcommands::Octal { text } => base::octal(text),
        Subcommands::Hex { text } => base::hexadecimal(text),
        Subcommands::Binary { text } => base::binary(text),
        Subcommands::Lower { text } => manipulation::lower(text),
        Subcommands::Upper { text } => manipulation::upper(text),
        Subcommands::RemoveWhitespace { text } => manipulation::remove_whitespace(text),
        Subcommands::Reverse { text } => manipulation::reverse(text),
        Subcommands::Length { text } => manipulation::length(text),
        Subcommands::A1z26 { text } => general::a1z26(text),
        Subcommands::Atbash { text } => general::atbash(text),
        Subcommands::Ascii { text } => general::ascii(text),
        Subcommands::Bacon { text } => general::bacon(text),
        Subcommands::Email { text } => extract::extractor("email", text),
        Subcommands::Phone { text } => extract::extractor("phone", text),
        Subcommands::Ipv4 { text } => extract::extractor("ip", text),
        Subcommands::Dna { text } => general::dna(text),
        Subcommands::Playfair { text, key } => general::playfair(text, key),
        Subcommands::Railfence { text } => general::railfence(text),
        Subcommands::Urle { text } => general::url_encode(text),
        Subcommands::Urld { text } => general::url_decode(text),
        Subcommands::Fdb { number } => services::factordb(number),
    };

    println!("{}", res.trim());
}