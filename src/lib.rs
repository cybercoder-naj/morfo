use std::io::Write;

use config::Config;

mod act;
pub mod config;

pub fn execute<W: Write>(main_file: &str, config: Config, out: &mut W) {
    let mut act = act::ACT::new(main_file);
    act.build(&config);
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
