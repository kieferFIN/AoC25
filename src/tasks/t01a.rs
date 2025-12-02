use crate::tasks::{name_from_file, Task};
use std::cmp::PartialEq;
use std::ops;
use std::str::FromStr;

pub struct Task01a {}

impl Task01a {
  pub const fn new() -> Task01a {
    Task01a {}
  }
}

impl Task for Task01a {
  fn run(&self, data: &str) -> String {
    data.split('\n')
        .map(|line| line.parse().unwrap())
        .fold(Dial::new(), |d, r| d + r)
        .hits()
        .to_string()
  }

  fn test_data(&self) -> &str {
    "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
  }

  fn test_answer(&self) -> &str {
    "3"
  }

  fn name(&self) -> &str {
    name_from_file(file!())
  }
}

struct Dial {
  value: i32,
  hits: u16,
}
impl Dial {
  pub fn new() -> Dial {
    Dial { value: 50, hits: 0 }
  }

  pub fn hits(&self) -> u16 {
    self.hits
  }
}

impl ops::Add<Rot> for Dial {
  type Output = Dial;

  fn add(self, rot: Rot) -> Self::Output {
    let value = if rot.dir == Dir::R {
      self.value + rot.len
    } else {
      self.value - rot.len
    };
    let hits = if value % 100 == 0 {
      self.hits + 1
    } else {
      self.hits
    };
    #[cfg(debug_assertions)]
    println!("value {}, hits: {}", value, hits);
    Dial { value, hits }
  }
}
#[derive(PartialEq, Eq)]
enum Dir {
  L,
  R,
}

struct Rot {
  len: i32,
  dir: Dir,
}

impl FromStr for Rot {
  type Err = String;

  fn from_str(text: &str) -> Result<Self, Self::Err> {
    let (f, n) = text.split_at(1);
    let dir = match f {
      "L" => Dir::L,
      "R" => Dir::R,
      _ => return Err(format!("Unknown direction: {}", f)),
    };
    let len = n.trim().parse::<i32>().map_err(|e| format!("{}, value: {}", e.to_string(),n ))?;

    Ok(Rot { len, dir })
  }
}
