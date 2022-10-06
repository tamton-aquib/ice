use ice::{analyze, base, caesar, general, manipulation, morse, xor};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        println!("Ice - A simple ctf tool store.");
        return;
    }
    if args.len() <= 2 {
        println!("Please provide an argument!");
        return;
    }

    let subcommand = &args[1];
    let query = &args[2].trim();
    // let subcommand = "sxb";
    // let query = "";

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
        "hxh" => xor::hex_x_hex(&args[2], &args[3]),
        "sxs" => xor::str_x_str(&args[2], &args[3]),
        "sxb" | "bxs" => xor::str_x_byte(&args[2]),

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

        // General
        "a1z26" | "az" => general::a1z26(query),
        "atbash" => general::atbash(query),
        "ascii" => general::ascii(query),

        // Analyzer
        // "strings" | "string" | "rb" => analyze::read_binary(query, &args[2]),
        _ => String::from("Subcommand not found!"),
    };

    println!("Result\n======\n{}", res.trim());
}

// COMPLETED:
// 1. caesar.rs: rot13, caesar, vigenere
// 2. morse.rs: morse_encode, morse_decode
// 3. xor.rs: str, hex, byte, etc
// 4. base.rs: b2,b8,b16,b32,b64
