use std::{
  fs, io,
  os::unix::process::CommandExt,
  path::{Path, PathBuf},
  process::Stdio,
  time::Duration,
};

use serde::Deserialize;
use tokio::{
  fs::File as AsyncFile,
  io::AsyncWriteExt,
  process::{Child, Command},
  signal::unix::{SignalKind, signal},
  time::sleep,
};

use crate::{config::Config, models::ServerConfig};

#[derive(Deserialize)]
pub struct LauncherConfig {
  #[serde(rename = "mainClass")]
  pub main_class: String,
  pub classpath: Vec<String>,
  #[serde(rename = "vmArgs")]
  pub vm_args: Vec<String>,
}

pub fn read_launcher_config(game_path: &Path, memory: &str) -> io::Result<LauncherConfig> {
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

pub fn java_command() -> PathBuf {
  match std::env::var("JAVA_HOME") {
    Ok(java_home) => Path::new(&java_home).join("bin").join("java"),
    Err(_) => PathBuf::from("java"),
  }
}

pub fn open_control(
  data_path: &Path,
  puid: u32,
  pgid: u32,
) -> io::Result<(std::fs::File, std::fs::File)> {
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

    if unsafe { libc::geteuid() } == 0 {
      if let Ok(c_path) = std::ffi::CString::new(path.to_str().unwrap_or_default()) {
        unsafe {
          libc::chown(c_path.as_ptr(), puid, pgid);
        }
      }
    }
  }

  let control = fs::OpenOptions::new().read(true).write(true).open(&path)?;
  let stdin = control.try_clone()?;

  Ok((control, stdin))
}

pub async fn launch_server(
  config: &Config,
  server_config: &ServerConfig,
  stdin: std::fs::File,
) -> io::Result<Child> {
  let game_path = PathBuf::from(config.game_path());
  let launcher = read_launcher_config(&game_path, config.memory())?;

  let mut cmd = Command::new(java_command());

  cmd.current_dir(&game_path);

  // If running as root, drop privileges to the requested PUID and PGID
  if unsafe { libc::geteuid() } == 0 {
    cmd.as_std_mut().uid(config.puid());
    cmd.as_std_mut().gid(config.pgid());
  }

  cmd.env("HOME", "/home/steam");

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
    .arg(config.server_name());

  if let Some(password) = config.admin_password() {
    cmd.arg("-adminpassword").arg(password);
  }

  cmd
    .arg("-port")
    .arg(server_config.default_port.to_string())
    .arg("-udpport")
    .arg(server_config.udp_port.to_string())
    .arg(format!("-cachedir={}", config.data_path()));

  if config.soft_reset() {
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

  cmd.stdin(Stdio::from(stdin));
  cmd.stdout(Stdio::inherit());
  cmd.stderr(Stdio::inherit());

  cmd.spawn()
}

pub async fn wait_for_child(mut child: Child, control: &mut AsyncFile) -> i32 {
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

pub async fn graceful_shutdown(control: &mut AsyncFile) {
  let _ = control.write_all(b"save\n").await;
  sleep(Duration::from_secs(15)).await;
  let _ = control.write_all(b"quit\n").await;
}
