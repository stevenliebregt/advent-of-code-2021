use std::collections::HashMap;
use std::str::FromStr;

use itertools::Itertools;

type PolymerTemplate = Vec<char>;
type PairInsertionRules = HashMap<(char, char), char>;

struct Input(PolymerTemplate, PairInsertionRules);

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let polymer_template = lines.next().unwrap().chars().collect::<PolymerTemplate>();
        let pair_insertion_rules = lines
            .skip(1)
            .into_iter()
            .map(|line| line.split_once(" -> ").unwrap())
            .map(|(pair, insertion)| {
                let mut pair_chars = pair.chars();

                (
                    (pair_chars.next().unwrap(), pair_chars.next().unwrap()),
                    insertion.chars().next().unwrap(),
                )
            })
            .collect::<PairInsertionRules>();

        Ok(Self(polymer_template, pair_insertion_rules))
    }
}

fn create_polymer(template: PolymerTemplate, rules: PairInsertionRules, steps: usize) -> usize {
    let result = (0..steps).fold(
        template.iter().tuple_windows::<(_, _)>().counts(),
        |acc, _| {
            acc.iter()
                .fold(HashMap::new(), |mut new_acc, (&(a, b), count)| {
                    let insertion = &rules[&(*a, *b)];

                    *new_acc.entry((a, insertion)).or_insert(0) += count;
                    *new_acc.entry((insertion, b)).or_insert(0) += count;

                    new_acc
                })
        },
    );

    // Only count first char of tuple as we're using windows
    let mut counts = result.iter().fold(
        HashMap::<char, usize>::new(),
        |mut acc, ((&a, _), count)| {
            *acc.entry(a).or_insert(0) += count;
            acc
        },
    );

    // Add the last char of the start because the window cuts it off
    *counts.entry(*template.last().unwrap()).or_insert(0) += 1;

    let max = counts.values().max().unwrap();
    let min = counts.values().min().unwrap();

    max - min
}

#[aoc(day14, part1)]
pub fn solve_part1(raw: &str) -> usize {
    let input = raw.parse::<Input>().unwrap();

    create_polymer(input.0, input.1, 10)
}

#[aoc(day14, part2)]
pub fn solve_part2(raw: &str) -> usize {
    let input = raw.parse::<Input>().unwrap();

    create_polymer(input.0, input.1, 40)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let input = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#;

        assert_eq!(1588, solve_part1(input))
    }

    #[test]
    fn test_example_part2() {
        let input = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#;

        assert_eq!(2188189693529, solve_part2(input))
    }
}
