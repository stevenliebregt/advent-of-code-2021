use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

fn get_range(a: usize, b: usize) -> Vec<usize> {
    if a <= b {
        (a..=b).collect()
    } else {
        (b..=a).rev().collect()
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Point {
    x: usize,
    y: usize,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap();
        Ok(Self {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

#[derive(Debug)]
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn is_horizontal_or_vertical(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }

    pub fn points(&self) -> Vec<Point> {
        let x_range = get_range(self.start.x, self.end.x);
        let y_range = get_range(self.start.y, self.end.y);

        // Get the longest of the two ranges, we'll use it to take as many points as we need
        let max_len = x_range.len().max(y_range.len());

        x_range
            .into_iter()
            .cycle()
            .zip(y_range.into_iter().cycle())
            .take(max_len) // Take until we've satisfied the whole range
            .map(|(a, b)| Point { x: a, y: b })
            .collect()
    }
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once(" -> ").unwrap();
        Ok(Line {
            start: start.parse()?,
            end: end.parse()?,
        })
    }
}

fn calculate_overlaps<'a, It>(lines: It) -> usize
where
    It: Iterator<Item = &'a Line>,
{
    lines
        .flat_map(|line| line.points()) // Get all points in a single flat list
        .fold(HashMap::<Point, usize>::new(), |mut map, point| {
            *map.entry(point).or_insert(0) += 1;
            map
        })
        .values() // Get all counts of overlaps
        .filter(|&&value| value >= 2) // Filter all greater or equal to 2
        .count()
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| Line::from_str(line).unwrap())
        .collect()
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &[Line]) -> usize {
    let lines = input.iter().filter(|line| line.is_horizontal_or_vertical());

    calculate_overlaps(lines)
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &[Line]) -> usize {
    calculate_overlaps(input.iter())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_points_for_line_x_up() {
        // Line should produce points 0;0 1;0 2;0
        let line = Line {
            start: Point { x: 0, y: 0 },
            end: Point { x: 2, y: 0 },
        };
        let points = line.points();

        assert_eq!(3, points.len());

        let mut it = points.iter();

        let point_a = it.next().unwrap();
        assert_eq!(0, point_a.x);
        assert_eq!(0, point_a.y);

        let point_b = it.next().unwrap();
        assert_eq!(1, point_b.x);
        assert_eq!(0, point_b.y);

        let point_c = it.next().unwrap();
        assert_eq!(2, point_c.x);
        assert_eq!(0, point_c.y);
    }

    #[test]
    fn test_generate_points_for_line_x_down() {
        // Line should produce points 0;0 1;0 2;0
        let line = Line {
            start: Point { x: 2, y: 0 },
            end: Point { x: 0, y: 0 },
        };
        let points = line.points();

        assert_eq!(3, points.len());

        let mut it = points.iter();

        let point_c = it.next().unwrap();
        assert_eq!(2, point_c.x);
        assert_eq!(0, point_c.y);

        let point_b = it.next().unwrap();
        assert_eq!(1, point_b.x);
        assert_eq!(0, point_b.y);

        let point_a = it.next().unwrap();
        assert_eq!(0, point_a.x);
        assert_eq!(0, point_a.y);
    }

    #[test]
    fn test_part_1() {
        let input = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
"#;

        let lines = input_generator(input);

        let result = solve_part1(&lines);

        assert_eq!(5, result)
    }

    #[test]
    fn test_generate_diagonal() {
        // Line should produce points 0;0 1;0 2;0
        let line = Line {
            start: Point { x: 1, y: 1 },
            end: Point { x: 3, y: 3 },
        };
        let points = line.points();

        assert_eq!(3, points.len());

        let mut it = points.iter();

        let point_a = it.next().unwrap();
        assert_eq!(1, point_a.x);
        assert_eq!(1, point_a.y);

        let point_b = it.next().unwrap();
        assert_eq!(2, point_b.x);
        assert_eq!(2, point_b.y);

        let point_c = it.next().unwrap();
        assert_eq!(3, point_c.x);
        assert_eq!(3, point_c.y);
    }
}
