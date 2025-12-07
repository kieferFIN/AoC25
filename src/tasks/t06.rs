use std::iter::zip;
use std::str::FromStr;
use crate::tasks::Task;

const TEST_DATA: &str = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +   ";


pub const T06A: Task = Task::new("t06a", TEST_DATA, "4277556", run_a);
pub const T06B: Task = Task::new("t06b", TEST_DATA, "3263827", run_b);

fn run_a(data: &str) -> String {
  let mut total = 0;
  let mut grid = Vec::new();
  let mut lines = data.lines();
  let first_line = lines.next().unwrap();

  for n in first_line.split_whitespace() {
    let n = n.parse::<u64>().unwrap();
    let v = vec![n];
    grid.push(v);
  }
  for line in lines {
    for (i, c) in line.split_whitespace().enumerate() {
      if c.starts_with('+') {
        total += grid[i].iter().sum::<u64>();
      } else if c.starts_with('*') {
        total += grid[i].iter().product::<u64>();
      } else {
        let n = c.parse::<u64>().unwrap();
        grid[i].push(n);
      }
    }
  };
  total.to_string()
}

fn run_b(data: &str) -> String {
  let lines: Vec<&str> = data.lines().collect();
  let (ops, widths) = read_operation_row(lines.last().unwrap());
  let mut columns: Vec<Column> = widths.iter().map(|w| Column::new(*w)).collect();
  for row in &lines[..lines.len() - 1] {
    read_row(row, &widths, &mut columns)
  };
  zip(columns, &ops).map(|(column, op)| column.calc(op)).sum::<u64>().to_string()
}

fn read_operation_row(row: &str) -> (Vec<Operation>, Vec<usize>) {
  let mut ops = Vec::new();
  let mut widths = Vec::new();
  let mut rest_row = row;
  while rest_row.len() > 0 {
    let c = &rest_row[0..1];
    ops.push(Operation::from(c).unwrap());
    let (s, n) = (&rest_row[1..]).trim_start_number();
    widths.push(n);
    rest_row = s;
  };

  (ops, widths)
}

fn read_row(row: &str, widths: &[usize], columns: &mut Vec<Column>) {
  let mut rest_row = row;
  let mut c = 0;
  loop {
    let w = widths[c];
    let cell: Cell = rest_row[0..w].parse().unwrap();
    #[cfg(debug_assertions)]
    println!("{:?}",cell);
    columns[c].add(cell);
    c += 1;
    if c>= columns.len() { break  }
    rest_row = &rest_row[w+1..];
  }
  #[cfg(debug_assertions)]
  println!("****");
}

enum Operation {
  Plus,
  Mul,
}

impl Operation {
  fn calc<I: Iterator<Item=u32>>(&self, iter: I) -> u64 {
    let it = iter.map(|i| i as u64);

    match self {
      Operation::Plus => it.sum(),
      Operation::Mul => it.product(),
    }
  }

  fn from(c: &str) -> Option<Operation> {
    match c {
      "+" => Some(Operation::Plus),
      "*" => Some(Operation::Mul),
      _ => None,
    }
  }
}

trait InfoTrimmer {
  fn trim_start_number(&self) -> (&str, usize);
}

impl InfoTrimmer for str {
  fn trim_start_number(&self) -> (&str, usize) {
    let og_len = self.len();
    let s = self.trim_start();
    (s, og_len - s.len())
  }
}

#[derive(Debug)]
struct Cell {
  value: Vec<u32>,
}

impl FromStr for Cell {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Cell { value: s.chars().map(|c| if c.is_whitespace() { 0 } else { c.to_digit(10).unwrap() }).collect() })
  }
}

struct Column {
  numbers: Vec<u32>,
}

impl Column {
  fn new(w:usize) -> Column {
    Column {numbers: vec![0; w]}
  }

  fn add(&mut self, c: Cell) {
    zip(self.numbers.iter_mut(),c.value.iter()).for_each(
      |(a, b)| {
        if *b != 0 {
        *a *= 10;
        *a += *b;
        }
      }
    )
  }

  fn calc(self, op: &Operation) -> u64 {
    #[cfg(debug_assertions)]
    println!("{:?}",self.numbers);
    op.calc(self.numbers.into_iter())
  }
}
