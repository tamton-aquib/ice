use clap::Parser;
use std::io::{self, IsTerminal, Read};

fn main() {
    let mut args: Vec<String> = std::env::args_os()
        .map(|s| s.into_string().unwrap())
        .collect();

    if args.len() == 2 && (args[1] == "-h" || args[1] == "--help") {
        ice::app::cli::print_help();
        return;
    }

    let stdin_has_content = !io::stdin().is_terminal();

    if stdin_has_content {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        let input = input.trim();
        if !input.is_empty() {
            args.push(input.to_string());
        }
    }

    let candidate = args
        .get(1)
        .filter(|a| !a.starts_with('-'))
        .cloned();

    let cli = ice::app::cli::Cli::try_parse_from(args).unwrap_or_else(|e| {
        let suggestion = if e.kind() == clap::error::ErrorKind::InvalidSubcommand {
            candidate
                .as_deref()
                .and_then(ice::utils::fuzzy::suggest_subcommand)
        } else {
            None
        };
        match suggestion {
            Some(list) => {
                eprintln!("{}", ice::utils::color::red(&format!("Error: {}", e)));
                eprintln!("{}", ice::utils::color::yellow(&format!("Did you mean: {}", list)));
            }
            None => eprintln!("{}", ice::utils::color::red(&format!("Error: {}", e))),
        }
        std::process::exit(1);
    });

    if let Err(e) = cli.command.run() {
        eprintln!("{}", ice::utils::color::red(&format!("Error: {}", e)));
        std::process::exit(1);
    }
}
