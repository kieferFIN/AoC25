use std::{env, fs};
use std::path::Path;
use crate::tasks::{find_task, get_tasks};

mod tasks;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    get_tasks().for_each(|task| task.test());
    return;
  }
  let task_name = &args[1];
  if args.len() == 3 {
    find_task(task_name).unwrap().test();
    return;
  }
  let path_str = "./input/".to_owned() + &task_name + ".txt";
  let path = Path::new(&path_str);
  let data = fs::read_to_string(path).unwrap();
  let result = find_task(task_name).unwrap().run(&data);
  println!("RESULT:\n{}", result);
}
