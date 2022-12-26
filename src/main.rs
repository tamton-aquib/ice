use ice::{base, caesar, extract, general, manipulation, morse, xor};
const VERSION: &str = "0.0.2";

// const HELP: &str = r#"

fn main() {
    let mut args: Vec<String> = std::env::args().collect();

    if ["-h", "-V", "--help", "--version"]
        .iter()
        .any(|f| args.contains(&f.to_string()))
        || args.len() == 1
    {
        println!(
            "
Ice - A simple ctf tool store.
version: {VERSION}

Usage   : ice [subcommand] [query] [OPTIONAL-QUERIES]
Example : ice b64 bmljZQ=="
        );
        return;
    }

    if !atty::is(atty::Stream::Stdin) {
        let mut extrargs: Vec<String> = std::io::stdin().lines().map(|i| i.unwrap()).collect();
        args.append(&mut extrargs);
    }

    if args.len() <= 2 {
        println!("Please provide the subcommand and/or query!");
        println!("Example: ice b64 bmljZQ==");
        return;
    }

    let subcommand = &args[1];
    let query = &args[2].trim();

    let res = match subcommand.trim() {
        // Caesar commands
        "caesar" | "ceasar" | "c" => caesar::caesar(query),
        "rot13" | "rot" => caesar::rot13(query),
        "vigenere" | "vig" => {
            if &args.len() >= &4 {
                caesar::vigenere(query, &args[3])
            } else {
                String::from("Please provide a key!")
            }
        }

        // Morse command
        // TODO: morse decode and encode into seperate subcommands
        "morse" => morse::morse(query),

        // XOR commands
        "xor" => String::from("Please select one from hxh, sxs, sxb instead!"),
        "hxh" => xor::hex_x_hex(query, &args[3]),
        "sxs" => xor::str_x_str(query, &args[3]),
        "sxb" | "bxs" => xor::str_x_byte(query),
        "hxb" | "bxh" => xor::hex_x_byte(query),

        // Base commands
        "b64" => base::b64(query),
        "b32" => base::b32(query),
        "octal" | "oct" => base::octal(query),
        "hex" | "hexa" | "b16" => base::hexadecimal(query),
        "binary" | "bin" => base::binary(query),

        // Manipulation commands
        "lower" => manipulation::lower(query),
        "upper" => manipulation::upper(query),
        "remove_whitespace" | "rw" => manipulation::remove_whitespace(query),
        "reverse" | "rev" => manipulation::reverse(query),
        "length" | "len" => manipulation::length(query),

        // General
        "a1z26" | "az" => general::a1z26(query),
        "atbash" => general::atbash(query),
        "ascii" => general::ascii(query),
        "bacon" => general::bacon(query),

        //Extractor
        "email" | "emails" | "mails" | "mail" => extract::extractor("email", query),
        "phone" | "phones" | "mobile" | "number" => extract::extractor("phone", query),
        "ipv4" | "ip" | "ips" => extract::extractor("ip", query),

        // Analyzer
        // "strings" | "string" | "rb" => analyze::read_binary(query, &args[2]),
        // NOTE: Working on...
        "DNA" | "dna" => general::dna(query),
        "playfair" | "pf" => general::playfair(query, &args[3]),
        "railfence" => general::railfence(query),
        _ => String::from("Subcommand not found!"),
    };

    println!("{}", res.trim());
}
