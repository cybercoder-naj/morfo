//! The configuration for morfo.
//!
//! This module contains the [`Config`] struct and [`ConfigBuilder`] struct.
//! You can either use [`parse_config_file`] to parse a config file, or use the ConfigBuilder to create a config.
//!
//! [`Config`]: struct.Config.html
//! [`ConfigBuilder`]: struct.ConfigBuilder.html
//! [`parse_config_file`]: fn.parse_config_file.html

use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::error::{MorfoError, MorfoResult};

/// `Config` holds the configuration for the compiler.
///
/// It includes the compiler command (`cc`) and the compiler flags (`cflags`).
/// This struct is created by [`ConfigBuilder`].
/// This struct can be created from a config file using [`parse_config_file`].
///
/// [`ConfigBuilder`]: struct.ConfigBuilder.html
/// [`parse_config_file`]: fn.parse_config_file.html
///
/// # Examples
///
/// ```
/// use morfo::config::ConfigBuilder;
///
/// let config = ConfigBuilder::default()
///     .set_cc("gcc")
///     .add_cflag("-O2")
///     .set_build_dir(".out")
///     .build();
///
/// assert_eq!(config.get_cc(), "gcc");
/// assert_eq!(config.get_cflags(), vec!["-O2"]);
/// ```
#[derive(Debug, serde::Deserialize)]
pub struct Config {
    cc: String,
    cflags: Option<Vec<String>>,
    builddir: Option<String>,
    includes: Option<Vec<String>>,
}

impl Config {
    /// Returns the compiler command.
    ///
    /// # Examples
    ///
    /// ```
    /// use morfo::config::ConfigBuilder;
    ///
    /// let config = ConfigBuilder::default().set_cc("gcc").build();
    /// assert_eq!(config.get_cc(), "gcc");
    /// ```
    pub fn get_cc(&self) -> &String {
        &self.cc
    }

    /// Returns the compiler flags.
    ///
    /// # Examples
    ///
    /// ```
    /// use morfo::config::ConfigBuilder;
    ///
    /// let config = ConfigBuilder::default().add_cflag("-O2").build();
    /// assert_eq!(config.get_cflags(), vec!["-O2"]);
    /// ```
    pub fn get_cflags(&self) -> Vec<String> {
        self.cflags.clone().unwrap_or_default()
    }

    /// Returns the build directory.
    /// If the build directory is not set, it will return ".out".
    ///
    /// # Examples
    ///
    /// ```
    /// use morfo::config::ConfigBuilder;
    /// use std::path::PathBuf;
    ///
    /// let config = ConfigBuilder::default().set_build_dir(".build").build();
    /// assert_eq!(config.get_build_dir(), PathBuf::from(".build"));
    /// ```
    pub fn get_build_dir(&self) -> PathBuf {
        match &self.builddir {
            Some(build_dir) => Path::new(build_dir).to_path_buf(),
            None => Path::new(".out").to_path_buf(),
        }
    }

    /// Returns the include directories.
    /// If the include directories are not set, it will return an empty vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use morfo::config::ConfigBuilder;
    ///
    /// let config = ConfigBuilder::default().build();
    /// assert_eq!(config.get_includes(), Vec::<String>::new());
    /// ```
    pub fn get_includes(&self) -> Vec<String> {
        self.includes.clone().unwrap_or_default()
    }
}

/// `ConfigBuilder` is a builder for [`Config`].
///
/// [`Config`]: struct.Config.html
///
/// # Examples
///
/// ```
/// use morfo::config::ConfigBuilder;
/// use std::path::PathBuf;
///
/// let config = ConfigBuilder::default()
///     .set_cc("gcc")
///     .add_cflag("-O2")
///     .set_build_dir(".out")
///     .add_include("include")
///     .build();
///
/// assert_eq!(config.get_cc(), "gcc");
/// assert_eq!(config.get_cflags(), vec!["-O2"]);
/// assert_eq!(config.get_build_dir(), PathBuf::from(".out"));
/// assert_eq!(config.get_includes(), vec!["include"]);
/// ```
#[derive(Default)]
pub struct ConfigBuilder {
    cc: String,
    cflags: Vec<String>,
    build_dir: Option<PathBuf>,
    includes: Vec<PathBuf>,
}

impl ConfigBuilder {
    pub fn set_cc(mut self, cc: &str) -> Self {
        self.cc = cc.to_string();
        self
    }

    pub fn add_cflag(mut self, cflag: &str) -> Self {
        self.cflags.push(cflag.to_string());
        self
    }

    pub fn set_build_dir(mut self, build_dir: &str) -> Self {
        self.build_dir = Some(Path::new(build_dir).to_path_buf());
        self
    }

    pub fn add_include(mut self, include: &str) -> Self {
        self.includes.push(Path::new(include).to_path_buf());
        self
    }

    pub fn build(self) -> Config {
        Config {
            cc: self.cc,
            cflags: Option::Some(self.cflags),
            builddir: self.build_dir.map(|p| p.to_str().unwrap().to_string()),
            includes: self
                .includes
                .iter()
                .map(|p| p.to_str().unwrap().to_string())
                .collect::<Vec<String>>()
                .into(),
        }
    }
}

/// Finds the config file in the following order:
///   1. If there is a local config file (./morfo.toml).
///   2. If there is a global config file (~/.config/morfo/config.toml).
///   3. If there is no config file, return an error.
///
/// # Returns
///
/// The path to the config file
///
/// # Errors
///
/// If there is no config file
///
/// # Examples
///
/// ```
/// let config_file = morfo::config::find_config_file();
/// ```
pub fn find_config_file() -> MorfoResult<PathBuf> {
    let local_config = Path::new("./morfo.toml");
    if local_config.exists() {
        return Ok(local_config.to_path_buf());
    }

    let home = dirs::home_dir().ok_or(MorfoError::MissingHomeDirectory)?;
    let global_config = home.join(".config/morfo/config.toml");
    let home_config = Path::new(&global_config);
    if home_config.exists() {
        Ok(home_config.to_path_buf())
    } else {
        Err(MorfoError::MissingConfigFile)
    }
}

/// Parses the correct config file.
/// If the filepath is provided, it will parse that file.
/// If the filepath is not provided, it will find the config file and parse that.
///
/// # Arguments
///
/// * `filepath` - The path to the config file. If None, it will find the config file.
///
/// # Returns
///
/// The parsed config file
///
/// # Errors
///
/// If the config file cannot be read, found, or parsed.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
///
/// let config = morfo::config::parse_config_file(&PathBuf::from("./morfo.toml"));
/// ```
pub fn parse_config_file(filepath: &PathBuf) -> MorfoResult<Config> {
    // assert the file exists
    if !filepath.exists() {
        return Err(MorfoError::FileNotFound(filepath.clone()));
    }

    // assert the file is a TOML file
    let ext = filepath.extension().ok_or(MorfoError::InvalidUnicode)?;
    if ext != "toml" {
        let ext = ext.to_str().ok_or(MorfoError::InvalidUnicode)?;
        return Err(MorfoError::InvalidConfigExtension(ext.to_owned()));
    }

    let config = fs::read_to_string(filepath)?;

    let config: Config = toml::from_str(&config)?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use serial_test::serial;
    use std::{
        env,
        fs::{self, File},
        io::Write,
    };

    use super::*;

    #[test]
    #[serial]
    fn config_find_local_file() {
        // SETUP
        let cargo_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        env::set_current_dir(cargo_path.join("examples/custom_build")).unwrap();

        // TEST FUNCTION
        let config_file = find_config_file();

        // ASSERTIONS
        // Assert that the local config file is found
        assert!(config_file.is_ok());
        assert_eq!(config_file.unwrap().to_str().unwrap(), "./morfo.toml");

        // TEARDOWN
        std::env::set_current_dir(cargo_path).unwrap();
    }

    #[test]
    #[serial]
    fn config_find_global_file() {
        // SETUP
        let cargo_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        env::set_current_dir(cargo_path.join("examples/hello_world")).unwrap();

        // Create the global config file
        let home = dirs::home_dir().unwrap();
        let global_config_path = home.join(".config/morfo/config.toml");

        // If the file does not exist, create it
        let mut remove_file = false;
        if !global_config_path.exists() {
            File::create(&global_config_path).unwrap();
            remove_file = true;
        }

        // TEST FUNCTION
        let config_file = find_config_file();

        // ASSERTIONS
        assert!(config_file.is_ok());
        assert_eq!(config_file.unwrap(), global_config_path);

        // TEARDOWN
        if remove_file {
            fs::remove_file(global_config_path).unwrap();
        }
        std::env::set_current_dir(cargo_path).unwrap();
    }

    #[test]
    fn config_parse_file() {
        // SETUP
        // Create a temporary file to write toml_contents to
        let toml_contents = r#"
            cc = 'gcc'
            cflags = ['-Wall', '-Wextra']
            builddir = ".build"
            includes = ["src/include", "src/aux/include"]"#;

        let temp_dir = tempfile::tempdir().unwrap();
        let temp_path = temp_dir.path().join("config.toml");
        let mut temp_file = File::create(&temp_path).unwrap();
        temp_file.write_all(toml_contents.as_bytes()).unwrap();

        // TEST FUNCTION
        let config = parse_config_file(&temp_path);

        // ASSERTIONS
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.cc, "gcc");
        assert_eq!(config.cflags.unwrap(), vec!["-Wall", "-Wextra"]);
        assert!(config.builddir.is_some());
        assert_eq!(config.builddir.unwrap(), ".build");
        assert!(config.includes.is_some());
        assert_eq!(
            config.includes.unwrap(),
            vec!["src/include", "src/aux/include"]
        );
    }

    #[test]
    fn config_parse_filepath_does_not_exist() {
        let filepath = PathBuf::from("something/that/does/not/exist.toml");
        let config = parse_config_file(&filepath);

        assert!(config.is_err());
        assert_eq!(config.unwrap_err(), MorfoError::FileNotFound(filepath));
    }

    #[test]
    fn config_parse_non_toml_file() {
        // SETUP
        // Create a temporary file to write some text to
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_path = temp_dir.path().join("config.txt");
        let mut temp_file = File::create(&temp_path).unwrap();
        temp_file.write_all(b"some text").unwrap();

        let config = parse_config_file(&temp_path);

        assert!(config.is_err());
        assert_eq!(
            config.unwrap_err(),
            MorfoError::InvalidConfigExtension("txt".to_owned())
        );
    }

    #[test]
    fn config_parse_invalid_toml_file() {
        // SETUP
        // Create a temporary file to write some text to
        let temp_dir = tempfile::tempdir().unwrap();
        let temp_path = temp_dir.path().join("config.toml");
        let mut temp_file = File::create(&temp_path).unwrap();

        let toml_contents = r#"
            builddir = ".build"
            includes = ["src/include", "src/aux/include"]"#;
        write!(temp_file, "{}", toml_contents).unwrap();

        let config = parse_config_file(&temp_path);

        assert!(config.is_err());
        assert_eq!(
            config.unwrap_err(),
            MorfoError::InvlidConfig("missing field `cc`".to_owned())
        );
    }
}
