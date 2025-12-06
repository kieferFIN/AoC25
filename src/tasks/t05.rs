use std::cmp::{max, min};
use std::str::FromStr;
use crate::tasks::Task;

const TEST_DATA: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

pub const T05A: Task = Task::new("t05a", TEST_DATA, "3", run_a);
pub const T05B: Task = Task::new("t05b", TEST_DATA, "14", run_b);

type IdxType = usize;

fn run_a(data: &str) -> String {
  let mut ranges = Vec::new();
  let mut fresh = None;
  for l in data.lines() {
    if l.is_empty() {
      fresh = Some(0);
      #[cfg(debug_assertions)]
      println!("{:?}", ranges);
      continue;
    }
    if fresh.is_none() {
      let r: Range = l.parse().unwrap();
      ranges.push(r);
      continue;
    }
    let n: IdxType = l.parse().unwrap();
    #[cfg(debug_assertions)]
    println!("{}", n);
    if ranges.iter().any(|r| r.includes(n)) {
      fresh = fresh.map(|i| i + 1);
      #[cfg(debug_assertions)]
      println!("\t{:?}", fresh);
    }
  }
  fresh.unwrap().to_string()
}

fn run_b(data: &str) -> String {
  let mut ranges = Vec::new();
  for l in data.lines() {
    if l.is_empty() {
      break;
    }
    let r: Range = l.parse().unwrap();
    ranges.push(r);
  };
  #[cfg(debug_assertions)]
  println!("{:?}", ranges.len());
  let mut set = RangeSet::new();
  ranges.sort_by(|a, b| a.start.cmp(&b.start));
  for r in ranges {
    set.add(r);
  };
  set.size().to_string()
}


#[derive(Debug)]
struct Range {
  start: IdxType,
  end: IdxType,
}

impl Range {
  fn new(start: IdxType, end: IdxType) -> Self {
    Range { start, end }
  }
  fn includes(&self, i: IdxType) -> bool {
    self.start <= i && i <= self.end
  }

  fn overlaps(&self, other: &Range) -> bool {
    self.includes(other.start) || self.includes(other.end) || other.includes(self.start) || other.includes(self.end)
  }

  fn compine(&mut self, other: Range) {
    self.start = min(self.start, other.start);
    self.end = max(self.end, other.end);
  }

  fn size(&self) -> IdxType {
    self.end - self.start + 1
  }
}

impl FromStr for Range {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut it = s.split('-').take(2).map(|s| s.parse::<IdxType>().unwrap());
    let start = it.next().unwrap();
    let end = it.next().unwrap();
    Ok(Range::new(start, end))
  }
}

struct RangeSet {
  ranges: Vec<Range>,
}

impl RangeSet {
  fn new() -> Self {
    RangeSet { ranges: Vec::new() }
  }
  fn add(&mut self, range: Range) {
    let i = self.ranges.iter().position(|r| r.overlaps(&range));
    if let Some(i) = i {
      self.ranges[i].compine(range);
    } else {
      self.ranges.push(range);
    }
  }

  fn size(&self) -> IdxType {
    self.ranges.iter().map(|r| r.size()).sum()
  }
}

