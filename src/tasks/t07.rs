use std::collections::{HashMap, HashSet};
use crate::tasks::Task;

const TEST_DATA:&str =".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

const START:char = 'S';
const EMPTY:u8 ='.' as u8;
const SPLITTER:u8 = '^' as u8;

pub const T07A: Task = Task::new("t07a", TEST_DATA, "21", run_a);
pub const T07B: Task = Task::new("t07b", TEST_DATA, "40", run_b);

fn run_a(data: &str)->String {
  let mut it = data.lines();
  let first_line = it.next().unwrap();
  let idx = first_line.find(START).unwrap();
  let (total,_)=it.fold((0,HashSet::from([idx])),|(mut s,idxs), line| {
    let line = line.as_bytes();
    let mut set = HashSet::new();
    for id in idxs{
      match line[id] {
        SPLITTER => {set.insert(id-1); set.insert(id+1); s += 1;},
        EMPTY => {set.insert(id);},
        _ => {panic!("invalid character {}, at {}", line[id], id)}
      };
    };
    (s,set)
  });

  total.to_string()
}

fn run_b(data: &str)->String {
  let mut it = data.lines();
  let first_line = it.next().unwrap();
  let idx = first_line.find(START).unwrap();
  let (total,_)=it.fold((1,HashMap::from([(idx,1u64)])),|(mut s,idxs), line| {
    let line = line.as_bytes();
    let mut set= HashMap::new();
    for (id,n) in idxs{
      match line[id] {
        SPLITTER => {set.add(id-1, n); set.add(id+1,n); s += n;},
        EMPTY => set.add(id,n),
        _ => {panic!("invalid character {}, at {}", line[id], id)}
      };
    };
    (s,set)
  });

  total.to_string()
}

trait Adder{
  fn add(&mut self, id: usize, n:u64);
}

impl Adder for HashMap<usize, u64> {
  fn add(&mut self, id: usize, n:u64) {
    if let Some(v) = self.get(&id){
      self.insert(id, v+n);
    } else { self.insert(id, n); }
  }
}