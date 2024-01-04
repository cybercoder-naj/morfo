use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The main file to build
    #[arg(value_name = "main")]
    main: String,

    /// The config file to use
    #[arg(
        long,
        default_value = "$HOME/.config/morfo/config.toml",
        value_name = "config"
    )]
    config: String,

    /// Display all the build steps
    #[arg(short, long, default_value = "false")]
    verbose: bool,
}

fn main() {
    let args = Cli::parse();

    println!("morfo {}", args.verbose);
}
