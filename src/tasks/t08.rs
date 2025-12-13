use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};
use std::str::FromStr;
use crate::tasks::Task;

const TEST_DATA: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

pub const T08A: Task = Task::new("t08a", TEST_DATA, "40", run_a);
pub const T08B: Task = Task::new("t08b", TEST_DATA, "25272", run_b);

type T = u64;

fn run_a(data: &str) -> String {
  let mut dist = if data.len() < 300 {
    BinaryHeap::init(10)
  } else {
    BinaryHeap::init(1000)
  };
  let data: Vec<Point> = data.lines().map(|row| row.parse().unwrap()).collect();
  for (i, p1) in data.iter().enumerate() {
    for (j, p2) in data[(i + 1)..].iter().enumerate() {
      dist.add(Dist(p1.dist2(p2), i, i + j + 1));
    }
  }
  let mut circuits = Circuits::new();
  for Dist(_, i, j) in dist {
    circuits.add(i, j);
  };
  let mut sizes = circuits.sizes();
  sizes.sort();
  sizes.reverse();
  #[cfg(debug_assertions)]
  println!("{:?}", sizes);
  (sizes[0] * sizes[1] * sizes[2]).to_string()
}

fn run_b(data: &str) -> String {
  let mut dist: BinaryHeap<Reverse<Dist>> = BinaryHeap::new();
  let data: Vec<Point> = data.lines().map(|row| row.parse().unwrap()).collect();
  for (i, p1) in data.iter().enumerate() {
    for (j, p2) in data[(i + 1)..].iter().enumerate() {
      dist.add(Dist(p1.dist2(p2), i, i + j + 1));
    }
  }
  let mut circuits = Circuits::new();
  while let Some(Reverse(Dist(_, i, j))) = dist.pop() {
    circuits.add(i, j);
    if circuits.is_ok(data.len()) {
      return (data[i].0 * data[j].0).to_string();
    }
  }
  "".to_string()
}

struct Point(T, T, T);

impl Point {
  fn dist2(&self, p: &Point) -> T {
    self.0.abs_diff(p.0).pow(2) + self.1.abs_diff(p.1).pow(2) + self.2.abs_diff(p.2).pow(2)
  }
}

impl FromStr for Point {
  type Err = String;
  fn from_str(data: &str) -> Result<Self, Self::Err> {
    let mut it = data.split(',').take(3).map(|s| s.parse().unwrap());
    Ok(Point(it.next().ok_or("Nan")?, it.next().ok_or("Nan")?, it.next().ok_or("Nan")?))
  }
}


struct Dist(T, usize, usize);

impl Eq for Dist {}

impl PartialEq<Self> for Dist {
  fn eq(&self, other: &Self) -> bool {
    self.0.eq(&other.0)
  }
}

impl PartialOrd<Self> for Dist {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.0.partial_cmp(&other.0)
  }
}

impl Ord for Dist {
  fn cmp(&self, other: &Self) -> Ordering {
    self.0.cmp(&other.0)
  }
}

trait MaxAdder {
  fn init(size: usize) -> Self;

  fn add(&mut self, dist: Dist);
}

impl MaxAdder for BinaryHeap<Dist> {
  fn init(size: usize) -> Self {
    let mut bh = BinaryHeap::with_capacity(size);
    for _ in 0..size {
      bh.push(Dist(T::MAX, 0, 0));
    };
    bh
  }

  fn add(&mut self, dist: Dist) {
    if self.peek().unwrap() > &dist {
      self.pop();
      self.push(dist);
    }
  }
}

struct Circuits {
  data: Vec<HashSet<usize>>,
}

impl Circuits {
  fn new() -> Circuits {
    Circuits { data: Vec::new() }
  }

  fn add(&mut self, i: usize, j: usize) {
    let ii = self.data.iter().position(|s| s.contains(&i));
    let jj = self.data.iter().position(|s| s.contains(&j));
    #[cfg(debug_assertions)]
    println!("{},{} -- {:?},{:?}", i, j, ii, jj);

    match (ii, jj) {
      (None, None) => {
        let mut h = HashSet::new();
        h.insert(i);
        h.insert(j);
        self.data.push(h);
      }
      (Some(k), None) => {
        self.data[k].insert(j);
      }
      (None, Some(k)) => {
        self.data[k].insert(i);
      }
      (Some(k), Some(l)) => {
        if k != l {
          let dd = self.data[l].clone();
          self.data[k].extend(dd);
          self.data.remove(l);
        }
      }
    }
    #[cfg(debug_assertions)]
    println!("  {:?}", self.data);
  }

  fn sizes(&self) -> Vec<usize> {
    self.data.iter().map(|d| d.len()).collect()
  }

  fn is_ok(&self, size: usize) -> bool {
    self.data.len() == 1 && self.data[0].len() == size
  }
}

trait MinAdder {
  fn add(&mut self, dist: Dist);
}

impl MinAdder for BinaryHeap<Reverse<Dist>> {
  fn add(&mut self, dist: Dist) {
    self.push(Reverse(dist));
  }
}