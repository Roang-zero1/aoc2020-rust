#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
  input.lines().map(|l| l.parse::<u32>().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
  for x in 0..input.len() - 1 {
    for y in x + 1..input.len() {
      if (input[x] + input[y]) == 2020 {
        println!("Found correct numbers: {} + {} = 2020", input[x], input[y]);
        return input[x] * input[y];
      }
    }
  }

  return 0;
}
