// https://stackoverflow.com/a/196164/2224875
// https://en.wikipedia.org/wiki/Linear_congruential_generator
// X(n+1) = (aX(n) + c) mod m
pub struct LCG {
  x: u32,
  a: u32,
  c: u32,
  m: u32,
}

impl LCG {
  /// `m` and `c` are relatively prime,
  /// `a − 1` is divisible by all prime factors of `m`,
  /// `a − 1` is divisible by `4` if m is divisible by `4`.
  pub const fn new(x: u32, a: u32, c: u32, m: u32) -> Self {
    Self { x, a, c, m }
  }

  pub fn next(&mut self) -> u32 {
    self.x = (self.a * self.x + self.c) % self.m;
    self.x
  }
}

// A mutex is too heavy here as the whole state fits into an atomic,
// but involving all the lock-free mental pressure for a robot name…
// I'd go with unsafe here.
fn get_next_id() -> u32 {
  const M: u32 = 26 * 26 * 1000;
  const PM: u32 = 2 * 13 * 5;
  const C: u32 = 7 * 11;
  const A1: u32 = PM * 2;
  const A: u32 = A1 + 1;
  const X: u32 = M / 3;

  static mut NAMES: LCG = LCG::new(X, A, C, M);
  // static mut NAMES: LCG = LCG::new(123_456, 261, 77, 676_000);

  // The app is single threaded,
  // otherwise we'd rather have a thread safe robot factory
  // with atomics or mutexes.
  unsafe { NAMES.next() }
}

#[cfg(test)]
mod tests {
  use std::collections::HashSet;

  #[test]
  fn test_the_prng_constants() {
    const M: u32 = 26 * 26 * 1000;

    let mut seen = HashSet::with_capacity(M as usize);

    for _ in 0..M {
      seen.insert(super::get_next_id());
    }

    assert_eq!(seen.len(), M as usize);
  }
}

fn number_to_name(n: u32) -> String {
  let numbers = n % 1000;

  let letters = n / 1000;
  let letter1 = letters / 26;
  let letter2 = letters % 26;

  let char1 = std::char::from_u32('A' as u32 + letter1).unwrap();
  let char2 = std::char::from_u32('A' as u32 + letter2).unwrap();

  format!("{}{}{:03}", char1, char2, numbers)
}

pub struct Robot {
  name: String,
}

impl Robot {
  pub fn new() -> Self {
    let mut robot = Self { name: "".into() };
    robot.reset_name();
    robot
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn reset_name(&mut self) {
    let n = get_next_id();
    self.name = number_to_name(n)
  }
}
