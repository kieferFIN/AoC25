use std::collections::{HashMap, VecDeque};
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};
use itertools::Itertools;
use crate::tasks::Task;

const TEST_DATA: &str =
"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

pub const T10A: Task = Task::new("t10a", TEST_DATA, "7", run_a);
pub const T10B: Task = Task::new("t10b", TEST_DATA, "33", run_b);

fn run_a(data: &str) -> String {
  let data: Vec<_> = data.lines().map(|l| parse(l)).collect();
  let results: Vec<_> = data.iter().map(|(s, m, _, _)| solver_a(*s, m)).collect();
  #[cfg(debug_assertions)]
  println!("{:?}", results);
  results.iter().sum::<u16>().to_string()
}

fn run_b(data: &str) -> String {
  let data: Vec<_> = data.lines().map(|l| parse(l)).collect();
  let results: Vec<_> = data.iter().map(|(_, _, m, e)| solver_b(e, m)).collect();

  #[cfg(debug_assertions)]
  println!("{:?}", results.iter().enumerate().collect::<Vec<_>>());
  results.iter().sum::<u32>().to_string()
}

fn parse(line: &str) -> (u16, Vec<u16>, Vec<Vec<u16>>, Vec<u16>) {
  let line = line.strip_prefix("[").unwrap();
  let (start, rest) = line.split_once("]").unwrap();
  let s = start.chars().enumerate().fold(0u16, |s, (i, c)| if c == '#' { s + 2u16.pow(i as u32) } else { s });
  let (buttons_str, jolts_str) = rest.split_once("{").unwrap();
  let buttons: Vec<Vec<_>> = buttons_str.trim().split(" ")
                                        .map(|c| c.trim_start_matches('(').trim_end_matches(')').split(',').map(|n| n.parse().unwrap()).collect()).collect();
  let button_numbers: Vec<_> = buttons.iter().map(|b| b.iter().fold(0u16, |s, i| s + 2u16.pow(*i as u32))).collect();
  let jolts: Vec<_> = jolts_str.trim_end_matches('}').split(',').map(|c| c.parse().unwrap()).collect();
  (s, button_numbers, buttons, jolts)
}

fn solver_a(end: u16, moves: &[u16]) -> u16 {
  let mut visited = HashMap::new();
  let mut queue = VecDeque::new();
  queue.push_back((0, 0));
  while let Some((steps, node)) = queue.pop_front() {
    visited.insert(node, steps);
    for m in moves {
      let new_node = node ^ m;
      #[cfg(debug_assertions)]
      println!("{:b} {:b} {:b}: {}", node, m, new_node, steps + 1);
      if new_node == end {
        return steps + 1;
      }
      if !visited.contains_key(&new_node) {
        queue.push_back((steps + 1, new_node));
      }
    }
  }
  panic!("No solution found");
}

fn solver_b(end: &[u16], buttons: &[Vec<u16>]) -> u32 {
  match (buttons.len(), end.len()) {
    (4, 6) => solve::<4, 6>(end, buttons),
    (5, 5) => solve::<5, 5>(end, buttons),
    (6, 4) => solve::<6, 4>(end, buttons),
    (10, 10) => solve::<10, 10>(end, buttons),
    (2, 4) => solve::<2, 4>(end, buttons),
    (8, 8) => solve::<8, 8>(end, buttons),
    (5, 4) => solve::<5, 4>(end, buttons),
    (8, 6) => solve::<8, 6>(end, buttons),
    (12, 10) => solve::<12, 10>(end, buttons),
    (6, 7) => solve::<6, 7>(end, buttons),
    (8, 9) => solve::<8, 9>(end, buttons),
    (13, 10) => solve::<13, 10>(end, buttons),
    (9, 8) => solve::<9, 8>(end, buttons),
    (6, 5) => solve::<6, 5>(end, buttons),
    (4, 5) => solve::<4, 5>(end, buttons),
    (5, 7) => solve::<5, 7>(end, buttons),
    (7, 7) => solve::<7, 7>(end, buttons),
    (10, 8) => solve::<10, 8>(end, buttons),
    (8, 7) => solve::<8, 7>(end, buttons),
    (7, 6) => solve::<7, 6>(end, buttons),
    (10, 9) => solve::<10, 9>(end, buttons),
    (7, 8) => solve::<7, 8>(end, buttons),
    (3, 4) => solve::<3, 4>(end, buttons),
    (9, 7) => solve::<9, 7>(end, buttons),
    (11, 10) => solve::<11, 10>(end, buttons),
    (9, 9) => solve::<9, 9>(end, buttons),
    (6, 8) => solve::<6, 8>(end, buttons),
    (11, 9) => solve::<11, 9>(end, buttons),
    (3, 5) => solve::<3, 5>(end, buttons),
    (4, 4) => solve::<4, 4>(end, buttons),
    (6, 6) => solve::<6, 6>(end, buttons),
    (7, 5) => solve::<7, 5>(end, buttons),
    (5, 6) => solve::<5, 6>(end, buttons),
    (8, 10) => solve::<8, 10>(end, buttons),
    (7, 9) => solve::<7, 9>(end, buttons),
    (9, 10) => solve::<9, 10>(end, buttons),

    _ => panic!("SIZE NOT DEFINED {},{}", end.len(), buttons.len()),
  }
}

fn solve<const R: usize, const N: usize>(end: &[u16], buttons: &[Vec<u16>]) -> u32 {
  let mut m: Matrix<R, N> = Matrix::create(end, buttons);
  let limits = m.limits();
  #[cfg(debug_assertions)]
  println!("{:?}", limits);
  #[cfg(debug_assertions)]
  println!("{}", m);

  gauss(&mut m);
  #[cfg(debug_assertions)]
  println!("{}", m);
  jordan(&mut m);
  #[cfg(debug_assertions)]
  println!("JORD:\n{}", m);
  let pivots = m.pivots();
  let free_v: Vec<_> = (0..R).into_iter().filter(|&i| !pivots.contains(&i)).map(|i| (i, limits[i])).collect();
  #[cfg(debug_assertions)]
  println!("FF: {free_v:?}");
  brute_find(&m, &free_v)
}


fn brute_find<const R: usize, const N: usize>(matrix: &Matrix<R, N>, values: &[(usize, i32)]) -> u32 {
  let mut answer = u32::MAX;
  'outer: for v in values.into_iter().map(|(_, v)| 0..=*v).multi_cartesian_product() {
    let vv: Vec<_> = v.iter().enumerate().map(|(i, vv)| (values[i].0, *vv)).collect();
    let s = vv.iter().map(|&(_, v)| v as u32).sum::<u32>();
    let mut m = Matrix { data: matrix.data.clone() };
    m.assign(&vv);
    for r in &mut m.data {
      if let Some(j) = r.find_first() && r[j] != 1 && r[j] != 0 {
        if r.answer % r[j] != 0 {
          continue 'outer;
        }
        r.answer /= r[j];
      }
    }
    if let Some(a) = m.sum_answers() {
      let ans = a + s;
      if ans < answer {
        answer = ans;
      }
    }
  }
  answer
}

fn gauss<const R: usize, const N: usize>(matrix: &mut Matrix<R, N>) {
  let mut n = 0;
  let mut off = 0;
  while n < N && n + off < R {
    while !matrix.find_and_swap(n, n + off) {
      off += 1;
      if n + off >= R {
        return;
      }
    }
    matrix.eliminate(n, n + off);
    //println!("{n}\n{}", matrix);
    n += 1;
  }
}

fn jordan<const R: usize, const N: usize>(matrix: &mut Matrix<R, N>) {
  for i in (1..N).rev() {
    if let Some(j) = matrix[i].find_first() {
      matrix.up_eliminate(i, j);
    }
  }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Row<const R: usize> {
  answer: i32,
  v: [i32; R],
}

impl<const R: usize> Row<R> {
  fn minus(&mut self, r: &Row<R>, col: usize) {
    let self_piv = self[col];
    let other_piv = r[col];

    for i in 0..R {
      self.v[i] *= other_piv;
      self.v[i] -= self_piv * r.v[i];
    }
    self.answer *= other_piv;
    self.answer -= self_piv * r.answer;
  }

  fn modulo(&mut self) {
    let some_n = self.v.into_iter().filter(|v| *v != 0).min_by_key(|v| v.abs());
    if let Some(n) = some_n && n != 0 && n != 1 {
      if self.v.iter().all(|&x| x % n == 0) && self.answer % n == 0 {
        self.v.iter_mut().for_each(|x| *x /= n);
        self.answer /= n;
      }
    }
  }

  fn find_first(&self) -> Option<usize> {
    self.v.iter().enumerate().find(|(_, v)| **v != 0).map(|(i, _)| i)
  }

  fn assign(&mut self, values: &[(usize, i32)]) {
    for (i, v) in values {
      self.answer -= self.v[*i] * v;
      self.v[*i] = 0;
    }
  }
}

struct Matrix<const R: usize, const N: usize> {
  data: [Row<R>; N],
}

impl<const R: usize, const N: usize> Matrix<R, N> {
  fn create(end: &[u16], buttons: &[Vec<u16>]) -> Matrix<R, N> {
    let mut m = Matrix { data: [Row { answer: 0, v: [0; R] }; N] };
    for (i, e) in end.iter().enumerate() {
      m[i].answer = *e as i32;
    }
    for (j, b) in buttons.iter().enumerate() {
      for i in b {
        m[*i as usize][j] = 1;
      }
    }
    m
  }

  fn limits(&self) -> [i32; R] {
    let l: Vec<_> = (0..R).into_iter()
                          .map(|j| (0..N).into_iter().map(|i| self[i].answer * self[i][j]).filter(|m| m > &0).min().unwrap())
                          .collect();
    l.try_into().unwrap()
  }

  fn find_and_swap(&mut self, i: usize, j: usize) -> bool {
    if let Some((k, _)) = self.data.iter().enumerate().skip(i).find(|(_, r)| r[j].abs() == 1) {
      self.data.swap(i, k);
      true
    } else {
      if let Some((k, _)) = self.data.iter().enumerate().skip(i).find(|(_, r)| r[j].abs() != 0) {
        self.data.swap(i, k);
        return true;
      }
      #[cfg(debug_assertions)]
      {
        println!("NOT FOUND {i},{j}");
        //println!("{}", self);
      };
      false
    }
  }

  fn eliminate(&mut self, i: usize, j: usize) {
    let pivot_row = self[i];
    for k in (i + 1)..N {
      if self[k][j] != 0 {
        self[k].minus(&pivot_row, j);
      }
      self[k].modulo();
    }
  }

  fn up_eliminate(&mut self, i: usize, j: usize) {
    let pivot_row = self[i];
    for k in 0..i {
      if self[k][j] != 0 {
        self[k].minus(&pivot_row, j);
        self[k].modulo();
      }
    }
  }
  fn pivots(&self) -> Vec<usize> {
    self.data.iter().filter_map(|r| r.find_first()).collect()
  }

  fn assign(&mut self, v: &[(usize, i32)]) {
    self.data.iter_mut().for_each(|r| r.assign(v))
  }

  fn sum_answers(&self) -> Option<u32> {
    let mut sum = 0u32;
    for r in self.data {
      if r.answer < 0 {
        return None;
      }
      sum += r.answer as u32;
    };
    Some(sum)
  }
}


impl<const R: usize> Index<usize> for Row<R> {
  type Output = i32;
  fn index(&self, i: usize) -> &Self::Output {
    self.v.index(i)
  }
}

impl<const R: usize> IndexMut<usize> for Row<R> {
  fn index_mut(&mut self, i: usize) -> &mut Self::Output {
    self.v.index_mut(i)
  }
}

impl<const R: usize, const N: usize> Index<usize> for Matrix<R, N> {
  type Output = Row<R>;

  fn index(&self, index: usize) -> &Self::Output {
    self.data.index(index)
  }
}

impl<const R: usize, const N: usize> IndexMut<usize> for Matrix<R, N> {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    self.data.index_mut(index)
  }
}

impl<const R: usize, const N: usize> Display for Matrix<R, N> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    for row in self.data {
      for n in row.v {
        write!(f, "{:3},", n)?;
      }
      writeln!(f, "||{:3}", row.answer)?;
    };
    Ok(())
  }
}
