pub mod cli;
pub mod config;
pub mod models;

use clap::Parser;
use std::process::{Command, exit};

use cli::Cli;
use config::{discover_os_configs, load_os_config};
use models::{OsConfig, TargetEntry};

fn push_unique(tags: &mut Vec<String>, tag: String) {
  if !tags.contains(&tag) {
    tags.push(tag);
  }
}

fn run_command(cmd: &[String], dry_run: bool) {
  println!("\n$ {}", cmd.join(" "));
  if dry_run {
    return;
  }

  let status = Command::new(&cmd[0]).args(&cmd[1..]).status();

  match status {
    Ok(status) if status.success() => {}
    Ok(status) => {
      let code = status.code().unwrap_or(1);
      eprintln!("Command failed with exit code {code}");
      exit(code);
    }
    Err(err) => {
      eprintln!("Command failed with exit code 1");
      eprintln!("{err}");
      exit(1);
    }
  }
}

fn generate_tags(os_config: &OsConfig, target: &TargetEntry, opts: &Cli) -> Vec<String> {
  let branch_suffix = if opts.unstable { "-unstable" } else { "" };
  let variant_suffix = if opts.variant == "rootless" {
    "-rootless"
  } else {
    ""
  };
  let primary_branch_tag = if opts.unstable { "unstable" } else { "latest" };

  let is_default_os_version = os_config.default == target.name;
  let is_root_os = os_config.root;

  let mut version_keys = vec![target.name.clone()];
  version_keys.extend(target.resolve_aliases());

  let mut tags: Vec<String> = Vec::new();

  for v_key in &version_keys {
    push_unique(
      &mut tags,
      format!(
        "{}:{}.{}{}-{}-{}{}",
        opts.registry,
        opts.game_major,
        opts.game_patch,
        branch_suffix,
        os_config.name,
        v_key,
        variant_suffix
      ),
    );
    push_unique(
      &mut tags,
      format!(
        "{}:{}{}-{}-{}{}",
        opts.registry, opts.game_major, branch_suffix, os_config.name, v_key, variant_suffix
      ),
    );
  }

  if is_default_os_version {
    push_unique(
      &mut tags,
      format!(
        "{}:{}.{}{}-{}{}",
        opts.registry,
        opts.game_major,
        opts.game_patch,
        branch_suffix,
        os_config.name,
        variant_suffix
      ),
    );
    push_unique(
      &mut tags,
      format!(
        "{}:{}{}-{}{}",
        opts.registry, opts.game_major, branch_suffix, os_config.name, variant_suffix
      ),
    );
  }

  if is_root_os && is_default_os_version {
    push_unique(
      &mut tags,
      format!(
        "{}:{}.{}{}{}",
        opts.registry, opts.game_major, opts.game_patch, branch_suffix, variant_suffix
      ),
    );
    push_unique(
      &mut tags,
      format!(
        "{}:{}{}{}",
        opts.registry, opts.game_major, branch_suffix, variant_suffix
      ),
    );
  }

  if is_root_os && is_default_os_version && opts.primary {
    push_unique(
      &mut tags,
      format!("{}:{}{}", opts.registry, primary_branch_tag, variant_suffix),
    );
  }

  tags
}

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
