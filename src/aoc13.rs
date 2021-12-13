#![allow(dead_code, unreachable_code)]
use std::{fmt, str::FromStr};

use itertools::Itertools;

#[derive(Debug)]
struct Sheet {
    matrix: Vec<Vec<u8>>,
    dim: (usize, usize),
}

impl Sheet {}

impl FromStr for Sheet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points: Vec<(usize, usize)> = s
            .lines()
            .map(|l| l.split_once(',').unwrap())
            .map(|a| (a.1.parse().unwrap(), a.0.parse().unwrap()))
            .collect_vec();

        let max: (usize, usize) = (
            points.iter().map(|a| a.0).max().unwrap() + 1,
            points.iter().map(|a| a.1).max().unwrap() + 1,
        );

        let mut matrix: Vec<Vec<u8>> = vec![vec![0; max.1]; max.0];

        for point in points {
            matrix[point.0][point.1] = 1;
        }

        Ok(Self {
            matrix,
            dim: (max.0, max.1),
        })
    }
}

impl fmt::Display for Sheet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.dim.0 {
            for j in 0..self.dim.1 {
                write!(f, "{}", if self.matrix[i][j] == 1 { '█' } else { ' ' }).unwrap();
            }
            writeln!(f).unwrap();
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
enum Fold {
    X(usize),
    Y(usize),
}

pub fn run_part2(path: &str) -> i64 {
    let input = read_data(&std::fs::read_to_string(path).unwrap());

    solve2(input)
}

pub fn run_part1(path: &str) -> i64 {
    let input = read_data(&std::fs::read_to_string(path).unwrap());
    solve1(input)
}

fn read_data(input: &str) -> (Sheet, Vec<Fold>) {
    let (points, folds) = input.split_once("\n\n").unwrap();
    let sheet = Sheet::from_str(points).unwrap();
    let folds = folds
        .lines()
        .map(|a| a.split_ascii_whitespace().nth(2).unwrap())
        .map(|a| {
            let (coord, line) = a.split_once("=").unwrap();
            match coord {
                "x" => Fold::X(line.parse().unwrap()),
                "y" => Fold::Y(line.parse().unwrap()),
                _ => panic!("Fold not valid"),
            }
        })
        .collect_vec();

    (sheet, folds)
}

fn solve_general(mut input: (Sheet, Vec<Fold>)) -> Sheet {
    let matrix = &mut input.0.matrix;
    let dims = &mut input.0.dim;
    for fold in input.1 {
        match fold {
            Fold::X(x) => {
                for i in 0..dims.0 {
                    for j in (x + 1)..dims.1 {
                        if matrix[i][j] == 1 {
                            matrix[i][x - (j - x)] = matrix[i][j];
                        }
                        matrix[i][j] = 0;
                    }
                }
                dims.1 /= 2;
            }
            Fold::Y(y) => {
                for i in (y + 1)..dims.0 {
                    for j in 0..dims.1 {
                        if matrix[i][j] == 1 {
                            matrix[y - (i - y)][j] = matrix[i][j];
                        }
                        matrix[i][j] = 0;
                    }
                }
                dims.0 /= 2;
            }
        }
    }
    input.0
}

fn solve1(input: (Sheet, Vec<Fold>)) -> i64 {
    let input = (input.0, vec![input.1[0]]);
    let sheet = solve_general(input);
    sheet.matrix.iter().flatten().filter(|a| **a != 0).count() as i64
}

fn solve2(input: (Sheet, Vec<Fold>)) -> i64 {
    let sheet = solve_general(input);
    println!("{}", sheet);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn test_example1() {
        let input = read_data(INPUT);
        assert_eq!(solve1(input), 17);
    }
    #[test]
    fn test_example2() {
        let input = read_data(INPUT);
        assert_eq!(solve2(input), 0);
    }
}
