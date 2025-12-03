use crate::tasks::Task;

const TEST_DATA: &str = "987654321111111\n811111111111119\n234234234234278\n818181911112111";

pub const T03A: Task = Task::new("t03a", TEST_DATA, "357", run_a);
pub const T03B: Task = Task::new("t03b", TEST_DATA, "3121910778619", run_b);


fn run_a(data: &str) -> String {
  joltage_parser(data,find_small_joltage)
}

fn run_b(data: &str) -> String {
  joltage_parser(data,find_big_joltage)
}

fn joltage_parser(data: &str, finder: fn(Vec<u8>)-> u64) -> String {
  data.lines().map(|line| line.bytes().map(|b| b - 48).collect()).map(finder).sum::<u64>().to_string()
}

fn find_small_joltage(bank: Vec<u8>) -> u64 {
  find_joltage(bank, 2)
}

fn find_big_joltage(bank: Vec<u8>) -> u64 {
  find_joltage(bank, 12)
}

fn find_joltage(bank: Vec<u8>, n: u8) -> u64 {
  let l = bank.len();
  let (sum_value, _) = (0..n as usize).rev().fold((0u64, 0), |(sum, s), e| {
    let (v, i) = (&bank[s..l - e]).find_max_and_index();
    //println!(" ** [{}..{}],{} {} ",s,l - e,v,i);
    (sum * 10 + v as u64, i+s+1)
  });
  #[cfg(debug_assertions)]
  println!("jolt {}", sum_value);
  sum_value
}

trait MaxFinder<T> {
  fn find_max_and_index(&self) -> (T, usize);
}

impl<T: PartialOrd<T> + Copy> MaxFinder<T> for &[T] {
  fn find_max_and_index(&self) -> (T, usize) {
    self.iter().enumerate().fold((self[0], 0), |(max, index), (i, v)| {
      if *v > max { (*v, i) } else { (max, index) }
    })
  }
}
