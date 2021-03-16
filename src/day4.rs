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
