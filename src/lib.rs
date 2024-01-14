use std::{error::Error, io::Write};

use config::Config;

mod act;
pub mod config;
mod utils;

pub fn execute<W: Write>(
    main_file: &str,
    config: Config,
    out: &mut W,
) -> Result<(), Box<dyn Error>> {
    let mut act = act::ACT::new(main_file);
    act.build(&config)?;

    run(main_file, &config, out)?;
    Ok(())
}

fn run<W: Write>(main_file: &str, config: &Config, out: &mut W) -> Result<(), Box<dyn Error>> {
    let executable = config.get_build_dir().join(utils::file_name(main_file));

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
        execute("main.morfo", ConfigBuilder::default().build(), &mut out);
        assert_eq!(
            String::from_utf8(out).unwrap(),
            vec![
                "Executing main file: main.morfo",
                "Config: Config { cc: \"\", cflags: [] }\n"
            ]
            .join("\n")
        );
    }
}
