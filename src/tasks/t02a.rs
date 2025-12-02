use crate::tasks::{Task, name_from_file};

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
        "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
    }

    fn test_answer(&self) -> &str {
        "1227775554"
    }

    fn name(&self) -> &str {
        name_from_file(file!())
    }
}

type StringRange<'a> = (&'a str, &'a str);

struct InvalidFinderIter<I> {
    iter: I,
    end: usize,
    current: usize,
}

impl<'a, I> InvalidFinderIter<I>
where
    I: Iterator<Item = StringRange<'a>>,
{
    pub fn new(iter: I) -> Self {
        InvalidFinderIter {
            iter,
            end: 0,
            current: 5,
        }
    }
}

impl<'a, I> Iterator for InvalidFinderIter<I>
where
    I: Iterator<Item = StringRange<'a>>,
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
            if is_invalid(self.current) {
                #[cfg(debug_assertions)]
                println!("hit {}",self.current);
                return Some(self.current);
            }
            self.current += 1;
        }
    }
}

trait InvalidFinderTrait<'a>: Iterator<Item = StringRange<'a>> + Sized {
    fn find_invalids(self) -> InvalidFinderIter<Self> {
        InvalidFinderIter::new(self)
    }
}

impl<'a, I: Iterator<Item = StringRange<'a>>> InvalidFinderTrait<'a> for I {}

fn is_invalid(number: usize) -> bool {
    let s = number.to_string();
    let l = s.len();
    if l%2!=0 {return false;}
    let h = l/2;
    s[0..h] == s[h..l]
}
