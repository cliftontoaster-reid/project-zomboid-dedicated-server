use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Config {
  /// Server configuration and save folder name
  #[arg(short = 's', long = "server-name", env = "SERVER_NAME")]
  pub server_name: Option<String>,

  /// Initial admin account password
  #[arg(short = 'a', long = "admin-password", env = "ADMIN_PASSWORD")]
  pub admin_password: Option<String>,

  /// Player join password
  #[arg(short = 'w', long = "server-password", env = "SERVER_PASSWORD")]
  pub server_password: Option<String>,

  /// Primary game UDP port
  #[arg(short = 'p', long = "port", env = "PORT")]
  pub port: Option<u16>,

  /// Steam connection UDP port
  #[arg(short = 't', long = "steam-port", env = "STEAM_PORT")]
  pub steam_port: Option<u16>,

  /// RCON TCP administration port
  #[arg(short = 'r', long = "rcon-port", env = "RCON_PORT")]
  pub rcon_port: Option<u16>,

  /// RCON authentication password
  #[arg(short = 'c', long = "rcon-password", env = "RCON_PASSWORD")]
  pub rcon_password: Option<String>,

  /// Memory allocation limit
  #[arg(short = 'm', long = "memory", env = "MEMORY")]
  pub memory: Option<String>,

  /// Maximum simultaneous player slots
  #[arg(short = 'x', long = "max-players", env = "MAX_PLAYERS")]
  pub max_players: Option<u32>,

  /// Pause world time when zero players are online
  #[arg(short = 'e', long = "pause-empty", env = "PAUSE_EMPTY")]
  pub pause_empty: Option<bool>,

  /// Enable Steam VAC anti-cheat
  #[arg(short = 'v', long = "steam-vac", env = "STEAM_VAC")]
  pub steam_vac: Option<bool>,

  /// Enable UPnP port forwarding
  #[arg(short = 'u', long = "upnp", env = "UPNP")]
  pub upnp: Option<bool>,

  /// Perform a soft reset on server startup
  #[arg(short = 'R', long = "soft-reset", env = "SOFT_RESET")]
  pub soft_reset: Option<bool>,

  /// Path where all the server data is stored.
  #[arg(short = 'd', long = "data-path", env = "DATA_PATH")]
  pub data_path: Option<String>,

  /// Path where the server is stored.
  #[arg(short = 'g', long = "game-path", env = "GAME_PATH")]
  pub game_path: Option<String>,

  /// Probe the running server over RCON and exit, used by the image healthcheck
  #[arg(long)]
  pub healthcheck: bool,

  /// UID to use when launching the server
  #[arg(long, env = "PUID")]
  pub puid: Option<u32>,

  /// GID to use when launching the server
  #[arg(long, env = "PGID")]
  pub pgid: Option<u32>,
}

impl Config {
  pub fn server_name(&self) -> &str {
    self.server_name.as_deref().unwrap_or("servertest")
  }
  pub fn admin_password(&self) -> Option<&str> {
    self.admin_password.as_deref()
  }
  pub fn memory(&self) -> &str {
    self.memory.as_deref().unwrap_or("4g")
  }
  pub fn soft_reset(&self) -> bool {
    self.soft_reset.unwrap_or(false)
  }
  pub fn data_path(&self) -> &str {
    self
      .data_path
      .as_deref()
      .unwrap_or("/home/steam/ZomboidServer")
  }
  pub fn game_path(&self) -> &str {
    self.game_path.as_deref().unwrap_or("/opt/zomboid")
  }
  pub fn puid(&self) -> u32 {
    self.puid.unwrap_or(1000)
  }
  pub fn pgid(&self) -> u32 {
    self.pgid.unwrap_or(1000)
  }
}
