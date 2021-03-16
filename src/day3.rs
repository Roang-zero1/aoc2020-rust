#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
  let toboggan_map = input
    .lines()
    .map(|l| l.chars().map(|b| b).collect::<Vec<_>>())
    .collect();
  /*for line in &toboggan_map {
    for pos in line {
      print!("{}", pos)
    }
    print!("\n")
  }*/
  return toboggan_map;
}

#[aoc(day3, part1)]
pub fn solve_part1(toboggan_map: &Vec<Vec<char>>) -> u32 {
  let x = 3;
  let y = 1;
  let line_length = toboggan_map[0].len();
  let mut trees = 0;
  let mut current_x = 0;
  let mut current_y = 0;
  while current_y < toboggan_map.len() {
    if toboggan_map[current_y][current_x] == '#' {
      trees += 1
    }
    current_x = (current_x + x) % line_length;
    current_y += y;
  }
  return trees;
}
