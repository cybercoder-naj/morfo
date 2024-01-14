use std::{env, error::Error, fs::create_dir, io::Write, path::Path, process::Command};

use act::ACT;
use config::Config;

mod act;
pub mod config;
mod utils;

pub fn execute<W: Write>(
    main_file: &str,
    config: Config,
    out: &mut W,
) -> Result<(), Box<dyn Error>> {
    let act = ACT::build(main_file);
    compile(&act, &config, out)?;

    run(main_file, &config, out)?;
    Ok(())
}

fn compile<W: Write>(act: &ACT, config: &Config, out: &mut W) -> Result<(), Box<dyn Error>> {
    // create .out directory if it doesn't exist
    if !Path::new(".out").exists() {
        create_dir(".out")?;
    }

    for dependency in &act.dependencies {
        compile(dependency, config, out)?;
    }

    // use command to print pwd
    let mut compile_cmd = Command::new(config.get_cc());
    compile_cmd
        .arg(config.get_cflags().join(" ").as_str())
        .arg(&act.name)
        .arg("-o")
        .arg(config.get_build_dir().join(utils::file_name(&act.name)));

    if env::var("VERBOSITY").unwrap_or_default() == "1" {
        writeln!(out, "{}", format!("{:?}", compile_cmd).replace("\"", ""))?;
    }

    let status = compile_cmd.status()?;
    if !status.success() {
        return Err(format!("Error compiling {}", act.name).into());
    }

    Ok(())
}

fn run<W: Write>(main_file: &str, config: &Config, out: &mut W) -> Result<(), Box<dyn Error>> {
    let executable = config.get_build_dir().join(utils::file_name(main_file));
    if !executable.exists() {
        return Err(format!("Executable {} does not exist", executable.display()).into());
    }

    // use command to invoke the executable
    let run_project = std::process::Command::new(executable)
        .arg(main_file)
        .output()?;

    // pipe the output to out
    out.write_all(&run_project.stdout)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::config::ConfigBuilder;

    use super::*;

    #[test]
    #[ignore]
    fn test_execute() {
        let mut out = Vec::new();
        execute(
            "examples/hello_world/main.c",
            ConfigBuilder::default().set_cc("gcc").build(),
            &mut out,
        )
        .unwrap();
        assert_eq!(
            String::from_utf8(out).unwrap(),
            vec!["Hello World!", ""].join("\n")
        );
    }
}
