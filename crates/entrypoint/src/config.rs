use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Config {
  /// Server configuration and save folder name
  #[arg(
    short = 's',
    long = "server-name",
    env = "SERVER_NAME",
    default_value = "servertest"
  )]
  pub server_name: String,

  /// Initial admin account password
  #[arg(
    short = 'a',
    long = "admin-password",
    env = "ADMIN_PASSWORD",
    default_value = "ChangeMe123"
  )]
  pub admin_password: String,

  /// Player join password
  #[arg(
    short = 'w',
    long = "server-password",
    env = "SERVER_PASSWORD",
    default_value = ""
  )]
  pub server_password: String,

  /// Primary game UDP port
  #[arg(short = 'p', long = "port", env = "PORT", default_value_t = 16261)]
  pub port: u16,

  /// Steam connection UDP port
  #[arg(
    short = 't',
    long = "steam-port",
    env = "STEAM_PORT",
    default_value_t = 16262
  )]
  pub steam_port: u16,

  /// RCON TCP administration port
  #[arg(
    short = 'r',
    long = "rcon-port",
    env = "RCON_PORT",
    default_value_t = 27015
  )]
  pub rcon_port: u16,

  /// RCON authentication password
  #[arg(
    short = 'c',
    long = "rcon-password",
    env = "RCON_PASSWORD",
    default_value = "ChangeMe123"
  )]
  pub rcon_password: String,

  /// Memory allocation limit
  #[arg(short = 'm', long = "memory", env = "MEMORY", default_value = "4g")]
  pub memory: String,

  /// Maximum simultaneous player slots
  #[arg(
    short = 'x',
    long = "max-players",
    env = "MAX_PLAYERS",
    default_value_t = 16
  )]
  pub max_players: u32,

  /// Pause world time when zero players are online
  #[arg(
    short = 'e',
    long = "pause-empty",
    env = "PAUSE_EMPTY",
    default_value_t = true
  )]
  pub pause_empty: bool,

  /// Enable Steam VAC anti-cheat
  #[arg(
    short = 'v',
    long = "steam-vac",
    env = "STEAM_VAC",
    default_value_t = true
  )]
  pub steam_vac: bool,

  /// Enable UPnP port forwarding
  #[arg(short = 'u', long = "upnp", env = "UPNP", default_value_t = false)]
  pub upnp: bool,

  /// Perform a soft reset on server startup
  #[arg(
    short = 'R',
    long = "soft-reset",
    env = "SOFT_RESET",
    default_value_t = false
  )]
  pub soft_reset: bool,

  /// Path where all the server data is stored.
  #[arg(
    short = 'd',
    long = "data-path",
    env = "DATA_PATH",
    default_value_t = "/home/steam/ZomboidServer".to_string()
  )]
  pub data_path: String,

  /// Path where the server is stored.
  #[arg(
    short = 'g',
    long = "game-path",
    env = "GAME_PATH",
    default_value_t = "/opt/zomboid".to_string()
  )]
  pub game_path: String,

  /// Probe the running server over RCON and exit, used by the image healthcheck
  #[arg(long)]
  pub healthcheck: bool,

  /// UID to use when launching the server
  #[arg(long, env = "PUID", default_value_t = 1000)]
  pub puid: u32,

  /// GID to use when launching the server
  #[arg(long, env = "PGID", default_value_t = 1000)]
  pub pgid: u32,
}
