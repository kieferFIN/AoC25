use std::collections::HashMap;
use std::str::FromStr;
use crate::tasks::Task;

const TEST_DATA_A: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

const TEST_DATA_B: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

const END: &str = "out";
const FFT: &str = "fft";
const DAC: &str = "dac";

pub const T11A: Task = Task::new("t11a", TEST_DATA_A, "5", run_a);
pub const T11B: Task = Task::new("t11b", TEST_DATA_B, "2", run_b);

fn run_a(data: &str) -> String {
  let nodes = parse_data(data);
  process(&nodes, "you", END).to_string()
}

fn run_b(data: &str) -> String {
  let nodes = parse_data(data);
  let fft_to_dac = find(&nodes, FFT, DAC, &[END], &mut HashMap::new());
  #[cfg(debug_assertions)]
  println!("***** {:?} ***********", fft_to_dac);
  let (first, second, length) = if fft_to_dac > 0 {
    (FFT, DAC, fft_to_dac)
  } else {
    let l = find(&nodes, DAC, FFT, &[END], &mut HashMap::new());
    #[cfg(debug_assertions)]
    println!("------------ {:?} ------------", l);
    (DAC, FFT, l)
  };
  let first_etappe = find(&nodes, "svr", first, &[END, second], &mut HashMap::new());
  let last_etappe = find(&nodes, second, END, &[], &mut HashMap::new());
  #[cfg(debug_assertions)]
  println!("{:?}", (first_etappe, length, last_etappe));
  (first_etappe as u64 * length as u64 * last_etappe as u64).to_string()
}

fn parse_data(data: &str) -> HashMap<String, Node> {
  data.lines().map(|line| line.parse().unwrap()).map(|node: Node| (node.name.clone(), node)).collect()
}

fn process(nodes: &HashMap<String, Node>, key: &str, end: &str) -> u32 {
  let node = &nodes[key];
  if node.children.iter().any(|c| *c == end) {
    return 1;
  }
  node.children.iter().map(|child| process(nodes, child, end)).sum()
}

fn find(nodes: &HashMap<String, Node>, start: &str, end: &str, stops: &[&str], path_cache: &mut HashMap<String, u32>) -> u32 {
  let node = &nodes[start];
  #[cfg(debug_assertions)]
  println!("{} {:?}", &node.name, &node.children);
  if let Some(n) = path_cache.get(&node.name) {
    #[cfg(debug_assertions)]
    println!("{} ## {}", &node.name, n);
    return *n;
  }
  let n = if node.children.iter().any(|c| *c == end) {
    1
  } else if node.children.iter().any(|c| stops.contains(&&**c)) {
    0
  } else {
    node.children.iter().map(|child| find(&nodes, child, end, stops, path_cache)).sum()
  };
  #[cfg(debug_assertions)]
  println!("{} -- {}", &node.name, n);
  path_cache.insert(node.name.clone(), n);
  n
}

struct Node {
  name: String,
  children: Vec<String>,
}

impl FromStr for Node {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, String> {
    let mut it = s.split(':');
    let name = it.next().ok_or(format!("No key; {}", s))?.to_string();
    let children = it.next().ok_or(format!("No child; {}", s))?
      .trim().split(' ')
      .map(|s| s.to_string()).collect();
    Ok(Node { name, children })
  }
}
