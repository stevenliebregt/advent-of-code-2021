use std::num::ParseIntError;
use std::str::FromStr;

// #[derive(Clone, PartialEq, Debug)]
// pub struct LanternFish {
//     internal_timer: usize,
// }
//
// impl LanternFish {
//     pub fn tick(&mut self) {
//         self.internal_timer -= 1
//     }
//
//     pub fn is_ready_to_birth(&self) -> bool {
//         self.internal_timer == 0
//     }
//
//     pub fn reset(&mut self) {
//         self.internal_timer = 7; // 7 instead of 6 so we can immediately tick down again
//     }
// }
//
// impl Default for LanternFish {
//     fn default() -> Self {
//         Self {
//             internal_timer: 8
//         }
//     }
// }
//
// impl FromStr for LanternFish {
//     type Err = ParseIntError;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         Ok(Self {
//             internal_timer: s.parse()?
//         })
//     }
// }

pub fn simulate_lanternfish_school(school: &mut Vec<usize>, days: usize) -> usize {
    let mut queue: usize = 0;

    for _ in 0..days {
        for fish in school.iter_mut() {
            if *fish == 0 {
                queue += 1;
                *fish = 7
            }

            *fish -= 1
        }

        // Spawn new lanternfish and reset queue
        for _ in 0..queue {
            school.push(8);
        }

        queue = 0;

        // println!("After {:>2} {:<4}: {}", day + 1, if day == 0 { "day" } else { "days" }, school.iter()
        //     .map(|f| f.to_string())
        //     .collect::<Vec<String>>()
        //     .join(","));
    }

    school.len()
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input.split(',').map(|x| x.parse().unwrap()).collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[usize]) -> usize {
    let mut school = Vec::from(input);

    simulate_lanternfish_school(&mut school, 80)
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[usize]) -> usize {
    let mut school = Vec::from(input);

    simulate_lanternfish_school(&mut school, 256)
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
}