use std::{error::Error, path::Path};

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

/// Finds the config file
///   1. If there is a local config file (./morfo.toml), use that.
///   2. If there is a global config file (~/.config/morfo/config.toml), use that.
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
pub fn find_config_file() -> Result<&'static Path, Box<dyn Error>> {
    let local_config = Path::new("./morfo.toml");
    if local_config.exists() {
        return Ok(local_config);
    }

    let home_config = Path::new("~/.config/morfo/config.toml");
    if home_config.exists() {
        Ok(home_config)
    } else {
        Err("No config file found".into())
    }
}

/// Parses the config file
pub fn parse_config_file(config: &str) -> Result<(), Box<dyn Error>> {
    let config: toml::Value = toml::from_str(config)?;
    println!("{:?}", config);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{
        fs::{self, File},
        io::Write,
    };
    use tempfile::NamedTempFile;

    use super::*;

    #[test]
    fn test_read_config_file() {
        // SETUP
        // Create a temporary file to write toml_contents to
        let toml_contents = r#"
            [morfo]
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
        let original_dir = std::env::current_dir().unwrap();

        // examples/custom_build has a morfo.toml file
        std::env::set_current_dir(Path::new("./examples/custom_build")).unwrap();

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
        let original_dir = std::env::current_dir().unwrap();

        // examples/hello_world has no morfo.toml file
        std::env::set_current_dir(Path::new("examples/hello_world")).unwrap();

        // Create the global config file
        let global_config = "~/.config/morfo/config.toml";
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
}
