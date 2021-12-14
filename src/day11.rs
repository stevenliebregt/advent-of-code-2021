use std::collections::BTreeMap;

type Coordinate = (i32, i32);
type Energy = u32;
type Cave = BTreeMap<Coordinate, Energy>;

fn parse_cave(s: &str) -> Cave {
    (s.lines().enumerate())
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, energy)| ((x as i32, y as i32), energy.to_digit(10).unwrap()))
        })
        .collect()
}

fn get_coordinate_permutations((x, y): Coordinate) -> Vec<Coordinate> {
    vec![
        // Straight
        (x, y - 1), // North
        (x + 1, y), // East
        (x, y + 1), // South
        (x - 1, y), // West
        // Diagonal
        (x + 1, y - 1), // North-east
        (x - 1, y - 1), // North-west
        (x + 1, y + 1), // South-east
        (x - 1, y + 1), // South-west
    ]
}

/// Run a single simulation step for the octopi in the cave, and return the amount of flashes that
/// occurred in this step.
fn run_step(cave: &mut Cave, coordinates: &[Coordinate]) -> usize {
    let mut flashes = 0;
    let mut flash_occurred: bool;

    // First, the energy level of each octopus increases by 1.
    coordinates
        .iter()
        .for_each(|coordinate| *cave.get_mut(coordinate).unwrap() += 1);

    // Then, any octopus with an energy level greater than 9 flashes. This increases the energy
    // level of all adjacent octopuses by 1, including octopuses that are diagonally adjacent.
    // If this causes an octopus to have an energy level greater than 9, it also flashes.
    // This process continues as long as new octopuses keep having their energy level increased
    // beyond 9. (An octopus can only flash at most once per step.)
    flash_occurred = true; // We need to go about at least once

    while flash_occurred {
        flash_occurred = false; // Will be set to true if any octopus flashes in this iteration

        coordinates.iter().for_each(|(x, y)| {
            let energy = cave.get_mut(&(*x, *y)).unwrap();

            if *energy > 9 {
                flashes += 1;
                flash_occurred = true; // We will need to iterate another time to check if there are more octopi at energy level 9+

                // Reset energy to 0 to denote that this octopus has already flashed
                // Since we increase all energy levels as the first step there are no natural zeroes
                *energy = 0;

                // Find all neighbours that have not already flashed and increase them too
                get_coordinate_permutations((*x, *y))
                    .iter()
                    .for_each(|coordinate| {
                        if let Some(neighbour) = cave.get_mut(coordinate) {
                            if *neighbour != 0 {
                                // If it is 0 it has already flashed this iteration
                                *neighbour += 1
                            }
                        }
                    });
            }
        });
    }

    flashes
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &str) -> usize {
    let mut cave: Cave = parse_cave(input);
    let coordinates = &cave.keys().copied().collect::<Vec<_>>();

    let mut flashes = 0;

    for _ in 0..100 {
        flashes += run_step(&mut cave, coordinates);
    }

    flashes
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut cave: Cave = parse_cave(input);
    let coordinates = &cave.keys().copied().collect::<Vec<_>>();

    let mut step = 0;

    loop {
        step += 1;

        run_step(&mut cave, coordinates);

        // Check if all octopi are at energy level 0, which means they all flashed this step
        if cave.values().all(|energy| *energy == 0) {
            break;
        }
    }

    step
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let input = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#;

        assert_eq!(1656, solve_part1(input))
    }

    #[test]
    fn test_example_part2() {
        let input = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#;

        assert_eq!(195, solve_part2(input))
    }
}
