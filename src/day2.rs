use lazy_static::lazy_static;
use regex::Regex;

pub struct PwCheck {
  min: String,
  max: String,
  character: String,
  password: String
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<PwCheck> {

  input.lines().map(|l| {
    lazy_static! {
      static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) ([[:alpha:]]): (\w*)$").unwrap();
    }
    let cap = RE.captures(l).unwrap();
    PwCheck {
      min: cap[1].to_string(),
      max: cap[2].to_string(),
      character: cap[3].to_string(),
      password: cap[4].to_string()
    }}
  ).collect()
}
