#![allow(dead_code)]
use itertools::Itertools;

pub fn run_part2(path: &str) -> u64 {
    let numbers = read_data(path);

    solve2(numbers)
}

pub fn run_part1(path: &str) -> u64 {
    let numbers = read_data(path);

    solve1(numbers)
}

fn read_data(path: &str) -> Vec<u8> {
    std::fs::read_to_string(path)
        .unwrap()
        .split(',')
        .map(|n| {
            n.parse::<u8>()
                .unwrap_or_else(|_| panic!("{}", n.to_string()))
        })
        .collect_vec()
}

fn solve_days(input: Vec<u8>, days: usize) -> u64 {
    let mut gen = [0u64; 9];

    for n in input.iter() {
        gen[*n as usize] += 1;
    }

    for _ in 0..days {
        let mut new_ones = 0;

        for i in 0..9 {
            if i == 0 {
                new_ones = gen[i];
            } else {
                gen[i - 1] = gen[i];
            }
            gen[i] = 0;
        }
        gen[6] += new_ones;
        gen[8] = new_ones;
    }

    gen.iter().sum()
}

fn solve1(input: Vec<u8>) -> u64 {
    solve_days(input, 80)
}

fn solve2(input: Vec<u8>) -> u64 {
    solve_days(input, 256)
}
