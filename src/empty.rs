#[aoc_generator(day0)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect()
}

#[aoc(day0, part1)]
pub fn solve_part1(input: &[usize]) -> usize {
    0
}

#[aoc(day0, part2)]
pub fn solve_part2(input: &[usize]) -> usize {
    0
}
