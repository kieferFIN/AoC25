use std::collections::HashMap;
use t01a::Task01a;
use crate::tasks::t01b::Task01b;
use crate::tasks::t02a::Task02a;

mod t01a;
mod t01b;
mod t02a;

pub fn get_tasks() -> HashMap<&'static str, &'static dyn Task> {
  HashMap::from([T01A.entry(), T01B.entry()])
}
const T01A: Task01a = Task01a::new();
const T01B: Task01b = Task01b::new();
const T02A: Task02a = Task02a::new();

pub trait Task {
  fn run(&self, data: &str) -> String;
  fn test_data(&self) -> &str;
  fn test_answer(&self) -> &str;

  fn name(&self) -> &str;

  fn entry(&self) -> (&str, &dyn Task)
  where
    Self: Sized,
  {
    (self.name(), self)
  }

  fn test(&self) {
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

fn name_from_file(file: &str) -> &str {
  file.split('\\').last().unwrap().split('.').next().unwrap()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_all() {
    get_tasks().iter().for_each(|entry| {
      let task = entry.1;
      let result = task.run(task.test_data());
      assert_eq!(result, task.test_answer());
    })
  }
}
