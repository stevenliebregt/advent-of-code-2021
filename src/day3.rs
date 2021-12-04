pub struct Input {
    number_width: usize,
    numbers: Vec<usize>,
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Input {
    Input {
        number_width: input.lines().next().unwrap().len(),
        numbers: input
            .lines()
            .map(|line| usize::from_str_radix(line, 2).unwrap())
            .collect(),
    }
}

fn divide_by_2_round_up(value: usize) -> usize {
    (value / 2) + (value & 1)
}

fn get_gamma_rate(numbers: &[usize], width: usize) -> usize {
    let input_length = numbers.len();

    (0..width).rev().fold(0, |result, number| {
        // Use bit shifting for each column to deduce if that index is a 1 or a 0, count those occurrences
        let count = numbers
            .iter()
            .filter(|n| ((*n >> number) & 1) == 1)
            .count();

        // If we have more ones than zeroes, OR 1, otherwise OR 0, then shift
        // Cast boolean to usize to get 0/1
        (result | (count >= divide_by_2_round_up(input_length)) as usize) << 1
    }) >> 1 // Shift once more to remove the extra 0
}

fn get_epsilon_rate(gamma_rate: usize, width: usize) -> usize {
    (!gamma_rate) & !(usize::MAX << width)
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &Input) -> usize {
    let gamma_rate = get_gamma_rate(&input.numbers, input.number_width);
    let epsilon_rate = get_epsilon_rate(gamma_rate, input.number_width);

    gamma_rate * epsilon_rate
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &Input) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_gamma_rate_3() {
        let width = 3;
        let numbers = vec![
            0b111,
            0b100,
            0b000,
        ];

        assert_eq!(0b100, get_gamma_rate(&numbers, width))
    }

    #[test]
    fn test_gamma_rate_5() {
        let width = 5;
        let numbers = vec![
            0b00100,
            0b11110,
            0b10110,
            0b10111,
            0b10101,
            0b01111,
            0b00111,
            0b11100,
            0b10000,
            0b11001,
            0b00010,
            0b01010,
        ];

        assert_eq!(0b10110, get_gamma_rate(&numbers, width))
    }

    #[test]
    fn test_get_epsilon_rate() {
        let width = 5;
        let gamma = 0b10110;
        let expected_epsilon = 0b01001;

        assert_eq!(expected_epsilon, get_epsilon_rate(gamma, width))
    }
}
