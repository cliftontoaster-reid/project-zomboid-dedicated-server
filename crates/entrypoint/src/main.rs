use std::{
  fs, io,
  path::{Path, PathBuf},
  process::Stdio,
  time::Duration,
};

use clap::Parser;
use serde::Deserialize;
use tokio::{
  fs::File as AsyncFile,
  io::AsyncWriteExt,
  process::{Child, Command},
  signal::unix::{SignalKind, signal},
  time::sleep,
};

use crate::{config::Config, models::ServerConfig};

mod config;
mod healthcheck;
mod models;

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
    fs::create_dir_all(&data_path).expect(&format!("Couldn't create folder {}", &config.data_path));
  }

  let server_data_path = data_path.join("Server");
  if !server_data_path.exists() {
    eprintln!(
      "Server config path '{}' doesn't exist, creating the folder.",
      &config.data_path
    );
    fs::create_dir_all(&server_data_path)
      .expect(&format!("Couldn't create folder {:?}", &server_data_path));
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
      .expect(&format!("Couldn't create file {:?}", &server_config_file));

    server_config
  } else {
    ServerConfig::read_config(&server_config_file)
      .expect(&format!("Couldn't read file {:?}", &server_config_file))
  };

  server_config.apply_cli(&config);

  server_config
    .write_config(&server_config_file)
    .expect(&format!("Couldn't create file {:?}", &server_config_file));

  let control = open_control(&data_path).expect("Couldn't create the control fifo");
  let stdin = control.try_clone().expect("Couldn't open the control fifo");
  let mut control = AsyncFile::from_std(control);

  let child = launch_server(&config, stdin).await.unwrap();

  let code = wait_for_child(child, &mut control).await;

  std::process::exit(code);
}

async fn wait_for_child(mut child: Child, control: &mut AsyncFile) -> i32 {
  let mut sigterm = signal(SignalKind::terminate()).expect("Couldn't listen for SIGTERM");
  let mut sigint = signal(SignalKind::interrupt()).expect("Couldn't listen for SIGINT");
  let mut stopping = false;

  let status = loop {
    tokio::select! {
      status = child.wait() => break status,
      _ = sigterm.recv(), if !stopping => {
        stopping = true;
        graceful_shutdown(control).await;
      }
      _ = sigint.recv(), if !stopping => {
        stopping = true;
        graceful_shutdown(control).await;
      }
    }
  };

  match status {
    Ok(status) => status.code().unwrap_or(1),
    Err(_) => 1,
  }
}

async fn graceful_shutdown(control: &mut AsyncFile) {
  let _ = control.write_all(b"save\n").await;
  sleep(Duration::from_secs(15)).await;
  let _ = control.write_all(b"quit\n").await;
}

fn open_control(data_path: &Path) -> io::Result<std::fs::File> {
  let path = data_path.join("zomboid.control");

  if !path.exists() {
    let status = std::process::Command::new("mkfifo")
      .arg(&path)
      .stdout(Stdio::null())
      .stderr(Stdio::null())
      .status();

    match status {
      Ok(status) if status.success() => {}
      _ => return Err(io::Error::other("Couldn't create the control fifo")),
    }
  }

  fs::OpenOptions::new().read(true).write(true).open(path)
}

#[derive(Deserialize)]
struct LauncherConfig {
  #[serde(rename = "mainClass")]
  main_class: String,
  classpath: Vec<String>,
  #[serde(rename = "vmArgs")]
  vm_args: Vec<String>,
}

fn read_launcher_config(game_path: &Path, memory: &str) -> io::Result<LauncherConfig> {
  let contents = fs::read_to_string(game_path.join("ProjectZomboid64.json"))?;
  let mut launcher: LauncherConfig = serde_json::from_str(&contents)
    .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;

  let mut heap = false;

  for arg in launcher.vm_args.iter_mut() {
    if arg.starts_with("-Xmx") {
      *arg = format!("-Xmx{memory}");
      heap = true;
    } else if arg.starts_with("-Xms") {
      *arg = format!("-Xms{memory}");
    }
  }

  if !heap {
    launcher.vm_args.push(format!("-Xmx{memory}"));
  }

  Ok(launcher)
}

fn java_command() -> PathBuf {
  match std::env::var("JAVA_HOME") {
    Ok(java_home) => Path::new(&java_home).join("bin").join("java"),
    Err(_) => PathBuf::from("java"),
  }
}

pub async fn launch_server(config: &Config, control: std::fs::File) -> io::Result<Child> {
  let game_path = PathBuf::from(&config.game_path);
  let launcher = read_launcher_config(&game_path, &config.memory)?;

  let mut cmd = Command::new(java_command());

  cmd.current_dir(&game_path);

  cmd.arg(format!(
    "-Djava.class.path={}",
    launcher.classpath.join(":")
  ));

  for arg in &launcher.vm_args {
    cmd.arg(arg);
  }

  cmd
    .arg(&launcher.main_class)
    .arg("-servername")
    .arg(&config.server_name)
    .arg("-adminpassword")
    .arg(&config.admin_password)
    .arg("-port")
    .arg(config.port.to_string())
    .arg("-udpport")
    .arg(config.steam_port.to_string())
    .arg(format!("-cachedir={}", config.data_path));

  if config.soft_reset {
    cmd.arg("-softreset");
  }

  cmd.env(
    "LD_LIBRARY_PATH",
    format!(
      "{}:{}",
      game_path.join("linux64").to_string_lossy(),
      game_path.to_string_lossy()
    ),
  );

  cmd.stdin(Stdio::from(control));
  cmd.stdout(Stdio::inherit());
  cmd.stderr(Stdio::inherit());

  cmd.spawn()
}
