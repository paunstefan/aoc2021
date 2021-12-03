#![allow(dead_code)]
use itertools::Itertools;

// Implemented the solution 2 ways, first one with manipulating the strings directly
// second one with bitwise operations

#[derive(PartialEq)]
enum Elements {
    O2,
    CO2,
}

pub fn run2(path: &str) -> u32 {
    let data = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect_vec();
    let size = data[0].len();
    let data = data
        .iter()
        .map(|l| u16::from_str_radix(l, 2).unwrap())
        .collect_vec();
    solve2_2(&data, size)
}

fn solve1_2(input: &[u16], bit_len: usize) -> u32 {
    let mut gamma = 0u16;

    for i in 0..bit_len {
        let ones = input
            .iter()
            .filter(|x| **x & (1 << (bit_len - i - 1)) != 0)
            .count();

        let common = if ones > (input.len() - ones) { 1 } else { 0 };

        gamma |= common << (bit_len - i - 1);
    }

    let epsilon = !gamma & (0xFFFF >> (16 - bit_len));

    (gamma as u32) * (epsilon as u32) as u32
}

fn solve2_2(input: &[u16], bit_len: usize) -> u32 {
    let o2: Vec<u16> = input.iter().copied().collect();

    // find oxygen generator rating
    let o2 = filter_numbers2(o2, bit_len, Elements::O2);

    let co2: Vec<u16> = input.iter().copied().collect();

    // find CO2 scrubber rating
    let co2 = filter_numbers2(co2, bit_len, Elements::CO2);

    println!("o2: {}, co2: {}", o2, co2);

    o2 * co2
}

fn filter_numbers2(mut numbers: Vec<u16>, bit_len: usize, which: Elements) -> u32 {
    let wanted = if which == Elements::O2 {
        (1, 0)
    } else {
        (0, 1)
    };

    for i in 0..bit_len {
        let ones = numbers
            .iter()
            .filter(|x| **x & (1 << (bit_len - i - 1)) != 0)
            .count();

        let equ = ones == (numbers.len() - ones);
        let chosen = if ones > (numbers.len() - ones) {
            wanted.0
        } else {
            wanted.1
        };

        numbers.retain(|x| *x >> (bit_len - i - 1) & 1 == if equ { wanted.0 } else { chosen });

        if numbers.len() < 2 {
            break;
        }
    }
    numbers[0] as u32
}

pub fn run_part2(path: &str) -> u32 {
    let data = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect_vec();
    solve2(&data)
}

pub fn run_part1(path: &str) -> u32 {
    let data = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect_vec();
    solve1(&data)
}

fn solve1(input: &[String]) -> u32 {
    let bit_length = input[0].len();
    let mut summed = vec![0i32; bit_length];
    for nr in input.iter() {
        for (i, c) in nr.chars().enumerate() {
            match c {
                '0' => summed[i] -= 1,
                '1' => summed[i] += 1,
                _ => panic!("Not binary"),
            }
        }
    }

    let gamma: String = summed
        .iter()
        .map(|n| if *n < 0 { '0' } else { '1' })
        .collect();

    let gamma = u32::from_str_radix(&gamma, 2).unwrap();
    let epsilon = !gamma & (0xFFFFFFFF >> (32 - bit_length));

    gamma * epsilon
}

fn filter_numbers(mut numbers: Vec<&String>, which: Elements) -> u32 {
    let wanted = if which == Elements::O2 {
        ('1', '0')
    } else {
        ('0', '1')
    };

    for i in 0..numbers[0].len() {
        // find most common bit
        let ones = numbers
            .iter()
            .filter(|s| s.chars().nth(i).unwrap() == '1')
            .count();

        let equ = ones == (numbers.len() - ones);
        let larger = if ones > (numbers.len() - ones) {
            wanted.0
        } else {
            wanted.1
        };

        numbers.retain(|s| s.chars().nth(i).unwrap() == if equ { wanted.0 } else { larger });

        if numbers.len() == 1 {
            break;
        }
    }

    u32::from_str_radix(numbers[0], 2).unwrap()
}

fn solve2(input: &[String]) -> u32 {
    let o2: Vec<&String> = input.iter().collect();

    // find oxygen generator rating
    let o2 = filter_numbers(o2, Elements::O2);

    let co2: Vec<&String> = input.iter().collect();

    // find CO2 scrubber rating
    let co2 = filter_numbers(co2, Elements::CO2);

    o2 * co2
}
