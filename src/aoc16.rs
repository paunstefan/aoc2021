#![allow(dead_code, unreachable_code)]
use std::collections::{BTreeMap, HashMap};

use itertools::Itertools;

pub fn run_part2(path: &str) -> i64 {
    let input = read_data(&std::fs::read_to_string(path).unwrap());

    solve2(input)
}

pub fn run_part1(path: &str) -> i64 {
    let input = read_data(&std::fs::read_to_string(path).unwrap());

    solve1(input)
}

fn read_data(input: &str) -> Vec<u8> {
    input
        .chars()
        .map(|c| c.to_digit(16).unwrap() as u8)
        .collect_vec()
}

fn solve1(input: Vec<u8>) -> i64 {
    0
}

fn solve2(input: Vec<u8>) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "8A004A801A8002F478";

    #[test]
    fn test_example1() {
        let input = read_data(INPUT);
        assert_eq!(solve1(input), 16);
    }
    #[test]
    fn test_example2() {
        let input = read_data(INPUT);
        assert_eq!(solve2(input), 2188189693529);
    }
}
