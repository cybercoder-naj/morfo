use std::{error::Error, fs::create_dir, path::Path, process::Command};

use crate::config::Config;
use crate::utils;

#[derive(Debug, PartialEq)]
pub struct ACT {
    pub name: String,
    pub header: Option<String>,
    pub linkers: Vec<String>,
    pub dependencies: Vec<ACT>,
}

impl ACT {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            header: Option::default(),
            linkers: Vec::default(),
            dependencies: Vec::default(),
        }
    }

    pub fn build(&mut self, config: &Config) -> Result<(), Box<dyn Error>> {
        // create .out directory if it doesn't exist
        if !Path::new(".out").exists() {
            create_dir(".out")?;
        }

        // use command to print pwd
        Command::new(config.get_cc())
            .arg(config.get_cflags().join(" ").as_str())
            .arg(&self.name)
            .arg("-o")
            .arg(config.get_build_dir().join(utils::file_name(&self.name)))
            .status()?;

        Ok(())
    }

    // file name of a path without the extension
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_act_new() {
        let act = ACT::new("main.c");
        assert_eq!(
            act,
            ACT {
                name: "main.c".to_string(),
                header: None,
                linkers: Vec::default(),
                dependencies: Vec::default(),
            }
        );
    }
}
