use std::time::Duration;

use rcon::Connection;
use tokio::{net::TcpStream, time::timeout};

use crate::{config::Config, models::ServerConfig};

const TIMEOUT: Duration = Duration::from_secs(5);

pub async fn run(config: &Config) -> i32 {
  let persisted = if config.rcon_password.is_none() || config.rcon_port.is_none() {
    let path = std::path::Path::new(config.data_path())
      .join("Server")
      .join(format!("{}.ini", config.server_name()));
    ServerConfig::read_config(&path).ok()
  } else {
    None
  };
  let port = config
    .rcon_port
    .or_else(|| persisted.as_ref().map(|value| value.rcon_port as u16))
    .unwrap_or(27015);
  let password = config
    .rcon_password
    .as_deref()
    .or_else(|| persisted.as_ref().map(|value| value.rcon_password.as_str()))
    .unwrap_or("");
  let address = format!("127.0.0.1:{port}");

  let result = timeout(TIMEOUT, async {
    let mut connection = Connection::<TcpStream>::builder()
      .connect(address, password)
      .await?;

    connection.cmd("players").await
  })
  .await;

  match result {
    Ok(Ok(response)) => {
      println!("{response}");
      0
    }
    Ok(Err(err)) => {
      eprintln!("Healthcheck failed: {err}");
      1
    }
    Err(_) => {
      eprintln!("Healthcheck timed out after {} seconds.", TIMEOUT.as_secs());
      1
    }
  }
}
