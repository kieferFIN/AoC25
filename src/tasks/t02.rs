use crate::tasks::Task;

const TEST_DATA: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

pub struct Task02b {}

impl Task02b {
  pub const fn new() -> Task02b {
    Task02b {}
  }
}

impl Task for Task02b {
  fn run(&self, data: &str) -> String {
    data.trim()
        .split(',')
        .filter_map(|range| range.split_once('-'))
        .find_all_invalids()
        .sum::<usize>()
        .to_string()
  }

  fn test_data(&self) -> &str {
    TEST_DATA
  }

  fn test_answer(&self) -> &str {
    "4174379265"
  }

  fn name(&self) -> &str {
    "t02b"
  }
}

pub struct Task02a {}

impl Task02a {
  pub const fn new() -> Task02a {
    Task02a {}
  }
}

impl Task for Task02a {
  fn run(&self, data: &str) -> String {
    data.trim()
        .split(',')
        .filter_map(|range| range.split_once('-'))
        .find_invalids()
        .sum::<usize>()
        .to_string()
  }
  fn test_data(&self) -> &str {
    TEST_DATA
  }

  fn test_answer(&self) -> &str {
    "1227775554"
  }

  fn name(&self) -> &str {
    "t02a"
  }
}

type StringRange<'a> = (&'a str, &'a str);

struct InvalidFinderIter<I> {
  iter: I,
  end: usize,
  current: usize,
  validator: fn(usize) -> bool,
}

impl<'a, I> InvalidFinderIter<I>
  where
    I: Iterator<Item=StringRange<'a>>,
{
  pub fn new(iter: I, validator: fn(usize) -> bool) -> Self {
    InvalidFinderIter {
      iter,
      end: 0,
      current: 5,
      validator,
    }
  }
}

impl<'a, I> Iterator for InvalidFinderIter<I>
  where
    I: Iterator<Item=StringRange<'a>>,
{
  type Item = usize;

  fn next(&mut self) -> Option<Self::Item> {
    self.current += 1;
    loop {
      if self.current > self.end {
        let next = self.iter.next()?;
        self.current = next.0.parse().unwrap();
        self.end = next.1.parse().unwrap();
        #[cfg(debug_assertions)]
        println!("start {}, end {}", self.current, self.end);
      };

      if (self.validator)(self.current) {
        #[cfg(debug_assertions)]
        println!("hit {}", self.current);
        return Some(self.current);
      }
      self.current += 1;
    }
  }
}

trait InvalidFinderTrait<'a>: Iterator<Item=StringRange<'a>> + Sized {
  fn find_invalids(self) -> InvalidFinderIter<Self> {
    InvalidFinderIter::new(self, is_invalid_strict)
  }
  fn find_all_invalids(self) -> InvalidFinderIter<Self> {
    InvalidFinderIter::new(self, is_invalid)
  }
}

impl<'a, I: Iterator<Item=StringRange<'a>>> InvalidFinderTrait<'a> for I {}

fn is_invalid_strict(number: usize) -> bool {
  let s = number.to_string();
  let l = s.len();
  if l % 2 != 0 {
    return false;
  }
  let h = l / 2;
  s[0..h] == s[h..l]
}

fn is_invalid(number: usize) -> bool {
  let s = number.to_string();
  let l = s.len();
  for i in 1..=l / 2 {
    if l % i != 0 {
      continue;
    };
    let n = l / i;
    if s[0..i].repeat(n) == s {
      return true;
    };
  }
  false
}
