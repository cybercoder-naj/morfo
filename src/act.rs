use std::{
    error::Error,
    fs::create_dir,
    path::{Path, PathBuf},
    process::Command,
};

use crate::config::Config;

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
        let build_project = Command::new(config.get_cc())
            .arg(config.get_cflags().join(" ").as_str())
            .arg(&self.name)
            .arg("-o")
            .arg(config.get_build_dir().join(self.file_name()))
            .status()?;

        Ok(())
    }

    // file name of a path without the extension
    fn file_name(&self) -> String {
        let file_name = self.name.split("/").last().unwrap().to_string();
        file_name.split(".").next().unwrap().to_string()
    }
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

    #[test]
    fn test_act_file_name() {
        let act = ACT::new("main.c");
        assert_eq!(act.file_name(), "main");

        let act = ACT::new("src/main.cpp");
        assert_eq!(act.file_name(), "main");
    }
}
