#![allow(dead_code)]
use itertools::Itertools;

pub fn run_part2(path: &str) -> i32 {
    let data = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|n| n.parse::<i32>().unwrap())
        .collect_vec();
    solve2(&data)
}

pub fn run_part1(path: &str) -> i32 {
    let data = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|n| n.parse::<i32>().unwrap())
        .collect_vec();
    solve1(&data)
}

fn solve1(input: &[i32]) -> i32 {
    let mut c = 0;

    for i in 1..input.len() {
        if input[i] > input[i - 1] {
            c += 1;
        }
    }

    c
}

fn solve2(input: &[i32]) -> i32 {
    let mut c = 0;

    // for i in 1..(input.len() - 2) {
    //     if window_sum(input, i) > window_sum(input, i - 1) {
    //         c += 1;
    //     }
    // }

    let windows = input
        .iter()
        .tuple_windows::<(_, _, _)>()
        .map(|w| w.0 + w.1 + w.2)
        .collect_vec();

    for i in 1..windows.len() {
        if windows[i] > windows[i - 1] {
            c += 1;
        }
    }

    c
}

// fn window_sum(data: &[i32], index: usize) -> i32 {
//     data[index..index + 3].iter().sum()
// }
