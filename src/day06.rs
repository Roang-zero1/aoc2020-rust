static ASCII_LOWER: [char; 26] = [
  'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
  't', 'u', 'v', 'w', 'x', 'y', 'z',
];

#[aoc(day6, part1, fold)]
pub fn solve_part1_fold(customs_forms: &str) -> u16 {
  let customs_forms: Vec<&str> = customs_forms.split("\n\n").collect();
  customs_forms.iter().fold(0u16, |sum, form| {
    sum
      + ASCII_LOWER.iter().fold(0u16, |r, c| {
        if form.matches(*c).count() > 0 {
          //println!("{} - {} matches {}", r, c, form);
          r + 1
        } else {
          r
        }
      })
  })
}

#[aoc(day6, part1, map)]
pub fn solve_part1_map(customs_forms: &str) -> u16 {
  let customs_forms: Vec<&str> = customs_forms.split("\n\n").collect();
  customs_forms
    .iter()
    .map(|form| {
      ASCII_LOWER
        .iter()
        .map(|c| {
          if form.matches(*c).count() > 0 {
            //println!("{} - {} matches {}", r, c, form);
            1
          } else {
            0
          }
        })
        .sum::<u16>()
    })
    .sum()
}
