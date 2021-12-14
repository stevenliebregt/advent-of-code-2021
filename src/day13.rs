use std::cmp;
use std::collections::HashSet;
use std::fmt::Display;
use std::iter::FromIterator;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub enum Instruction {
    X(i32),
    Y(i32),
}

type Input = (Vec<Point>, Vec<Instruction>);

pub struct Paper {
    data: HashSet<Point>,
}

impl Paper {
    pub fn new(data: HashSet<Point>) -> Self {
        Self { data }
    }

    pub fn fold(self, instruction: Instruction) -> Self {
        let mut new_points = HashSet::<Point>::new();

        for mut point in self.data.into_iter() {
            match instruction {
                Instruction::X(column_index) => {
                    if point.x > column_index {
                        point.x = column_index - (point.x - column_index)
                    }
                }
                Instruction::Y(row_index) => {
                    if point.y > row_index {
                        point.y = row_index - (point.y - row_index)
                    }
                }
            }

            new_points.insert(point);
        }

        Self::new(new_points)
    }

    pub fn width(&self) -> i32 {
        self.data
            .iter()
            .fold(0, |acc, point| cmp::max(acc, point.x))
            + 1
    }

    pub fn height(&self) -> i32 {
        self.data
            .iter()
            .fold(0, |acc, point| cmp::max(acc, point.y))
            + 1
    }
}

impl Display for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() {
                write!(
                    f,
                    "{}",
                    if self.data.contains(&Point { x, y }) {
                        "#"
                    } else {
                        " "
                    }
                )?;
            }

            writeln!(f)?;
        }
        write!(f, "")
    }
}

fn parse_input(input: &str) -> Input {
    let (points_str, instructions_str) = input.split_once("\n\n").unwrap();

    let points = points_str
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(x_str, y_str)| Point {
            x: x_str.parse::<i32>().unwrap(),
            y: y_str.parse::<i32>().unwrap(),
        })
        .collect::<Vec<Point>>();

    let instructions = instructions_str
        .lines()
        .map(|line| line.split(' ').nth(2).unwrap().split_once('=').unwrap())
        .map(|(direction, amount_str)| {
            let amount = amount_str.parse::<i32>().unwrap();

            match direction {
                "x" => Instruction::X(amount),
                "y" => Instruction::Y(amount),
                _ => unreachable!(),
            }
        })
        .collect::<Vec<Instruction>>();

    (points, instructions)
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &str) -> usize {
    let data = parse_input(input);
    let mut paper = Paper::new(HashSet::<Point>::from_iter(data.0.into_iter()));

    for instruction in data.1.into_iter().take(1) {
        paper = paper.fold(instruction)
    }

    paper.data.len()
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &str) -> usize {
    let data = parse_input(input);
    let mut paper = Paper::new(HashSet::<Point>::from_iter(data.0.into_iter()));

    for instruction in data.1 {
        paper = paper.fold(instruction);

        if paper.height() == 6 {
            break;
        }
    }

    println!("{}", paper);

    0
}
