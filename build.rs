use std::fs;

fn main() {
    let home = env!("HOME");
    let config_dir = format!("{}/.config/morfo/", home);
    let config_path = format!("{}/config.toml", config_dir);

    // Create directory if it doesn't exist
    fs::create_dir_all(config_dir).unwrap();

    // Copy the file
    fs::copy("./config.toml", config_path).unwrap();
}
