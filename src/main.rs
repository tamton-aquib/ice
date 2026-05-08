use clap::Parser;
use std::io::{self, IsTerminal, Read};

fn main() {
    let mut args: Vec<String> = std::env::args_os()
        .map(|s| s.into_string().unwrap())
        .collect();

    let stdin_has_content = !io::stdin().is_terminal();

    if stdin_has_content {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        let input = input.trim();
        if !input.is_empty() {
            args.push(input.to_string());
        }
    }

    let cli = ice::app::cli::Cli::try_parse_from(args).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });

    cli.command.run();
}
