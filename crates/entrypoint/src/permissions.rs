use std::{
  ffi::CString,
  io,
  os::unix::fs::MetadataExt,
  path::Path,
};

use walkdir::WalkDir;

/// Adjusts ownership of `path` recursively to `puid:pgid` if running as root
/// and the directory ownership does not already match.
pub fn fix_permissions(path: &Path, puid: u32, pgid: u32) -> io::Result<()> {
  // If not running as root, we cannot chown anyway
  if unsafe { libc::geteuid() } != 0 {
    return Ok(());
  }

  // Check if the root of the path already matches puid:pgid
  if let Ok(meta) = path.metadata() {
    if meta.uid() == puid && meta.gid() == pgid {
      return Ok(());
    }
  }

  println!("Adjusting permissions for {:?} to {}:{}...", path, puid, pgid);

  for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
    let c_path = CString::new(entry.path().to_str().unwrap_or_default())
      .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    let res = unsafe { libc::chown(c_path.as_ptr(), puid, pgid) };
    if res != 0 {
      let err = io::Error::last_os_error();
      eprintln!("Warning: failed to chown {:?}: {err}", entry.path());
    }
  }

  Ok(())
}
