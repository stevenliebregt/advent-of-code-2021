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

pub enum Direction {
    FORWARD,
    UP,
    DOWN,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Direction::FORWARD),
            "up" => Ok(Direction::UP),
            "down" => Ok(Direction::DOWN),
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
            Direction::FORWARD => submarine.horizontal += self.amount,
            Direction::UP => submarine.depth -= self.amount,
            Direction::DOWN => submarine.depth += self.amount,
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
    let mut submarine = Submarine::new();

    input
        .iter()
        .for_each(|instruction| match instruction.direction {
            Direction::FORWARD => submarine.horizontal += instruction.amount,
            Direction::UP => submarine.depth -= instruction.amount,
            Direction::DOWN => submarine.depth += instruction.amount,
        });

    submarine.horizontal * submarine.depth
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Instruction]) -> isize {
    let mut submarine = Submarine::new();

    input
        .iter()
        .for_each(|instruction| match instruction.direction {
            Direction::FORWARD => {
                submarine.horizontal += instruction.amount;
                submarine.depth += instruction.amount * submarine.aim
            }
            Direction::UP => submarine.aim -= instruction.amount,
            Direction::DOWN => submarine.aim += instruction.amount,
        });

    submarine.horizontal * submarine.depth
}
