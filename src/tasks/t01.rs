use crate::tasks::{Task};
use std::cmp::PartialEq;
use std::ops::{Sub, Add};
use std::str::FromStr;

const TEST_DATA: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

pub const T01A: Task = Task::new("t01a", TEST_DATA, "3", run_a);
pub const T01B: Task = Task::new("t01b", TEST_DATA, "6", run_b);

fn run_a(data: &str) -> String {
  data.split('\n')
      .map(|line| line.parse().unwrap())
      .fold(Dial::new(), |d, r| d.smooth(r))
      .hits()
      .to_string()
}

fn run_b(data: &str) -> String {
  data.split('\n')
      .map(|line| line.parse().unwrap())
      .fold(Dial::new(), |d, r| d.steps(r))
      .hits()
      .to_string()
}

struct Dial {
  value: i32,
  hits: u16,
}

impl Dial {
  fn new() -> Dial {
    Dial { value: 50, hits: 0 }
  }

  fn hits(&self) -> u16 {
    self.hits
  }

  fn smooth(self, rot: Rot) -> Dial {
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

  fn steps(self, rot: Rot) -> Dial {
    let op: fn(_: i32, _: i32) -> _ = if rot.dir == Dir::R {
      i32::add
    } else {
      i32::sub
    };
    let mut hits = self.hits;
    let mut value = self.value;
    for _ in 0..rot.len {
      value = op(value, 1);
      if value % 100 == 0 {
        hits += 1;
      }
    }
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
    let len = n
      .trim()
      .parse::<i32>()
      .map_err(|e| format!("{}, value: {}", e.to_string(), n))?;

    Ok(Rot { len, dir })
  }
}
