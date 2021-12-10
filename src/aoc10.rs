#![allow(dead_code)]
use std::collections::HashMap;

use itertools::Itertools;

pub fn run_part2(path: &str) -> i64 {
    let numbers = read_data(&std::fs::read_to_string(path).unwrap());

    solve2(numbers)
}

pub fn run_part1(path: &str) -> i64 {
    let numbers = read_data(&std::fs::read_to_string(path).unwrap());

    solve1(numbers)
}

fn read_data(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect_vec()).collect_vec()
}

fn solve1(input: Vec<Vec<char>>) -> i64 {
    let mut matching: HashMap<char, (char, i64)> = HashMap::new();
    matching.insert(')', ('(', 3));
    matching.insert(']', ('[', 57));
    matching.insert('}', ('{', 1197));
    matching.insert('>', ('<', 25137));

    let mut sum = 0;

    for line in input {
        let mut stack: Vec<char> = Vec::new();
        for c in line {
            if !matching.contains_key(&c) {
                stack.push(c);
            } else {
                let m = matching[&c];
                if m.0 != stack.pop().unwrap() {
                    sum += m.1;
                    break;
                }
            }
        }
    }

    sum
}

fn solve2(input: Vec<Vec<char>>) -> i64 {
    let mut pairs: HashMap<char, i64> = HashMap::new();
    pairs.insert('(', 1);
    pairs.insert('[', 2);
    pairs.insert('{', 3);
    pairs.insert('<', 4);

    let mut matching: HashMap<char, char> = HashMap::new();
    matching.insert(')', '(');
    matching.insert(']', '[');
    matching.insert('}', '{');
    matching.insert('>', '<');

    let mut scores: Vec<i64> = Vec::new();

    for line in input {
        let mut stack: Vec<char> = Vec::new();
        let mut valid = true;
        let mut sum = 0;
        for c in line {
            if !matching.contains_key(&c) {
                stack.push(c);
            } else {
                let m = matching[&c];
                if m != stack.pop().unwrap() {
                    valid = false;
                    break;
                }
            }
        }
        if valid {
            for c in stack.iter().rev() {
                sum = sum * 5 + pairs[c];
            }
            scores.push(sum);
        }
    }
    scores.sort_unstable();

    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_example1() {
        let chrs = read_data(INPUT);
        assert_eq!(solve1(chrs), 26397);
    }
    #[test]
    fn test_example2() {
        let chrs = read_data(INPUT);
        assert_eq!(solve2(chrs), 288957);
    }
}
