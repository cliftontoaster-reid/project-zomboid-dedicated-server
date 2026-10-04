use std::time::Duration;

use rcon::Connection;
use tokio::{net::TcpStream, time::timeout};

use crate::config::Config;

const TIMEOUT: Duration = Duration::from_secs(5);

pub async fn run(config: &Config) -> i32 {
  let address = format!("127.0.0.1:{}", config.rcon_port);

  let result = timeout(TIMEOUT, async {
    let mut connection = Connection::<TcpStream>::builder()
      .connect(address, &config.rcon_password)
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
