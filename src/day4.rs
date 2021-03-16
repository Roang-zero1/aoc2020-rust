use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

#[aoc_generator(day4)]
pub fn input_generator<'a>(input: &'a str) -> Vec<HashMap<String, String>> {
  let raw_passports = input.split("\n\n");
  let mut passports = Vec::new();
  for pass in raw_passports {
    let values: Vec<(String, String)> = pass
      .lines()
      .map(|l| {
        l.split(" ")
          .map(|v| {
            let mut values = v.split(":");
            (
              values.next().unwrap().to_string(),
              values.next().unwrap().to_string(),
            )
          })
          .collect::<Vec<(String, String)>>()
      })
      .collect::<Vec<Vec<(String, String)>>>()
      .concat();
    let mut passport: HashMap<String, String> = HashMap::new();
    for val in values {
      passport.insert(val.0, val.1);
    }
    passports.push(passport)
  }
  return passports;
}

#[aoc(day4, part1)]
pub fn solve_part1(passports: &Vec<HashMap<String, String>>) -> u8 {
  let mut valid: u8 = 0;
  for pass in passports {
    if pass.len() == 8 || (pass.len() == 7 && !pass.contains_key("cid")) {
      valid += 1
    }
  }
  valid
}

static EYE_COLOR: &[&str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

#[aoc(day4, part2)]
pub fn solve_part2(passports: &Vec<HashMap<String, String>>) -> u8 {
  let mut valid_number: u8 = 0;
  for pass in passports {
    if pass.len() == 8 || (pass.len() == 7 && !pass.contains_key("cid")) {
      //byr (Birth Year) - four digits; at least 1920 and at most 2002.
      let byr = pass.get("byr").unwrap().parse::<u32>().unwrap();
      if byr < 1920 || byr > 2002 {
        println!("byr: {}", byr);
        continue;
      }
      //iyr (Issue Year) - four digits; at least 2010 and at most 2020.
      let iyr = pass.get("iyr").unwrap().parse::<u32>().unwrap();
      if iyr < 2010 || iyr > 2020 {
        println!("iyr: {}", iyr);
        continue;
      }
      //eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
      let eyr = pass.get("eyr").unwrap().parse::<u32>().unwrap();
      if eyr < 2020 || eyr > 2030 {
        println!("eyr: {}", eyr);
        continue;
      }
      //hgt (Height) - a number followed by either cm or in:
      //If cm, the number must be at least 150 and at most 193.
      //If in, the number must be at least 59 and at most 76.
      let hgt = pass.get("hgt").unwrap();
      let hgt_len = hgt.len();
      let unit = &hgt[hgt_len - 2..];
      if unit == "cm" {
        let height = hgt[0..hgt_len - 2].parse::<u32>().unwrap();
        if height < 150 || height > 193 {
          println!("hgt: {}", hgt);
          continue;
        }
      } else if unit == "in" {
        let height = hgt[0..hgt_len - 2].parse::<u32>().unwrap();
        if height < 59 || height > 76 {
          println!("hgt: {}", hgt);
          continue;
        }
      } else {
        println!("hgt: {}", hgt);
        continue;
      }
      //hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
      lazy_static! {
        static ref RE_HCL: Regex = Regex::new(r"#[0-9a-f]{6}").unwrap();
      }
      let hcl = pass.get("hcl").unwrap();
      if !RE_HCL.is_match(hcl) {
        println!("hcl: {}", hcl);
        continue;
      }
      //ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
      let ecl = pass.get("ecl").unwrap();
      if !EYE_COLOR.contains(&&ecl[..]) {
        println!("ecl: {}", ecl);
        continue;
      }
      //pid (Passport ID) - a nine-digit number, including leading zeroes.
      lazy_static! {
        static ref RE_PID: Regex = Regex::new(r"^\d{9}$").unwrap();
      }
      let pid = pass.get("pid").unwrap();
      if !RE_PID.is_match(pid) {
        println!("pid: {}", pid);
        continue;
      }
      //cid (Country ID) - ignored, missing or not.

      valid_number += 1;
    }
  }
  valid_number
}
