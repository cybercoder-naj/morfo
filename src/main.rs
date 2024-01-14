use std::process;

use clap::Parser;
use colored::Colorize;
use morfo::{config::parse_config_file, execute};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The main file to execute
    #[arg(value_name = "main")]
    main: String,

    /// The directory to use
    #[arg(short, long, default_value = ".")]
    dir: String,

    /// The config file to use
    #[arg(long, value_name = "config")]
    config: Option<String>,

    /// Display all the build steps
    #[arg(short, long, default_value = "false")]
    verbose: bool,
}

fn main() {
    let args = Cli::parse();

    let config = parse_config_file(args.config.as_deref());
    if config.is_err() {
        eprintln!(
            "{}",
            format!("Error parsing config file: {:?}", config).red()
        );
        process::exit(1);
    }
    let config = config.unwrap();

    let result = execute(&args.main, config, &mut std::io::stdout());
    if result.is_err() {
        eprintln!("{}", format!("Error executing: {:?}", result).red());
        process::exit(1);
    }
}
