pub fn simulate_lanternfish_school(input: &[usize], days: usize) -> usize {
    // Split school on timer
    let mut school: [usize; 9] = [0; 9];

    for fish in input {
        school[*fish] += 1;
    }

    // Simulate
    for _ in 0..days {
        school.rotate_left(1);
        school[6] += school[8];
    }

    school.iter().sum()
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input.split(',').map(|x| x.parse().unwrap()).collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[usize]) -> usize {
    simulate_lanternfish_school(&input, 80)
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[usize]) -> usize {
    simulate_lanternfish_school(&input, 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = input_generator("3,4,3,1,2");

        assert_eq!(5, input.len());
    }

    #[test]
    fn test_part1_example_18days() {
        let mut input = input_generator("3,4,3,1,2");

        assert_eq!(26, simulate_lanternfish_school(&mut input, 18))
    }

    #[test]
    fn test_part1_example_80days() {
        let mut input = input_generator("3,4,3,1,2");

        assert_eq!(5934, simulate_lanternfish_school(&mut input, 80))
    }

    #[test]
    fn test_part2_example_256days() {
        let mut input = input_generator("3,4,3,1,2");

        assert_eq!(26984457539, simulate_lanternfish_school(&mut input, 256))
    }
}
