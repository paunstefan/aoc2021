#![allow(dead_code)]
use std::collections::HashMap;

pub fn run_part2(path: &str) -> i64 {
    let graph = read_data(&std::fs::read_to_string(path).unwrap());

    solve2(graph)
}

pub fn run_part1(path: &str) -> i64 {
    let graph = read_data(&std::fs::read_to_string(path).unwrap());

    solve1(graph)
}

fn read_data(input: &str) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for v in input.lines() {
        let (a, b) = v.split_once("-").unwrap();
        graph
            .entry(a.to_string())
            .or_insert_with(Vec::<String>::new)
            .push(b.to_string());
        graph
            .entry(b.to_string())
            .or_insert_with(Vec::<String>::new)
            .push(a.to_string());
    }

    graph
}

fn is_large(cave: &str) -> bool {
    if cave != "start" && cave != "end" && cave.chars().any(|c| c.is_lowercase()) {
        return false;
    }

    true
}

fn create_paths<'a>(
    input: &'a HashMap<String, Vec<String>>,
    start: &'a String,
    path: &mut Vec<&'a String>,
    visited_twice: bool,
) -> i64 {
    path.push(start);

    let mut sum = 0;

    for node in input.get(start).unwrap() {
        if node == "start" {
            continue;
        } else if node == "end" {
            sum += 1;
        } else if is_large(node) {
            sum += create_paths(input, node, path, visited_twice);
        } else if !visited_twice || !path.contains(&node) {
            sum += create_paths(input, node, path, visited_twice || path.contains(&node));
        }
    }
    path.pop();

    sum
}

fn solve1(input: HashMap<String, Vec<String>>) -> i64 {
    create_paths(&input, &"start".to_owned(), &mut vec![], true)
}

fn solve2(input: HashMap<String, Vec<String>>) -> i64 {
    create_paths(&input, &"start".to_owned(), &mut vec![], false)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end
";

    #[test]
    fn test_example1() {
        let graph = read_data(INPUT);
        assert_eq!(solve1(graph), 10);
    }
    #[test]
    fn test_example2() {
        let graph = read_data(INPUT);
        assert_eq!(solve2(graph), 36);
    }
}
