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

fn read_data(input: &str) -> (Vec<char>, HashMap<String, char>) {
    let (string, mapping) = input.split_once("\n\n").unwrap();
    let string = string.chars().collect_vec();

    let mut char_map = HashMap::new();
    mapping
        .lines()
        .map(|x| {
            let (a, b) = x.split_once(" -> ").unwrap();
            let a = a.to_string();
            let b = b.chars().next().unwrap();
            (a, b)
        })
        .for_each(|x| {
            char_map.insert(x.0, x.1);
        });

    (string, char_map)
}

fn solve_general(input: (Vec<char>, HashMap<String, char>), times: usize) -> i64 {
    let mut pairs: BTreeMap<String, usize> = BTreeMap::new();

    // Add initial pairs
    for key in input.1.keys() {
        pairs.insert(key.to_owned(), 0);
    }
    let mut i = 0;
    while i < input.0.len() - 1 {
        let pair = format!("{}{}", input.0[i], input.0[i + 1]);
        *pairs.get_mut(&pair).unwrap() += 1;
        i += 1;
    }

    for _ in 0..times {
        let mut new_vec: Vec<(String, usize)> = Vec::new();
        for pair in pairs.iter_mut() {
            if *pair.1 != 0 {
                let new_pairs = {
                    let c = input.1.get(pair.0).unwrap();
                    let parts = (
                        pair.0.chars().next().unwrap(),
                        pair.0.chars().nth(1).unwrap(),
                    );

                    (format!("{}{}", parts.0, c), format!("{}{}", c, parts.1))
                };
                new_vec.push((new_pairs.0, *pair.1));
                new_vec.push((new_pairs.1, *pair.1));
                *pair.1 = 0;
            }
        }

        for pair in new_vec {
            *pairs.get_mut(&pair.0).unwrap() += pair.1;
        }
    }

    let mut letters: BTreeMap<char, usize> = BTreeMap::new();
    for pair in &pairs {
        let chrs = (
            pair.0.chars().next().unwrap(),
            pair.0.chars().nth(1).unwrap(),
        );

        *letters.entry(chrs.0).or_insert(0) += pair.1;
        *letters.entry(chrs.1).or_insert(0) += pair.1;
    }

    let mut count = letters
        .iter()
        .map(|a| {
            // Needed because each letter will appear in 2 pairs
            if a.1 % 2 != 0 {
                (*a.0, (a.1 + 1) / 2)
            } else {
                (*a.0, a.1 / 2)
            }
        })
        .collect_vec();
    count.sort_by(|a, b| b.1.cmp(&a.1));

    count[0].1 as i64 - count[count.len() - 1].1 as i64
}

fn solve1(input: (Vec<char>, HashMap<String, char>)) -> i64 {
    solve_general(input, 10)
}

fn solve2(input: (Vec<char>, HashMap<String, char>)) -> i64 {
    solve_general(input, 40)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn test_example1() {
        let input = read_data(INPUT);
        assert_eq!(solve1(input), 1588);
    }
    #[test]
    fn test_example2() {
        let input = read_data(INPUT);
        assert_eq!(solve2(input), 2188189693529);
    }
}
