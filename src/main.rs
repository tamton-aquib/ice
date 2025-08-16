use pico_args::Arguments;

mod cli;
use cli::{Command, parse_args};

fn print_usage() {
    println!("ICE - A command-line tool for various ciphers, encoding, and text manipulation.\n");
    println!("USAGE:");
    println!("    ice <SUBCOMMAND> [ARGUMENTS]");
    println!("    ice help");
    println!("\nFor a list of available subcommands, run `ice help`.");
}

fn main() {

    let mut args = Arguments::from_env();

    let command = match parse_args(&mut args) {
        Ok(cmd) => cmd,
        Err(e) => {
            eprintln!("Error: {}", e);
            print_usage();
            return;
        }
    };

    match command {
        Command::Help => {
            print_usage();
        }
        Command::Unknown(cmd) => {
            eprintln!("Error: Unknown subcommand '{}'", cmd);
            print_usage();
        }
        _ => {
            command.run();
        }
    }
}
