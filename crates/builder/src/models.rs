use serde::{Deserialize, Deserializer};

fn string_or_seq<'de, D: Deserializer<'de>>(
  deserializer: D,
) -> Result<Option<Vec<String>>, D::Error> {
  #[derive(Deserialize)]
  #[serde(untagged)]
  enum StringOrSeq {
    One(String),
    Many(Vec<String>),
  }

  Ok(Some(
    match Option::<StringOrSeq>::deserialize(deserializer)? {
      None => return Ok(None),
      Some(StringOrSeq::One(value)) => vec![value],
      Some(StringOrSeq::Many(values)) => values,
    },
  ))
}

#[derive(Debug, Deserialize)]
pub struct TargetEntry {
  pub name: String,
  #[serde(default)]
  pub aliases: Vec<String>,
  #[serde(default, deserialize_with = "string_or_seq")]
  pub alias: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct OsConfig {
  #[serde(default)]
  pub root: bool,
  pub default: String,
  pub name: String,
  pub target: Vec<TargetEntry>,
}

impl TargetEntry {
  pub fn resolve_aliases(&self) -> Vec<String> {
    let mut aliases = self.aliases.clone();
    if let Some(alias) = &self.alias {
      aliases.extend(alias.iter().cloned());
    }

    let mut seen = Vec::new();
    aliases
      .into_iter()
      .filter(|alias| {
        if seen.contains(alias) {
          false
        } else {
          seen.push(alias.clone());
          true
        }
      })
      .collect()
  }
}
