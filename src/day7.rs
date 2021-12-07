#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<isize> {
    input
        .split(',')
        .map(|line| line.parse::<isize>().unwrap())
        .collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &[isize]) -> isize {
    // Solve part 1 by finding the median value, then fold the numbers with the absolute difference
    // between the number and the median value.
    let mut numbers = Vec::from(input);
    numbers.sort_unstable();

    let median = numbers[numbers.len() / 2];

    input
        .iter()
        .fold(0, |result, number| result + (number - median).abs())
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &[isize]) -> isize {
    // Solve part 2 by finding mean value and
    let mean_f32 = input.iter().sum::<isize>() as f32 / input.len() as f32;

    [mean_f32.floor() as isize, mean_f32.ceil() as isize]
        .iter()
        .map(|mean| {
            input.iter().fold(0, |result, number| {
                let distance = (number - mean).abs();
                let cost = distance * (distance + 1) / 2;

                result + cost
            })
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

        assert_eq!(37, solve_part1(&input))
    }

    #[test]
    fn test_part2() {
        let input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

        assert_eq!(168, solve_part2(&input))
    }
}
