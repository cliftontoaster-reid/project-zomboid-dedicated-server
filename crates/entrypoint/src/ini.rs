// serini quotes values the way a strict INI parser expects (`\;`, `\"`, `key =
// value`), but Project Zomboid reads the file as plain `key=value` lines and
// never unescapes, so the quoting is stripped before the file is written.
pub fn to_pz_ini(content: &str) -> String {
  let mut out = String::with_capacity(content.len());

  for line in content.lines() {
    let Some((key, value)) = line.split_once('=') else {
      continue;
    };

    out.push_str(key.trim());
    out.push('=');
    out.push_str(
      value
        .trim()
        .replace("\\;", ";")
        .replace("\\#", "#")
        .replace("\\\"", "\"")
        .replace("\\\\", "\\")
        .as_str(),
    );
    out.push('\n');
  }

  out
}
