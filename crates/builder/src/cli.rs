use clap::Parser;

const DEFAULT_REGISTRY: &str = "cliftontoasterreid/project-zomboid-dedicated-server";

#[derive(Debug, Parser)]
#[command(
  name = "zomboid-builder",
  bin_name = "builder",
  version,
  about = "Builds multi-OS Project Zomboid dedicated server Docker images"
)]
pub struct Cli {
  /// Target OS family (e.g. debian)
  #[arg(short, long, value_name = "os")]
  pub os: Option<String>,

  /// Target OS version (e.g. 13)
  #[arg(short = 'v', long, value_name = "version")]
  pub os_version: Option<String>,

  /// Game major version
  #[arg(short = 'm', long, value_name = "major", default_value = "42")]
  pub game_major: String,

  /// Game patch version
  #[arg(short = 'p', long, value_name = "patch", default_value = "21")]
  pub game_patch: String,

  /// Build unstable branch
  #[arg(short = 'u', long, num_args = 0..=1, default_value_t = false, default_missing_value = "true")]
  pub unstable: bool,

  /// Build target variant (root or rootless)
  #[arg(long, value_name = "variant", default_value = "root", value_parser = ["root", "rootless"])]
  pub variant: String,

  /// Tag as primary top-level release (latest/unstable)
  #[arg(long, num_args = 0..=1, default_value_t = true, default_missing_value = "true")]
  pub primary: bool,

  /// Container registry image name
  #[arg(short, long, value_name = "registry", default_value = DEFAULT_REGISTRY)]
  pub registry: String,

  /// Prebuilt downloader image providing /opt/zomboid (skips the Steam download)
  #[arg(long = "downloader-image", value_name = "image", default_value = "")]
  pub downloader_image: String,

  /// Path to TOML configuration directory
  #[arg(
    short = 'c',
    long = "config-dir",
    value_name = "dir",
    default_value = "config/os"
  )]
  pub config_dir: String,

  /// Push image tags directly via buildx
  #[arg(long, num_args = 0..=1, default_value_t = false, default_missing_value = "true")]
  pub push: bool,

  /// Print tags and commands without building
  #[arg(short = 'd', long = "dry-run", num_args = 0..=1, default_value_t = false, default_missing_value = "true")]
  pub dry_run: bool,
}
