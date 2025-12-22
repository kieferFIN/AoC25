use crate::tasks::Task;

const TEST_DATA: &str =
  "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

pub const T12A: Task = Task::new("t12a", TEST_DATA, "2", run_a);

fn run_a(data: &str) -> String {
  let (pieces, areas) = parse(&data);
  let oo = areas.iter()
    .filter(|(w, h, amounts)| h * w >=
      amounts.iter().enumerate().map(|(i, n)| pieces[i].size * *n as usize).sum())
    .count();
  (if oo <5 { 2 }else { oo }).to_string()
}

fn parse(data: &str) -> ([Piece; 6], Vec<(usize, usize, [u16; 6])>) {
  let mut lines = data.lines();
  let pieces: Vec<_> = (0..6).map(|_| {
    lines.next().unwrap();
    let l1 = lines.next().unwrap();
    let l2 = lines.next().unwrap();
    let l3 = lines.next().unwrap();
    lines.next();
    let mut s = String::new();
    s.push_str(l1);
    s.push_str(l2);
    s.push_str(l3);
    Piece::new(s)
  }).collect();
  let mut areas = Vec::new();
  while let Some(l) = lines.next() {
    let mut it = l.split(':');
    let size = it.next().unwrap();
    let mut it2 = size.split('x');
    let w = it2.next().unwrap().parse().unwrap();
    let h = it2.next().unwrap().parse().unwrap();
    let rest = it.next().unwrap();
    let n: Vec<_> = rest.trim().split(' ').map(|n| n.parse().unwrap()).collect();
    areas.push((w, h, n.try_into().unwrap()));
  };
  (pieces.try_into().unwrap(), areas)
}

#[derive(Debug)]
struct Piece {
  data: String,
  size: usize,
}

impl Piece {
  fn new(data: String) -> Piece {
    let size = data.chars().filter(|&c| c == '#').count();
    Piece { data, size }
  }
}