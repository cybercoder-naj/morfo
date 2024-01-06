use std::fs;

fn main() {
  let home = env!("HOME");
  let config_path = format!("{}/.config/morfo/config.toml", home);

  let config = fs::read("./config.toml").unwrap();
  fs::write(config_path, config).unwrap();
}