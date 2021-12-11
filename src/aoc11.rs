#![allow(dead_code, unreachable_code)]
use std::{collections::HashSet, str::FromStr};

use itertools::Itertools;
#[derive(Debug)]
struct OctopusMatrix {
    data: Vec<Vec<i8>>,
    dim: (usize, usize),
}

impl OctopusMatrix {
    fn get_neighbors(&self, y: i32, x: i32) -> Vec<(usize, usize)> {
        let mut neigh = Vec::new();

        for i in (y - 1)..=(y + 1) {
            for j in (x - 1)..=(x + 1) {
                if i >= 0
                    && i < self.dim.0 as i32
                    && j >= 0
                    && j < self.dim.1 as i32
                    && (i, j) != (y, x)
                {
                    neigh.push((i as usize, j as usize));
                }
            }
        }

        neigh
    }

    fn flash(&mut self, y: usize, x: usize) -> HashSet<(usize, usize)> {
        let mut flashed: HashSet<(usize, usize)> = HashSet::new();

        flashed.insert((y, x));

        for n in self.get_neighbors(y as i32, x as i32) {
            self.data[n.0][n.1] += 1;
            if self.data[n.0][n.1] == 10 {
                // Recursively flash neighbor
                flashed.extend(self.flash(n.0, n.1));
            }
        }

        flashed
    }
}

impl FromStr for OctopusMatrix {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap() as i8)
                    .collect_vec()
            })
            .collect_vec();

        let dim = (data.len(), data[0].len());

        Ok(Self { data, dim })
    }
}

pub fn run_part2(path: &str) -> i64 {
    let matrix = read_data(&std::fs::read_to_string(path).unwrap());

    solve2(matrix)
}

pub fn run_part1(path: &str) -> i64 {
    let matrix = read_data(&std::fs::read_to_string(path).unwrap());

    solve1(matrix)
}

fn read_data(input: &str) -> OctopusMatrix {
    OctopusMatrix::from_str(input).unwrap()
}

fn solve1(mut input: OctopusMatrix) -> i64 {
    let mut sum = 0;

    for _ in 0..100 {
        let mut flashed: HashSet<(usize, usize)> = HashSet::new();
        for i in 0..input.dim.0 {
            for j in 0..input.dim.1 {
                input.data[i][j] += 1;
                if input.data[i][j] == 10 {
                    let new_flashes = input.flash(i, j);
                    flashed.extend(new_flashes);
                }
            }
        }

        flashed.iter().for_each(|a| input.data[a.0][a.1] = 0);

        sum += flashed.len() as i64;
    }

    sum
}

fn solve2(mut input: OctopusMatrix) -> i64 {
    let mut step: i64 = 0;

    loop {
        let mut flashed: HashSet<(usize, usize)> = HashSet::new();
        for i in 0..input.dim.0 {
            for j in 0..input.dim.1 {
                input.data[i][j] += 1;
                if input.data[i][j] == 10 && !flashed.contains(&(i, j)) {
                    let new_flashes = input.flash(i, j);
                    flashed.extend(new_flashes);
                }
            }
        }

        flashed.iter().for_each(|a| input.data[a.0][a.1] = 0);

        step += 1;
        if flashed.len() == (input.dim.0 * input.dim.1) {
            return step;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test_example1() {
        let matrix = read_data(INPUT);
        assert_eq!(solve1(matrix), 1656);
    }
    #[test]
    fn test_example2() {
        let matrix = read_data(INPUT);
        assert_eq!(solve2(matrix), 195);
    }
}
