use std::{env, path::PathBuf, process};

use clap::Parser;
use colored::Colorize;
use morfo::{
    config::{find_config_file, parse_config_file},
    execute,
};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The main file to execute
    #[arg(value_name = "main")]
    main: String,

    /// The arguments to pass to the main file
    #[arg(value_name = "args")]
    args: Vec<String>,

    /// The config file to use
    #[arg(long, value_name = "config")]
    config: Option<PathBuf>,

    /// Display all the build steps
    #[arg(short, long, default_value = "false")]
    verbose: bool,
}

fn main() {
    let args = Cli::parse();

    if args.verbose {
        env::set_var("VERBOSITY", "1");
    }

    let config_path = args.config.unwrap_or_else(|| {
        let config_path = find_config_file();
        if config_path.is_err() {
            eprintln!(
                "{}",
                format!("Error finding config file: {:?}", config_path).red()
            );
            process::exit(1);
        }
        config_path.unwrap()
    });

    let config = parse_config_file(&config_path);
    if config.is_err() {
        eprintln!(
            "{}",
            format!("Error parsing config file: {:?}", config).red()
        );
        process::exit(1);
    }
    let config = config.unwrap();

    let result = execute(&args.main, config, &mut std::io::stdout(), args.args);
    if result.is_err() {
        eprintln!("{}", format!("Error executing: {:?}", result).red());
        process::exit(1);
    }
}
