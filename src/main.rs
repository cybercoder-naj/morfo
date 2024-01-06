use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The main file to build
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

    println!("morfo {:?}", args);
}
