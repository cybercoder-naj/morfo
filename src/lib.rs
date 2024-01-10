use std::io::Write;

use config::Config;

pub mod config;

pub fn execute<W: Write>(main_file: &str, config: Config, out: &mut W) {
    writeln!(out, "Executing main file: {}", main_file).unwrap();
    writeln!(out, "Config: {:?}", config).unwrap();
}

#[cfg(test)]
mod tests {
    use crate::config::ConfigBuilder;

    use super::*;

    #[test]
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
