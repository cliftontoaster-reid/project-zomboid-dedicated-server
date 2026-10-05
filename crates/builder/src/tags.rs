use crate::{
  cli::Cli,
  models::{OsConfig, TargetEntry},
};

pub fn push_unique(tags: &mut Vec<String>, tag: String) {
  if !tags.contains(&tag) {
    tags.push(tag);
  }
}

pub fn generate_tags(os_config: &OsConfig, target: &TargetEntry, opts: &Cli) -> Vec<String> {
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
