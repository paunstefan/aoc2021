#![allow(dead_code)]
use itertools::Itertools;

pub fn run_part2(path: &str) -> i64 {
    let numbers = read_data(&std::fs::read_to_string(path).unwrap());

    solve2(numbers)
}

pub fn run_part1(path: &str) -> i64 {
    let numbers = read_data(&std::fs::read_to_string(path).unwrap());

    solve1(numbers)
}

fn read_data(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|n| {
            n.parse::<i64>()
                .unwrap_or_else(|_| panic!("{}", n.to_string()))
        })
        .collect_vec()
}

fn solve1(input: Vec<i64>) -> i64 {
    let sorted = input.iter().sorted().collect_vec();
    let mid = sorted[sorted.len() / 2];
    let res = sorted.iter().map(|n| (**n - mid).abs()).sum::<i64>();

    res
}

fn solve2(input: Vec<i64>) -> i64 {
    let mean = input.iter().sum::<i64>() as f64 / input.len() as f64;

    // The best position is in range [mean(input) - 0.5, mean(input) + 0.5] (from the reddit paper)
    // so I'll check both
    let range = ((mean - 0.5).round() as i64, (mean + 0.5).round() as i64);

    let n_sum = |n| (n * (n + 1)) / 2;
    let calc_all =
        |list: &[i64], mid: i64| list.iter().map(|n| n_sum((*n - mid).abs())).sum::<i64>();

    std::cmp::min(calc_all(&input, range.0), calc_all(&input, range.1))
}

#[test]
fn test_example1() {
    let input = "16,1,2,0,4,2,7,1,2,14";

    let numbers = read_data(input);
    assert_eq!(solve1(numbers), 37);
}

#[test]
fn test_example2() {
    let input = "16,1,2,0,4,2,7,1,2,14";

    let numbers = read_data(input);
    assert_eq!(solve2(numbers), 168);
}
