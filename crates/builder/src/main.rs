pub mod cli;
pub mod config;
pub mod models;
pub mod runner;
pub mod tags;

use std::process::exit;

use clap::Parser;
use cli::Cli;
use config::{discover_os_configs, load_os_config};
use models::TargetEntry;
use runner::run_command;
use tags::generate_tags;

fn main() {
  let opts = Cli::parse();

  let os_list = match &opts.os {
    Some(os) => vec![os.clone()],
    None => match discover_os_configs(&opts.config_dir) {
      Ok(names) => names,
      Err(err) => {
        eprintln!(
          "Failed to read config directory '{}': {err}",
          opts.config_dir
        );
        exit(1);
      }
    },
  };

  if os_list.is_empty() {
    eprintln!(
      "No OS configuration TOML files found in '{}'.",
      opts.config_dir
    );
    exit(1);
  }

  for os_name in &os_list {
    let os_config = match load_os_config(&opts.config_dir, os_name) {
      Ok(config) => config,
      Err(err) => {
        eprintln!("{err}");
        exit(1);
      }
    };

    let targets_to_build: Vec<&TargetEntry> = match &opts.os_version {
      Some(os_version) => os_config
        .target
        .iter()
        .filter(|t| &t.name == os_version)
        .collect(),
      None => os_config.target.iter().collect(),
    };

    if targets_to_build.is_empty() {
      match &opts.os_version {
        Some(os_version) => {
          eprintln!("OS version '{os_version}' not found in {os_name}.toml");
        }
        None => eprintln!("No targets defined in {os_name}.toml"),
      }
      exit(1);
    }

    for target in targets_to_build {
      let tags = generate_tags(&os_config, target, &opts);

      println!("\n==================================================");
      println!(" Target OS      : {}:{}", os_config.name, target.name);
      println!(" Variant        : {}", opts.variant);
      println!(
        " Game Version   : {}.{}{}",
        opts.game_major,
        opts.game_patch,
        if opts.unstable { " (unstable)" } else { "" }
      );
      println!(" Generated Tags :");
      for tag in &tags {
        println!("   - {tag}");
      }
      println!("==================================================");

      let base_dockerfile = format!("docker/{}/{}/Dockerfile", os_config.name, target.name);
      let base_image_tag = format!("zomboid-base:{}-{}", os_config.name, target.name);

      let base_build_cmd: Vec<String> = vec![
        "docker".into(),
        "build".into(),
        "-t".into(),
        base_image_tag.clone(),
        "-f".into(),
        base_dockerfile,
        ".".into(),
      ];
      run_command(&base_build_cmd, opts.dry_run);

      let mut app_build_cmd: Vec<String> = vec![
        "docker".into(),
        "buildx".into(),
        "build".into(),
        "--target".into(),
        opts.variant.clone(),
        "--build-arg".into(),
        format!("BASE_IMAGE={base_image_tag}"),
        "--build-arg".into(),
        format!("GAME_MAJOR={}", opts.game_major),
        "--build-arg".into(),
        format!("GAME_PATCH={}", opts.game_patch),
        "--build-arg".into(),
        format!("IS_UNSTABLE={}", opts.unstable),
        "-f".into(),
        "docker/app/Dockerfile".into(),
      ];

      if !opts.downloader_image.is_empty() {
        app_build_cmd.push("--build-arg".into());
        app_build_cmd.push(format!("DOWNLOADER_IMAGE={}", opts.downloader_image));
      }

      for tag in &tags {
        app_build_cmd.push("-t".into());
        app_build_cmd.push(tag.clone());
      }

      if opts.push {
        app_build_cmd.push("--push".into());
      }

      app_build_cmd.push(".".into());
      run_command(&app_build_cmd, opts.dry_run);
    }
  }
}
