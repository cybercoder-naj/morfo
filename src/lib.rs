//! An automatic C/C++ compiler and runner.
//!
//! # Usage
//!
//! A basic usage to execute a C program to stdout would be:
//!
//! ```rust
//! use morfo::config::ConfigBuilder;
//! use morfo::execute;
//!
//! fn main() {
//!    let config = ConfigBuilder::default().build();
//!    execute("main.c", config, &mut std::io::stdout(), vec![]);
//! }
//! ```

use std::{
    env,
    fs::create_dir,
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

use act::ACT;
use config::Config;
use error::{MorfoError, MorfoResult};

mod act;
pub mod config;
pub mod error;
mod utils;

pub fn execute<W: Write>(
    main_file: &str,
    config: Config,
    out: &mut W,
    prog_args: Vec<String>,
) -> MorfoResult<()> {
    let act = ACT::build(main_file);
    compile(&act, &config)?;

    run(act, &config, out, prog_args)?;
    Ok(())
}

fn compile(act: &ACT, config: &Config) -> MorfoResult<()> {
    // create .out directory if it doesn't exist
    if !Path::new(&config.get_build_dir()).exists() {
        create_dir(config.get_build_dir())?;
    }

    for dependency in &act.dependencies {
        compile(dependency, config)?;
    }

    // use command to print pwd
    let mut compile_cmd = Command::new(config.get_cc());
    if config.get_cflags().len() != 0 {
        compile_cmd
            .arg(config.get_cflags().join(" ").as_str());
    }
    compile_cmd
        .arg(&act.name)
        .arg("-o")
        .arg(config.get_build_dir().join(utils::file_name(&act.name)));

    if env::var("VERBOSITY").unwrap_or_default() == "1" {
        println!("{}", format!("{:?}", compile_cmd).replace("\"", ""));
    }

    let status = compile_cmd.status()?;
    match status.code() {
        Some(code) => {
            if code != 0 {
                return Err(MorfoError::CompilationFailure(code.into()));
            }
        }
        None => return Err(MorfoError::CompilationFailure(Option::None)),
    }

    Ok(())
}

fn run<W: Write>(
    act: ACT,
    config: &Config,
    out: &mut W,
    prog_args: Vec<String>,
) -> MorfoResult<()> {
    let executable = config.get_build_dir().join(utils::file_name(&act.name));
    if !executable.exists() {
        return Err(MorfoError::MissingExecutable);
    }

    // use command to invoke the executable
    let mut run_cmd = Command::new(executable);
    for arg in prog_args {
        run_cmd.arg(arg);
    }
    run_cmd
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit());

    if env::var("VERBOSITY").unwrap_or_default() == "1" {
        println!("{}", format!("{:?}", run_cmd).replace("\"", ""));
    }
    println!("");

    // pipe the output to out
    let run_project = run_cmd.output()?;
    out.write_all(&run_project.stdout)?;

    Ok(())
}
