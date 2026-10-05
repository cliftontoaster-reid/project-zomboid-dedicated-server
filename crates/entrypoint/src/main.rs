use std::{fs, path::PathBuf};

use clap::Parser;

use crate::{config::Config, models::ServerConfig};

mod config;
mod healthcheck;
mod ini;
mod models;
mod permissions;
mod server;
mod util;

#[tokio::main]
async fn main() {
  let config = Config::parse();

  if config.healthcheck {
    std::process::exit(healthcheck::run(&config).await);
  }

  let data_path = PathBuf::from(&config.data_path);
  if !data_path.exists() {
    eprintln!(
      "Data path '{}' doesn't exist, creating the folder.",
      &config.data_path
    );
    fs::create_dir_all(&data_path)
      .unwrap_or_else(|_| panic!("Couldn't create folder {}", &config.data_path));
  }

  // Adjust ownership of the data path to PUID:PGID if running as root
  if let Err(err) = permissions::fix_permissions(&data_path, config.puid, config.pgid) {
    eprintln!("Warning: could not adjust permissions on data path: {err}");
  }

  let server_data_path = data_path.join("Server");
  if !server_data_path.exists() {
    eprintln!(
      "Server config path '{}' doesn't exist, creating the folder.",
      &config.data_path
    );
    fs::create_dir_all(&server_data_path)
      .unwrap_or_else(|_| panic!("Couldn't create folder {:?}", &server_data_path));
    let _ = permissions::fix_permissions(&server_data_path, config.puid, config.pgid);
  }

  let server_config_file = server_data_path.join(&format!("{}.ini", config.server_name));
  let mut server_config = if !server_config_file.exists() {
    eprintln!(
      "Server config file '{}' doesn't exist, creating a default config.",
      &config.data_path
    );
    let server_config = ServerConfig::default();

    server_config
      .write_config(&server_config_file)
      .unwrap_or_else(|_| panic!("Couldn't create file {:?}", &server_config_file));

    let _ = permissions::fix_permissions(&server_config_file, config.puid, config.pgid);
    server_config
  } else {
    ServerConfig::read_config(&server_config_file)
      .unwrap_or_else(|_| panic!("Couldn't read file {:?}", &server_config_file))
  };

  server_config.apply_cli(&config);

  server_config
    .write_config(&server_config_file)
    .unwrap_or_else(|_| panic!("Couldn't create file {:?}", &server_config_file));
  let _ = permissions::fix_permissions(&server_config_file, config.puid, config.pgid);

  let (control, stdin) = server::open_control(&data_path, config.puid, config.pgid)
    .expect("Couldn't create the control fifo");
  let mut control = tokio::fs::File::from_std(control);

  let child = server::launch_server(&config, stdin).await.unwrap();

  let code = server::wait_for_child(child, &mut control).await;

  std::process::exit(code);
}
