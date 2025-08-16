use atty::Stream;
use ice::app::cli::{parse_args, Command};
use pico_args::Arguments;
use std::{
    ffi::OsString,
    io::{self, Read},
};

fn print_usage() {
    println!("ICE - A simple CTF tool store.");
    println!("Usage: ice <SUBCOMMAND> [ARGUMENTS]");
    println!("Run 'ice help' for a list of available subcommands");
}

fn main() {
    let mut args: Vec<OsString> = std::env::args_os().skip(1).collect();

    if !atty::is(Stream::Stdin) {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        let input = input.trim();
        if !input.is_empty() {
            args.push(OsString::from(input));
        }
    }

    let mut args = Arguments::from_vec(args);

    let command = match parse_args(&mut args) {
        Ok(cmd) => cmd,
        Err(e) => {
            eprintln!("Error: {}", e);
            print_usage();
            return;
        }
    };

    match command {
        Command::Help => print_usage(),
        Command::Version => println!("ice {}", env!("CARGO_PKG_VERSION")),
        Command::Unknown(cmd) => {
            eprintln!("Error: Unknown subcommand '{}'", cmd);
            print_usage();
        }
        _ => command.run(),
    }
}
