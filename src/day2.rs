use lazy_static::lazy_static;
use regex::Regex;

pub struct PwCheck {
  min: u32,
  max: u32,
  character: char,
  password: String,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<PwCheck> {
  input
    .lines()
    .map(|l| {
      lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) ([[:alpha:]]): (\w*)$").unwrap();
      }
      let cap = RE.captures(l).unwrap();
      PwCheck {
        min: cap[1].parse::<u32>().unwrap(),
        max: cap[2].parse::<u32>().unwrap(),
        character: cap[3].chars().nth(0).unwrap(),
        password: cap[4].to_string(),
      }
    })
    .collect()
}
