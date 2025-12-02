use crate::tasks::{Task, name_from_file};

pub struct Task02a {}

impl Task02a {
  pub const fn new() -> Task02a {
    Task02a {}
  }
}

impl Task for Task02a {
  fn run(&self, data: &str) -> String {
    data.to_string()
  }
  fn test_data(&self) -> &str {
    "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
  }

  fn test_answer(&self) -> &str {
    "1227775554"
  }

  fn name(&self) -> &str {
    name_from_file(file!())
  }
}
