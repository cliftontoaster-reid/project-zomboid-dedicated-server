use std::error::Error;
use std::path::PathBuf;

pub fn load_os_config(config_dir: &str, os_name: &str) -> Result<crate::models::OsConfig, String> {
  let file_path = PathBuf::from(config_dir).join(format!("{os_name}.toml"));
  let content = std::fs::read_to_string(&file_path)
    .map_err(|err| format!("Failed to load config at {}: {err}", file_path.display()))?;

  toml::from_str(&content)
    .map_err(|err| format!("Failed to parse config at {}: {err}", file_path.display()))
}

pub fn discover_os_configs(config_dir: &str) -> Result<Vec<String>, Box<dyn Error>> {
  let mut os_names = Vec::new();

  for entry in std::fs::read_dir(config_dir)? {
    let entry = entry?;
    let name = entry.file_name().to_string_lossy().into_owned();

    if entry.file_type()?.is_file() && name.ends_with(".toml") {
      os_names.push(name.trim_end_matches(".toml").to_string());
    }
  }

  os_names.sort();
  Ok(os_names)
}
