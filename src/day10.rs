#[aoc(day10, part1)]
pub fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| -> Option<usize> {
            let mut stack: Vec<char> = Vec::with_capacity(line.len());

            for c in line.chars() {
                match c {
                    '(' => stack.push(')'),
                    '[' => stack.push(']'),
                    '{' => stack.push('}'),
                    '<' => stack.push('>'),
                    _ => {
                        if stack.pop() != Some(c) {
                            return Some(match c {
                                ')' => 3,
                                ']' => 57,
                                '}' => 1197,
                                '>' => 25137,
                                _ => unreachable!(),
                            });
                        }
                    }
                }
            }

            None
        })
        .sum()
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &str) -> usize {
    let mut scores = input
        .lines()
        .filter_map(|line| -> Option<Vec<char>> {
            let mut stack: Vec<char> = Vec::with_capacity(line.len());

            for c in line.chars() {
                match c {
                    '(' => stack.push(')'),
                    '[' => stack.push(']'),
                    '{' => stack.push('}'),
                    '<' => stack.push('>'),
                    _ => {
                        if stack.pop() != Some(c) {
                            return None;
                        }
                    }
                }
            }

            Some(stack)
        })
        .map(|chars| {
            chars.iter().rev().fold(0, |accumulator, c| {
                accumulator * 5
                    + match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => unreachable!(),
                    }
            })
        })
        .collect::<Vec<usize>>();

    scores.sort();

    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#;

        assert_eq!(26397, solve_part1(input));
    }

    #[test]
    fn test_part2() {
        let input = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#;

        assert_eq!(288957, solve_part2(input));
    }
}
