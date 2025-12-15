use std::cmp::{Ordering};
use std::ops::Sub;
use std::str::FromStr;
use crate::tasks::Task;
#[cfg(debug_assertions)]
use plotters::backend::{BitMapBackend, DrawingBackend};
#[cfg(debug_assertions)]
use plotters::style::{BLUE, GREEN, RED, YELLOW};

const TEST_DATA: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

pub const T09A: Task = Task::new("t09a", TEST_DATA, "50", run_a);
pub const T09B: Task = Task::new("t09b", TEST_DATA, "24", run_b);

fn run_a(data: &str) -> String {
  let data: Vec<Point> = data.lines().map(|l| l.parse().unwrap()).collect();
  let mut max_area = 0;
  for (i, p1) in data.iter().enumerate() {
    for p2 in data[(i + 1)..].iter() {
      let a = p1.area(p2);
      max_area = max_area.max(a);
    }
  }
  max_area.to_string()
}

fn run_b(data: &str) -> String {
  let data: Vec<Point> = data.lines().map(|l| l.parse().unwrap()).collect();
  println!("***");
  let mut floor = Floor::new();

  let mut areas = Vec::with_capacity(122760);
  for (i, p1) in data.iter().enumerate() {
    let p = if i + 1 < data.len() {
      &data[i + 1]
    } else {
      &data[0]
    };
    floor.update(p1, p);
    for p2 in data[(i + 1)..].iter() {
      areas.push(Area::new(p1, p2))
    }
  }
  println!("{}", areas.len());
  println!("*****");
  #[cfg(debug_assertions)]
  let mut draw_area = draw_lines(&floor.data);

  let mut max_area = 0;
  for a in &areas {
    if a.size() > max_area && floor.is_correct(a) {
      max_area = a.size()
    }
  }
  #[cfg(debug_assertions)]
  areas.iter().filter(|a| a.size() == 24).for_each(|a| {
    for l in &a.smaller().unwrap().edges() {
      let c = if floor.data.iter().any(|l1| l1.intersects(l)) {
        RED
      } else {
        YELLOW
      };
      println!("{:?}, {:?}", l, &c);
      let t = l.tuples(10);
      draw_area.draw_line(t.0, t.1, &c).unwrap();
    }
  });
  #[cfg(debug_assertions)]
  draw_area.present().unwrap();

  max_area.to_string()
}

type T = u64;

#[derive(Debug, Ord, Eq, PartialOrd, PartialEq)]
struct Point(T, T);

impl Point {
  fn area(&self, p: &Point) -> T {
    (self.0.abs_diff(p.0) + 1) * (self.1.abs_diff(p.1) + 1)
  }
}

impl Sub for &Point {
  type Output = Point;
  fn sub(self, rhs: &Point) -> Point {
    Point(self.0 - rhs.0, self.1 - rhs.1)
  }
}

impl FromStr for Point {
  type Err = String;
  fn from_str(data: &str) -> Result<Self, Self::Err> {
    let mut it = data.split(',').take(2).map(|s| s.parse().unwrap());
    Ok(Point(it.next().ok_or("Nan")?, it.next().ok_or("Nan")?))
  }
}

#[derive(Debug)]
struct Area {
  start: Point,
  size: Point,
}

impl Area {
  fn new(p1: &Point, p2: &Point) -> Area {
    Area { start: Point(p1.0.min(p2.0), p1.1.min(p2.1)), size: Point(p1.0.abs_diff(p2.0) + 1, p1.1.abs_diff(p2.1) + 1) }
  }

  fn size(&self) -> T {
    self.size.0 * self.size.1
  }

  fn smaller(&self) -> Option<Area> {
    if self.size.0 < 2 || self.size.1 < 2 {
      None
    } else {
      Some(Area { start: Point(self.start.0 + 1, self.start.1 + 1), size: Point(self.size.0 - 2, self.size.1 - 2) })
    }
  }

  fn edges(&self) -> [Line; 4] {
    let corners = self.corners();
    let edges = [
      Line::new(&corners[0], &corners[1]),
      Line::new(&corners[1], &corners[2]),
      Line::new(&corners[2], &corners[3]),
      Line::new(&corners[3], &corners[0])
    ];
    edges
  }

  fn corners(&self) -> [Point; 4] {
    let c = [Point(self.start.0, self.start.1),
      Point(self.start.0, self.start.1 + self.size.1 - 1),
      Point(self.start.0 + self.size.0 - 1, self.start.1 + self.size.1 - 1),
      Point(self.start.0 + self.size.0 - 1, self.start.1),
    ];
    c
  }
}

#[derive(Debug)]
struct Line {
  start: Point,
  length: T,
  is_vertical: bool,
}

impl Line {
  fn new(p1: &Point, p2: &Point) -> Line {
    let is_vertical = if p1.0 == p2.0 {
      true
    } else if p1.1 == p2.1 {
      false
    } else {
      panic!("not line: {:?}, {:?}", p1, p2);
    };
    let length = if is_vertical {
      p1.1.abs_diff(p2.1)
    } else {
      p1.0.abs_diff(p2.0)
    };
    Line { start: Point(p1.0.min(p2.0), p1.1.min(p2.1)), length, is_vertical }
  }

  fn intersects(&self, other: &Line) -> bool {
    if self.length == 0 || other.length == 0 {
      false
    } else if self.is_vertical == other.is_vertical {
      false
    } else {
      let (v, h) = if self.is_vertical {
        (self, other)
      } else {
        (other, self)
      };
      v.start.1 <= h.start.1 && v.start.1 + v.length >= h.start.1 && h.start.0 <= v.start.0 && h.start.0 + h.length >= v.start.0
    }
  }
  #[cfg(debug_assertions)]
  fn tuples(&self, f: i32) -> ((i32, i32), (i32, i32)) {
    let s = (self.start.0 as i32 * f, self.start.1 as i32 * f);
    let e = if self.is_vertical {
      (self.start.0 as i32 * f, (self.start.1 + self.length) as i32 * f)
    } else {
      ((self.start.0 + self.length) as i32 * f, self.start.1 as i32 * f)
    };
    (s, e)
  }
}

impl Eq for Area {}

impl PartialEq<Self> for Area {
  fn eq(&self, other: &Self) -> bool {
    self.size().eq(&other.size())
  }
}

impl PartialOrd<Self> for Area {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    self.size().partial_cmp(&other.size())
  }
}

impl Ord for Area {
  fn cmp(&self, other: &Self) -> Ordering {
    self.size().cmp(&other.size())
  }
}

struct Floor {
  data: Vec<Line>,
}

impl Floor {
  fn new() -> Self { Floor { data: Vec::new() } }

  fn update(&mut self, p1: &Point, p2: &Point) {
    self.data.push(Line::new(p1, p2));
  }
  fn is_correct(&self, area: &Area) -> bool {
    if let Some(smaller) = area.smaller() {
      let edges = smaller.edges();
      !self.data.iter().any(|l| edges.iter().any(|l1| l.intersects(l1)))
    } else {
      false
    }
  }
}

#[cfg(debug_assertions)]
fn draw_lines(lines: &'_ [Line]) -> BitMapBackend<'_> {
  let mut root = BitMapBackend::new("lines.png", (150, 150));
  let verticals: Vec<_> = lines.iter().filter(|l| l.is_vertical).map(|l| l.tuples(10)).collect();
  let horizontals: Vec<_> = lines.iter().filter(|l| !l.is_vertical).map(|l| l.tuples(10)).collect();
  for line in verticals {
    root.draw_line(line.0, line.1, &BLUE).unwrap();
  }
  for line in horizontals {
    root.draw_line(line.0, line.1, &GREEN).unwrap();
  }
  root
}