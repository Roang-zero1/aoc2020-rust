use lazy_static::lazy_static;
use regex::Regex;
use std::convert::TryInto;

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

#[aoc(day2, part1)]
pub fn solve_part1(input: &[PwCheck]) -> u32 {
  let mut valid: u32 = 0;
  for pw in input {
    let count = pw
      .password
      .matches(pw.character)
      .count()
      .try_into()
      .unwrap();
    /*print!(
      "{}-{} {}: {} = {}",
      pw.min, pw.max, pw.character, pw.password, count
    );*/
    if pw.min <= count && pw.max >= count {
      //println!(": OK");
      valid += 1;
    } else {
      //println!(": NOK");
    }
  }
  return valid;
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[PwCheck]) -> u32 {
  let mut valid: u32 = 0;
  for pw in input {
    let mut cnt: u32 = 0;
    if pw.password.chars().nth((pw.min as usize) - 1).unwrap() == pw.character {
      cnt += 1
    }
    if pw.password.chars().nth(pw.max as usize - 1).unwrap() == pw.character {
      cnt += 1
    }
    if cnt == 1 {
      valid += 1
    }
  }
  return valid;
}
