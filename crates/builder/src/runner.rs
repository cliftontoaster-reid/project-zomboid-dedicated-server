use std::process::{Command, exit};

pub fn run_command(cmd: &[String], dry_run: bool) {
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
