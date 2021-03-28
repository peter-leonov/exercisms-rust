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
}

impl Iterator for LCG {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    self.x = (self.a * self.x + self.c) % self.m;
    Some(self.x)
  }
}

static mut NAMES: LCG = LCG::new(123_456, 261, 77, 676_000);

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
    let n = unsafe { NAMES.next().unwrap() };
    self.name = number_to_name(n)
  }
}
