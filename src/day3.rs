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
