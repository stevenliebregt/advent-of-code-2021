use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone)]
pub struct Cave(BTreeMap<(i32, i32), u32>, BTreeSet<(i32, i32)>);

impl Cave {
    pub fn height_at(&self, (x, y): (i32, i32)) -> u32 {
        self.0.get(&(x, y)).unwrap_or(&u32::MAX).clone()
    }

    pub fn find_low_points(&self) -> Vec<(&(i32, i32), &u32)> {
        self.0
            .iter()
            .filter(|((x, y), current_depth)| {
                // Get all neighbours
                let north = self.height_at((*x, *y - 1));
                let east = self.height_at((*x + 1, *y));
                let south = self.height_at((*x, y + 1));
                let west = self.height_at((*x - 1, *y));

                north > **current_depth
                    && east > **current_depth
                    && south > **current_depth
                    && west > **current_depth
            })
            .collect::<Vec<_>>()
    }

    pub fn explore_basin(&mut self, (x, y): (i32, i32)) -> usize {
        let mut basin = BTreeSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((x, y));

        while let Some((x, y)) = queue.pop_front() {
            // Ignore if we already processed this position
            if self.1.contains(&(x, y)) {
                continue;
            }

            // Search to find where there is height 9
            if (0..9).contains(&self.height_at((x, y))) {
                if basin.insert((x, y)) {
                    queue.push_back((x, y - 1));
                    queue.push_back((x - 1, y));
                    queue.push_back((x + 1, y));
                    queue.push_back((x, y + 1));
                }
            }
        }

        basin.len()
    }
}

impl FromStr for Cave {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cave = (s.lines().enumerate())
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, height)| ((x as i32, y as i32), height.to_digit(10).unwrap()))
            })
            .collect();

        Ok(Self(cave, BTreeSet::new()))
    }
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let cave: Cave = input.parse().unwrap();

    cave.find_low_points()
        .iter()
        .fold(0, |risk, (_, depth)| risk + (*depth + 1))
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let mut cave: Cave = input.parse().unwrap();
    let clone = cave.clone();

    // Find basins
    let mut basins = clone
        .find_low_points()
        .iter()
        .map(|(position, _)| cave.explore_basin(**position) as u32)
        .collect::<Vec<_>>();

    basins.sort();
    basins.iter().rev().take(3).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let input = r#"2199943210
3987894921
9856789892
8767896789
9899965678"#;

        assert_eq!(15, solve_part1(&input))
    }

    #[test]
    fn test_example_part2() {
        let input = r#"2199943210
3987894921
9856789892
8767896789
9899965678"#;

        assert_eq!(1134, solve_part2(&input))
    }
}
