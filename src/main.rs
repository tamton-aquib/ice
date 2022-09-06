use ctfu::base;
use ctfu::caesar;
use ctfu::general;
use ctfu::manipulation;
use ctfu::morse;
use ctfu::xor;
// TODO: add colors
// TODO: complete xor.rs
// TODO: start general.rs
// TODO: complete caesar::vigenere()

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        println!("Please enter 2 args atleast!");
        return;
    }

    let c_type = &args[1];
    let query = &args[2].trim();

    let res = match c_type.trim() {
        // Caesar commands
        "caesar" | "ceasar" => caesar::caesar(query),
        "rot13" | "rot" => caesar::rot13(query),
        "vigenere" => {
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
        "hex" | "b16" => base::hexadecimal(query),
        "binary" | "bin" => base::binary(query),

        // Manipulation commands
        "lower" => manipulation::lower(query),
        "upper" => manipulation::lower(query),
        "remove_whitespace" | "rw" => manipulation::lower(query),

        // General
        "a1z26" | "az" => general::a1z26(query),

        _ => String::from("Subcommand not found!"),
    };

    println!("Result\n======\n{}", res);
}

// COMPLETED:
// 1. caesar.rs: rot13, caesar, vigenere
// 2. morse.rs: morse_encode, morse_decode
// 3. xor.rs: str, hex, byte, etc
// 4. base.rs: b64, b32, octal, hexadecimal
