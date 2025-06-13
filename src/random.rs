use rand::distr::Alphanumeric;
use rand::{rng, Rng};

pub(crate) fn random_string(len: usize) -> String {
  rng()
    .sample_iter(&Alphanumeric)
    .take(len)
    .map(char::from)
    .collect()
}
