use std::collections::{HashMap, HashSet};

type Cave = String;
type CaveSystem = HashMap<Cave, Vec<Cave>>;

fn count_number_of_paths<'a>(
    cave_system: &'a CaveSystem,
    current_node: &'a str,
    visited: &mut HashSet<&'a str>,
    mut visited_twice: Option<&'a str>,
) -> usize {
    if current_node == "end" {
        return 1;
    }

    let is_small = current_node.chars().all(|char| char.is_lowercase());
    let is_visited = visited.insert(current_node);

    // Insert will fail if we visited it before, then we check if we have already seen something twice
    // if that is the case, we stop, otherwise we'll allow it once.
    if is_small && !is_visited {
        if visited_twice.is_some() || current_node == "start" {
            return 0;
        }

        // Mark as seen twice
        visited_twice = Some(current_node);
    }

    let count = cave_system
        .get(current_node)
        .unwrap()
        .iter()
        .map(|next_node| count_number_of_paths(cave_system, next_node, visited, visited_twice))
        .sum();

    if visited_twice.unwrap_or("") != current_node {
        visited.remove(current_node);
    }

    count
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> CaveSystem {
    input.lines().fold(CaveSystem::new(), |mut graph, line| {
        let (a, b) = line.split_once('-').unwrap();

        graph
            .entry(a.to_string())
            .or_insert_with(Vec::new)
            .push(b.to_string());
        graph
            .entry(b.to_string())
            .or_insert_with(Vec::new)
            .push(a.to_string());

        graph
    })
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &CaveSystem) -> usize {
    count_number_of_paths(input, "start", &mut HashSet::new(), Some(""))
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &CaveSystem) -> usize {
    count_number_of_paths(input, "start", &mut HashSet::new(), None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_part1() {
        let input = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#;
        let cave_system = input_generator(input);

        assert_eq!(10, solve_part1(&cave_system))
    }

    #[test]
    fn test_medium_part1() {
        let input = r#"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"#;
        let cave_system = input_generator(input);

        assert_eq!(19, solve_part1(&cave_system))
    }

    #[test]
    fn test_large_part1() {
        let input = r#"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"#;
        let cave_system = input_generator(input);

        assert_eq!(226, solve_part1(&cave_system))
    }

    #[test]
    fn test_small_part2() {
        let input = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#;
        let cave_system = input_generator(input);

        assert_eq!(36, solve_part2(&cave_system))
    }

    #[test]
    fn test_medium_part2() {
        let input = r#"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"#;
        let cave_system = input_generator(input);

        assert_eq!(103, solve_part2(&cave_system))
    }

    #[test]
    fn test_large_part2() {
        let input = r#"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"#;
        let cave_system = input_generator(input);

        assert_eq!(3509, solve_part2(&cave_system))
    }
}
