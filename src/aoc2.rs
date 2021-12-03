#![allow(dead_code)]
use itertools::Itertools;

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

pub fn run_part1(path: &str) -> i32 {
    let data = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|c| parse_command(c))
        .collect_vec();
    solve1(&data)
}

pub fn run_part2(path: &str) -> i32 {
    let data = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|c| parse_command(c))
        .collect_vec();
    solve2(&data)
}

fn parse_command(command: &str) -> Command {
    let words = command.split_whitespace().collect_vec();
    match words[0] {
        "forward" => Command::Forward(words[1].parse::<i32>().unwrap()),
        "down" => Command::Down(words[1].parse::<i32>().unwrap()),
        "up" => Command::Up(words[1].parse::<i32>().unwrap()),
        _ => unimplemented!(),
    }
}

fn solve1(input: &[Command]) -> i32 {
    let mut pos: (i32, i32) = (0, 0);
    for command in input {
        match command {
            Command::Forward(n) => pos.0 += n,
            Command::Down(n) => pos.1 += n,
            Command::Up(n) => pos.1 -= n,
        }
    }

    pos.0 * pos.1
}

fn solve2(input: &[Command]) -> i32 {
    let mut pos: (i32, i32, i32) = (0, 0, 0);
    for command in input {
        match command {
            Command::Forward(n) => {
                pos.0 += n;
                pos.1 += n * pos.2;
            }
            Command::Down(n) => pos.2 += n,
            Command::Up(n) => pos.2 -= n,
        }
    }

    pos.0 * pos.1
}
