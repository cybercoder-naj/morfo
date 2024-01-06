use std::{
    error::Error,
    path::{Path, PathBuf},
};

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
///     .build();
///
/// assert_eq!(config.get_cc(), "gcc");
/// assert_eq!(config.get_cflags(), &vec!["-O2"]);
/// ```
#[derive(Debug, serde::Deserialize)]
pub struct Config {
    cc: String,
    cflags: Vec<String>,
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
    /// assert_eq!(config.get_cflags(), &vec!["-O2"]);
    /// ```
    pub fn get_cflags(&self) -> &Vec<String> {
        &self.cflags
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
///
/// let config = ConfigBuilder::default()
///     .set_cc("gcc")
///     .add_cflag("-O2")
///     .build();
///
/// assert_eq!(config.get_cc(), "gcc");
/// assert_eq!(config.get_cflags(), &vec!["-O2"]);
/// ```
#[derive(Default)]
pub struct ConfigBuilder {
    cc: String,
    cflags: Vec<String>,
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

    pub fn build(self) -> Config {
        Config {
            cc: self.cc,
            cflags: self.cflags,
        }
    }
}

/// Reads the contents of a file
///
/// # Arguments
///
/// * `filepath` - The path to the file
///
/// # Returns
///
/// The contents of the file
///
/// # Errors
///
/// If the file cannot be read
///
/// # Examples
///
/// ```
/// let contents = morfo::config::read_config_file("./morfo.toml");
/// ```
pub fn read_config_file(filepath: &str) -> Result<String, Box<dyn Error>> {
    let contents = std::fs::read_to_string(filepath)?;
    Ok(contents)
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
pub fn find_config_file() -> Result<PathBuf, Box<dyn Error>> {
    let local_config = Path::new("./morfo.toml");
    if local_config.exists() {
        return Ok(local_config.to_path_buf());
    }

    let home = env!("HOME");
    let global_config = format!("{}/.config/morfo/config.toml", home);
    let home_config = Path::new(&global_config);
    if home_config.exists() {
        Ok(home_config.to_path_buf())
    } else {
        Err("No config file found".into())
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
/// let config = morfo::config::parse_config_file(Option::Some("./morfo.toml"));
/// ```
pub fn parse_config_file(filepath: Option<&str>) -> Result<Config, Box<dyn Error>> {
    let config = if let Some(filepath) = filepath {
        read_config_file(filepath)?
    } else {
        let config_file = find_config_file()?;
        let filepath = config_file
            .to_str()
            .ok_or("Invalid UTF-8 sequence in file path")?;
        read_config_file(filepath)?
    };

    let config: Config = toml::from_str(&config)?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use std::{
        fs::{self, File},
        io::Write,
    };
    use tempfile::NamedTempFile;

    use super::*;

    fn go_to_directory(dir: &str) -> PathBuf {
        let original_dir = std::env::current_dir().unwrap();

        let cargo_manifest_dir = env!("CARGO_MANIFEST_DIR");
        let working_dir = Path::new(dir);
        if !cargo_manifest_dir.starts_with("error") {
            let working_dir = Path::new(cargo_manifest_dir).join(working_dir);
            std::env::set_current_dir(working_dir).unwrap();
        } else {
            std::env::set_current_dir(working_dir).unwrap();
        }

        original_dir
    }

    #[test]
    fn test_read_config_file() {
        // SETUP
        // Create a temporary file to write toml_contents to
        let toml_contents = r#"
            key = 'value'"#;

        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "{}", toml_contents).unwrap();
        let temp_path = temp_file.path().to_str().unwrap();

        // TEST FUNCTION
        let contents = read_config_file(temp_path);

        // ASSERTIONS
        assert!(contents.is_ok());
        assert_eq!(contents.unwrap(), toml_contents);
    }

    #[test]
    fn test_find_local_config_file() {
        // SETUP
        let original_dir = go_to_directory("examples/custom_build");

        // TEST FUNCTION
        let config_file = find_config_file();

        // ASSERTIONS
        // Assert that the local config file is found
        assert!(config_file.is_ok());
        assert_eq!(config_file.unwrap().to_str().unwrap(), "./morfo.toml");

        // TEARDOWN
        std::env::set_current_dir(original_dir).unwrap();
    }

    #[test]
    fn test_find_global_config_file() {
        // SETUP
        let original_dir = go_to_directory("examples/hello_world");

        // Create the global config file
        let home = env!("HOME");
        let global_config = &format!("{}/.config/morfo/config.toml", home);
        let file_path = Path::new(global_config);

        // If the file exists, save the contents for teardown
        let original_file = if file_path.exists() {
            Some(file_path.to_str().unwrap())
        } else {
            None
        };
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        let _file = File::create(file_path).unwrap();

        // TEST FUNCTION
        let config_file = find_config_file();

        // ASSERTIONS
        assert!(config_file.is_ok());
        assert_eq!(config_file.unwrap().to_str().unwrap(), global_config);

        // TEARDOWN
        if let Some(data) = original_file {
            fs::write(file_path, data).unwrap();
        } else {
            fs::remove_file(file_path).unwrap();
        }
        std::env::set_current_dir(original_dir).unwrap();
    }

    #[test]
    fn test_parse_config_file() {
        // SETUP
        // Create a temporary file to write toml_contents to
        let toml_contents = r#"
            cc = 'gcc'
            cflags = ['-Wall', '-Wextra']"#;

        let mut temp_file = NamedTempFile::new().unwrap();
        write!(temp_file, "{}", toml_contents).unwrap();
        let temp_path = temp_file.path().to_str().unwrap();

        // TEST FUNCTION
        let config = parse_config_file(Option::Some(temp_path));

        // ASSERTIONS
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.cc, "gcc");
        assert_eq!(config.cflags, vec!["-Wall", "-Wextra"]);
    }
}
