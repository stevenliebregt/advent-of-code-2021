use std::str::FromStr;

pub struct Submarine {
    horizontal: isize,
    depth: isize,
    aim: isize,
}

impl Submarine {
    pub fn new() -> Self {
        Submarine {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }
}

impl Default for Submarine {
    fn default() -> Self {
        Self::new()
    }
}

pub enum Direction {
    Forward,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Direction::Forward),
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            _ => Err(()),
        }
    }
}

pub struct Instruction {
    direction: Direction,
    amount: isize,
}

impl Instruction {
    pub fn apply(&self, submarine: &mut Submarine) {
        match self.direction {
            Direction::Forward => submarine.horizontal += self.amount,
            Direction::Up => submarine.depth -= self.amount,
            Direction::Down => submarine.depth += self.amount,
        }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();

        Ok(Instruction {
            direction: Direction::from_str(split.next().unwrap()).unwrap(),
            amount: split.next().unwrap().parse::<isize>().unwrap(),
        })
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| Instruction::from_str(line).unwrap())
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Instruction]) -> isize {
    let mut submarine = Submarine::default();

    input
        .iter()
        .for_each(|instruction| match instruction.direction {
            Direction::Forward => submarine.horizontal += instruction.amount,
            Direction::Up => submarine.depth -= instruction.amount,
            Direction::Down => submarine.depth += instruction.amount,
        });

    submarine.horizontal * submarine.depth
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Instruction]) -> isize {
    let mut submarine = Submarine::default();

    input
        .iter()
        .for_each(|instruction| match instruction.direction {
            Direction::Forward => {
                submarine.horizontal += instruction.amount;
                submarine.depth += instruction.amount * submarine.aim
            }
            Direction::Up => submarine.aim -= instruction.amount,
            Direction::Down => submarine.aim += instruction.amount,
        });

    submarine.horizontal * submarine.depth
}
