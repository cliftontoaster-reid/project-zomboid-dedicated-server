use rand::{RngExt, rng};

pub static ALPHA: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
pub static PASSWORD: &[u8] =
  b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_.-!@$%&*+?";

pub fn generate_random_string(length: usize, charset: &[u8]) -> String {
  let mut rng = rng();

  (0..length)
    .map(|_| {
      let idx = rng.random_range(0..charset.len());
      charset[idx] as char
    })
    .collect()
}
