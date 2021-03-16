use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Direction {
  Lower,
  Upper,
}

impl FromStr for Direction {
  type Err = ();

  fn from_str(input: &str) -> Result<Direction, Self::Err> {
    match input {
      "F" => Ok(Direction::Lower),
      "B" => Ok(Direction::Upper),
      "L" => Ok(Direction::Lower),
      "R" => Ok(Direction::Upper),
      _ => Err(()),
    }
  }
}

pub fn binary_space_partitioning(available: (u16, u16), dir: &str) -> (u16, u16) {
  let direction = Direction::from_str(dir).unwrap();
  let range = available.1 - available.0;
  match direction {
    Direction::Lower => (available.0, available.0 + range / 2),
    Direction::Upper => (available.1 - range / 2, available.1),
  }
}

#[aoc(day5, part1)]
pub fn solve_part1(boarding_passes: &str) -> u16 {
  let mut max = 0;
  for line in boarding_passes.lines() {
    let mut rows = (0, 127);
    let mut columns = (0, 7);
    for direction in line.chars() {
      if direction == 'F' || direction == 'B' {
        //println!("rows: {},{}/{},{}", rows.0, rows.1, columns.0, columns.1);
        rows = binary_space_partitioning(rows, &direction.to_string())
      } else {
        //println!("columns: {},{}/{},{}", rows.0, rows.1, columns.0, columns.1);
        columns = binary_space_partitioning(columns, &direction.to_string())
      }
    }
    let res = (rows.0 * 8) + columns.0;
    //println!("{}/{} = {}", rows.0, columns.0, res);
    if res > max {
      max = res
    }
  }
  max
}

#[aoc(day5, part2)]
pub fn solve_part2(boarding_passes: &str) -> u16 {
  let mut rows: HashMap<u16, Vec<u16>> = HashMap::new();
  for line in boarding_passes.lines() {
    let mut rows_available = (0, 127);
    let mut columns_available = (0, 7);
    for direction in line.chars() {
      if direction == 'F' || direction == 'B' {
        //println!("rows: {},{}/{},{}", rows.0, rows.1, columns.0, columns.1);
        rows_available = binary_space_partitioning(rows_available, &direction.to_string())
      } else {
        //println!("columns: {},{}/{},{}", rows.0, rows.1, columns.0, columns.1);
        columns_available = binary_space_partitioning(columns_available, &direction.to_string())
      }
    }
    rows.entry(rows_available.0).or_insert(Vec::new());
    rows
      .entry(rows_available.0)
      .and_modify(|r| r.push(columns_available.0));
  }
  for (row, seats) in rows.iter_mut() {
    if seats.len() == 8 {
      continue;
    }
    seats.sort();
    //println!("{}", row);
    loop {
      let current = seats.pop().unwrap();
      if seats.len() == 0 {
        break;
      }
      if current - 2 == *seats.last().unwrap() {
        println!("Seat found at: {}/{}", row, current - 1);
        return row * 8 + current - 1;
      }
    }
  }
  0
}
