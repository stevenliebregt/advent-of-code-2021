use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;

const BOARD_WIDTH: usize = 5;
const BOARD_HEIGHT: usize = 5;

const COLOR_RED: &str = "\x1b[0;31m";
const COLOR_GREEN: &str = "\x1b[0;32m";
const COLOR_RESET: &str = "\x1b[0;0m";

#[derive(Debug)]
pub struct Number {
    value: usize,
    marked: bool,
}

impl FromStr for Number {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            value: s.parse::<usize>()?,
            marked: false,
        })
    }
}

pub struct Board {
    numbers: Vec<Number>,
}

impl Board {
    pub fn mark(&mut self, number: usize) -> bool {
        self.numbers.iter_mut().for_each(|n| {
            if n.value == number {
                n.marked = true;
            }
        });

        self.check_rows() || self.check_columns()
    }

    fn check_rows(&self) -> bool {
        for i in 0..BOARD_HEIGHT {
            let mut values = Vec::with_capacity(BOARD_WIDTH);

            for j in 0..BOARD_WIDTH {
                values.push(&self.numbers[(i * BOARD_HEIGHT) + j]);
            }

            if values.iter().all(|num| num.marked) {
                return true;
            }
        }

        false
    }

    fn check_columns(&self) -> bool {
        for i in 0..BOARD_WIDTH {
            let mut values = Vec::with_capacity(BOARD_HEIGHT);

            for j in 0..BOARD_HEIGHT {
                values.push(&self.numbers[i + (j * BOARD_WIDTH)]);
            }

            if values.iter().all(|num| num.marked) {
                return true;
            }
        }

        false
    }

    fn sum_of_unmarked_numbers(&self) -> usize {
        self.numbers
            .iter()
            .filter_map(|number| {
                if number.marked {
                    None
                } else {
                    Some(number.value)
                }
            })
            .sum()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..BOARD_HEIGHT {
            for j in 0..BOARD_WIDTH {
                let num = &self.numbers[(i * BOARD_WIDTH) + j];
                let color = if num.marked { COLOR_GREEN } else { COLOR_RED };

                write!(f, "{}{:<5}{}", color, num.value, COLOR_RESET);
            }
            write!(f, "\n");
        }

        Ok(())
    }
}

pub struct Bingo {
    numbers: Vec<usize>,
    boards: Vec<Board>,
}

impl Bingo {
    pub fn from_input(input: &str) -> Self {
        let mut lines = input.lines();
        let numbers = lines
            .next()
            .unwrap()
            .trim()
            .split(',')
            .map(|c| c.parse::<usize>().unwrap())
            .collect();
        lines.next(); // Skip first empty line

        let mut boards: Vec<Board> = vec![];
        let mut board_numbers: Vec<Number> = Vec::with_capacity(BOARD_WIDTH * BOARD_HEIGHT);

        while let Some(line) = lines.next() {
            // Push the board and reset the numbers
            if line.trim().is_empty() {
                boards.push(Board {
                    numbers: board_numbers,
                });
                board_numbers = Vec::with_capacity(BOARD_WIDTH * BOARD_HEIGHT);
                continue;
            }

            board_numbers.append(
                &mut line
                    .trim()
                    .split_whitespace()
                    .map(|c| Number::from_str(c).unwrap())
                    .collect::<Vec<Number>>(),
            );
        }

        // Construct final board
        boards.push(Board {
            numbers: board_numbers,
        });

        Self { numbers, boards }
    }
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut bingo = Bingo::from_input(input);

    for number in &bingo.numbers {
        for board in bingo.boards.iter_mut() {
            if board.mark(*number) {
                println!("Winning board with number: {} = \n{:#}", number, board);

                return number * board.sum_of_unmarked_numbers();
            }
        }
    }

    0
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_gen() {
        let input = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#;

        let bingo = Bingo::from_input(input);

        assert_eq!(27, bingo.numbers.len());
        assert_eq!(3, bingo.boards.len())
    }

    #[test]
    fn test_part_1() {
        let input = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#;

        let expected = 4512;

        assert_eq!(expected, solve_part1(input))
    }
}
