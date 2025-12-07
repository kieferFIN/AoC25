use crate::tasks::t01::{T01A, T01B};
use crate::tasks::t02::{T02A, T02B};
use crate::tasks::t03::{T03A, T03B};
use crate::tasks::t04::{T04A, T04B};
use crate::tasks::t05::{T05A, T05B};
use crate::tasks::t06::{T06A, T06B};

mod t01;
mod t02;
mod t03;
mod t04;
mod t05;
mod t06;

const N: usize = 12;
pub const ALL_TASKS: [(&str, &Task); N] = [
  T01A.entry(), T01B.entry(),
  T02A.entry(), T02B.entry(),
  T03A.entry(), T03B.entry(),
  T04A.entry(), T04B.entry(),
  T05A.entry(), T05B.entry(),
  T06A.entry(), T06B.entry()
];

pub fn get_tasks() -> impl Iterator<Item=&'static Task> {
  ALL_TASKS.iter().map(|entry| entry.1)
}

pub fn find_task(name: &str) -> Option<&'static Task> {
  ALL_TASKS.iter().find(|entry| entry.0 == name).map(|entry| entry.1)
}

pub struct Task {
  name: &'static str,
  test_data: &'static str,
  answer: &'static str,
  task: fn(&str) -> String,
}

impl Task {
  pub const fn new(name: &'static str, test_data: &'static str, answer: &'static str, task: fn(&str) -> String) -> Self {
    Task { name, test_data, answer, task }
  }
  pub fn run(&self, data: &str) -> String {
    (self.task)(data)
  }

  const fn test_data(&self) -> &str {
    self.test_data
  }

  const fn test_answer(&self) -> &str {
    self.answer
  }

  pub const fn name(&self) -> &str {
    self.name
  }

  const fn entry(&self) -> (&str, &Task)
    where
      Self: Sized,
  {
    (self.name(), self)
  }

  pub fn test(&self) {
    println!("*******");
    println!("testing {}", self.name());
    let data = self.test_data();
    let answer = self.run(&data);
    if self.test_answer() == answer {
      println!("PASS");
      return;
    }
    println!("FAILED");
    println!(" get: {}\n should: {}", answer, self.test_answer());
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_all() {
    get_tasks().for_each(|task| {
      let result = task.run(task.test_data());
      assert_eq!(result, task.test_answer());
    })
  }
}
