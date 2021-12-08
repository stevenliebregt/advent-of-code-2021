use std::collections::BTreeSet;

/// Number segment usage:
/// 0 uses 6 segments
/// 1 uses 2 segments
/// 2 uses 5 segments
/// 3 uses 5 segments
/// 4 uses 4 segments
/// 5 uses 5 segments
/// 6 uses 6 segments
/// 7 uses 3 segments
/// 8 uses 7 segments
/// 9 uses 6 segments
#[aoc(day8, part1)]
pub fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split(" | ")
                .nth(1)
                .unwrap()
                .split(' ')
                .map(|output| output.len())
                .collect::<Vec<usize>>()
        })
        .flatten()
        .filter(|count| matches!(count, 2 | 3 | 4 | 7))
        .count()
}

fn get_vec_set_char(input: &str) -> Vec<BTreeSet<char>> {
    input
        .split_whitespace()
        .map(|i| i.chars().collect())
        .collect()
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.split_once(" | ").unwrap())
        .map(|(pattern, output)| {
            let patterns: Vec<BTreeSet<char>> = get_vec_set_char(pattern);
            let outputs: Vec<BTreeSet<char>> = get_vec_set_char(output);

            // Create a map of digits
            let mut digits: Vec<BTreeSet<char>> = vec![BTreeSet::new(); 10];

            // Find of size 2 (1)
            if let Some(digit_1) = patterns.iter().find(|p| p.len() == 2) {
                digits[1] = digit_1.clone()
            }

            // Find of size 3 (7)
            if let Some(digit_7) = patterns.iter().find(|p| p.len() == 3) {
                digits[7] = digit_7.clone()
            }

            // Find of size 4 (4)
            if let Some(digit_4) = patterns.iter().find(|p| p.len() == 4) {
                digits[4] = digit_4.clone()
            }

            // Find of size 7 (8)
            if let Some(digit_8) = patterns.iter().find(|p| p.len() == 7) {
                digits[8] = digit_8.clone()
            }

            // Now that we have 4 we can decode 9
            if let Some(digit_9) = patterns
                .iter()
                .filter(|p| p.len() == 6)
                .find(|p| p.is_superset(&digits[4]))
            {
                digits[9] = digit_9.clone()
            }

            // Now that we have 1 and 9 we can decode 0
            if let Some(digit_0) = patterns
                .iter()
                .filter(|p| p.len() == 6)
                .find(|p| p.is_superset(&digits[1]) && **p != digits[9])
            {
                digits[0] = digit_0.clone()
            }

            // Now that we have 0 and 9 we can decode 6
            if let Some(digit_6) = patterns
                .iter()
                .filter(|p| p.len() == 6)
                .find(|p| **p != digits[0] && **p != digits[9])
            {
                digits[6] = digit_6.clone()
            }

            // Now that we have 1 and 7 we can decode 3
            if let Some(digit_3) = patterns
                .iter()
                .filter(|p| p.len() == 5)
                .find(|p| p.is_superset(&digits[1]) && p.is_superset(&digits[7]))
            {
                digits[3] = digit_3.clone()
            }

            // Now that we have 6 we can decode 5 as it is the only subset of it
            if let Some(digit_5) = patterns
                .iter()
                .filter(|p| p.len() == 5)
                .find(|p| p.is_subset(&digits[6]))
            {
                digits[5] = digit_5.clone()
            }

            // Now that we have the rest we can decode 2
            if let Some(digit_2) = patterns
                .iter()
                .filter(|p| p.len() == 5)
                .find(|p| **p != digits[3] && **p != digits[5])
            {
                digits[2] = digit_2.clone()
            }

            outputs
                .iter()
                .map(|output_value| {
                    digits
                        .iter()
                        .enumerate()
                        .find(|(_, value)| &output_value == value)
                        .unwrap()
                        .0
                })
                .fold(0, |acc, dig| acc * 10 + dig as usize)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let input = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;

        assert_eq!(26, solve_part1(input))
    }

    #[test]
    fn test_example_part2() {
        let input = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;

        assert_eq!(61229, solve_part2(input))
    }
}
