#![allow(dead_code, non_snake_case)]
use std::collections::HashSet;

use itertools::Itertools;

type Digit = HashSet<char>;

pub fn run_part2(path: &str) -> i64 {
    let numbers = read_data2(&std::fs::read_to_string(path).unwrap());

    solve2(numbers)
}

pub fn run_part1(path: &str) -> i64 {
    let numbers = read_data1(&std::fs::read_to_string(path).unwrap());

    solve1(numbers)
}

fn read_data1(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|a| a.split(" | ").nth(1).unwrap().split_ascii_whitespace())
        .flatten()
        .map(|s| s.to_string())
        .collect_vec()
}

fn read_data2(input: &str) -> Vec<(Vec<Digit>, Vec<Digit>)> {
    input
        .lines()
        .map(|l| l.split_once(" | ").unwrap())
        .map(|elems| {
            let nrs = (elems.1)
                .split_ascii_whitespace()
                .map(|n| n.chars().collect::<HashSet<_>>())
                .collect_vec();

            let signals = (elems.0)
                .split_ascii_whitespace()
                .map(|n| n.chars().collect::<HashSet<_>>())
                .collect_vec();

            (signals, nrs)
        })
        .collect_vec()
}

fn solve1(input: Vec<String>) -> i64 {
    input
        .iter()
        .map(|a| a.len())
        .filter(|a| [2, 3, 4, 7].contains(a))
        .count() as i64
}

fn solve2(input: Vec<(Vec<Digit>, Vec<Digit>)>) -> i64 {
    let mut result = 0;
    for i in input {
        let (mut signals, nrs) = i;
        let mut digits: Vec<Digit> = vec![".".chars().collect(); 10];
        digits[1] = signals.iter().find(|a| a.len() == 2).unwrap().clone();
        digits[7] = signals.iter().find(|a| a.len() == 3).unwrap().clone();
        digits[4] = signals.iter().find(|a| a.len() == 4).unwrap().clone();
        digits[8] = signals.iter().find(|a| a.len() == 7).unwrap().clone();
        signals
            .retain(|x| *x != digits[1] && *x != digits[7] && *x != digits[4] && *x != digits[8]);

        // Now for the rest
        let almost9: Digit = digits[4].union(&digits[7]).copied().collect();
        let L: Digit = digits[4].difference(&digits[1]).copied().collect();

        digits[9] = signals
            .iter()
            .find(|a| a.is_superset(&almost9) && a != &&digits[8])
            .unwrap()
            .clone();
        signals.retain(|x| *x != digits[9]);

        digits[3] = signals
            .iter()
            .find(|a| a.is_superset(&digits[1]) && a.is_subset(&digits[9]))
            .unwrap()
            .clone();
        signals.retain(|x| *x != digits[3]);

        let middle: Digit = digits[3].intersection(&L).copied().collect();

        digits[0] = digits[8].difference(&middle).copied().collect();
        signals.retain(|x| *x != digits[0]);

        digits[2] = signals.iter().find(|a| !a.is_superset(&L)).unwrap().clone();
        signals.retain(|x| *x != digits[2]);

        let lower_left: Digit = digits[2].difference(&digits[3]).copied().collect();

        digits[6] = signals
            .iter()
            .find(|a| a.is_superset(&lower_left))
            .unwrap()
            .clone();
        signals.retain(|x| *x != digits[6]);

        digits[5] = signals[0].clone();

        // now decode output
        let mut output_nr = String::new();
        for c in nrs {
            let n = digits.iter().position(|x| *x == c).unwrap();
            output_nr += &n.to_string();
        }
        result += output_nr.parse::<i64>().unwrap();
    }
    result
}

#[test]
fn test_example1() {
    let input =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    let data = read_data1(input);
    assert_eq!(solve1(data), 26);
}

#[test]
fn test_example2() {
    let input =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    let data = read_data2(input);
    assert_eq!(solve2(data), 61229);
}
