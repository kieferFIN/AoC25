use std::{env, fs};
use std::path::Path;
use crate::tasks::get_tasks;

mod tasks;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    let tasks = get_tasks();
    return tasks.iter().for_each(|entry| entry.1.test())
  }
  let file_name = &args[1];
  let path_str = "./input/".to_owned() + &file_name +".txt";
  let path = Path::new(&path_str);
  let data = fs::read_to_string(path).unwrap();
  let result = get_tasks()[&file_name as &str].run(&data);
  println!("RESULT:\n{}", result);

}
