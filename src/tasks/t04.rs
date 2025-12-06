use crate::tasks::Task;

const TEST_DATA: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

pub const T04A: Task = Task::new("t04a", TEST_DATA, "13", run_a);
pub const T04B: Task = Task::new("t04b", TEST_DATA, "43", run_b);

fn run_a(data: &str) -> String {
  let line_it = data.lines();
  let mut grid: Vec<Vec<u32>> = line_it.map(|l| l.collect_line()).collect();
  check_grid(&mut grid, false).to_string()
}

fn run_b(data: &str) -> String {
  let mut total: u32 = 0;
  let line_it = data.lines();
  let mut grid: Vec<Vec<u32>> = line_it.map(|l| l.collect_line()).collect();
  let mut s = check_grid(&mut grid, true);
  while s > 0 {
    total += s;
    s = check_grid(&mut grid, true);
  };
  total.to_string()
}

fn check_grid(grid: &mut Vec<Vec<u32>>, remove: bool) -> u32 {
  let mut it = grid.iter_mut();
  let firs_line = it.next().unwrap();
  let size = firs_line.len();
  let dummy_line1 = vec![1; size];
  let dummy_line2 = vec![1; size];


  let (s, prev, mut current) = it.fold((0u32, &dummy_line1, firs_line), |mut data, next_line| {
    let s = check_line(&data.1, &mut data.2, next_line, remove);
    (data.0 + s, data.2, next_line)
  });
  check_line(&prev, &mut current, &dummy_line2, remove) + s
}

fn check_line(prev: &Vec<u32>, current: &mut Vec<u32>, next: &Vec<u32>, remove: bool) -> u32 {
  let mut correct = 0;
  let size = current.len();
  for i in 1..size - 1 {
    if current[i] == 1 { continue; };
    let s = prev[i - 1..=i + 1].iter().sum::<u32>() + next[i - 1..=i + 1].iter().sum::<u32>() + current[i - 1] + current[i + 1];
    if s > 4 {
      correct += 1;
      if remove {
        current[i] = 1;
      }
      #[cfg(debug_assertions)]
      println!("{}\n{:?}\n{:?}\n{:?}\n", i, &prev[i - 1..=i + 1], &current[i - 1..=i + 1], &next[i - 1..=i + 1]);
    }
  };
  #[cfg(debug_assertions)]
  println!("****");
  correct
}

trait LineCollector {
  fn collect_line(self) -> Vec<u32>;
}

impl LineCollector for &str {
  fn collect_line(self) -> Vec<u32> {
    [1].into_iter().chain(self.chars().map(|c| if c == '.' { 1 } else { 0 })).chain([1].into_iter()).collect()
  }
}
