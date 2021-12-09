#![allow(dead_code)]
use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug)]
struct CaveFloor {
    data: Vec<Vec<i8>>,
    dim: (usize, usize),
}

impl CaveFloor {
    fn above(&self, y: usize, x: usize) -> Option<(usize, usize)> {
        if y > 0 {
            Some((y - 1, x))
        } else {
            None
        }
    }
    fn below(&self, y: usize, x: usize) -> Option<(usize, usize)> {
        if y < self.dim.0 - 1 {
            Some((y + 1, x))
        } else {
            None
        }
    }
    fn left(&self, y: usize, x: usize) -> Option<(usize, usize)> {
        if x > 0 {
            Some((y, x - 1))
        } else {
            None
        }
    }
    fn right(&self, y: usize, x: usize) -> Option<(usize, usize)> {
        if x < self.dim.1 - 1 {
            Some((y, x + 1))
        } else {
            None
        }
    }

    fn low_point(&self, y: usize, x: usize) -> bool {
        let neigh = [
            self.above(y, x),
            self.below(y, x),
            self.right(y, x),
            self.left(y, x),
        ];

        for n in neigh.iter().flatten() {
            let (i, j) = n;
            if self.data[*i][*j] <= self.data[y][x] {
                return false;
            }
        }

        true
    }

    fn get_neighbors(&self, y: usize, x: usize) -> Vec<(usize, usize)> {
        let neigh = vec![
            self.above(y, x),
            self.below(y, x),
            self.right(y, x),
            self.left(y, x),
        ];

        neigh.iter().flatten().copied().collect_vec()
    }
}

impl FromStr for CaveFloor {
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
    let numbers = read_data(&std::fs::read_to_string(path).unwrap());

    solve2(numbers)
}

pub fn run_part1(path: &str) -> i64 {
    let numbers = read_data(&std::fs::read_to_string(path).unwrap());

    solve1(numbers)
}

fn read_data(input: &str) -> CaveFloor {
    CaveFloor::from_str(input).unwrap()
}

fn solve1(input: CaveFloor) -> i64 {
    let mut sum = 0;
    for y in 0..input.dim.0 {
        for x in 0..input.dim.1 {
            if input.low_point(y, x) {
                let s = (input.data[y][x] + 1) as i64;
                sum += s;
            }
        }
    }

    sum
}

fn solve2(mut input: CaveFloor) -> i64 {
    let mut size_vec = Vec::<i64>::new();
    for y in 0..input.dim.0 {
        for x in 0..input.dim.1 {
            if input.data[y][x] != 9 && input.data[y][x] != -1 {
                // BFS
                let mut pool = vec![(y, x)];
                let mut i = 0;
                // Find all pool members
                while let Some(point) = pool.get(i) {
                    let neigh = input.get_neighbors(point.0, point.1);
                    for n in neigh {
                        // Add to pool if not already there and not 9
                        if !pool.contains(&n) && input.data[n.0][n.1] != 9 {
                            pool.push(n);
                        }
                    }
                    i += 1;
                }
                size_vec.push(pool.len() as i64);

                pool.iter().for_each(|p| input.data[p.0][p.1] = -1);
            }
        }
    }
    size_vec.iter().sorted().rev().take(3).product()
}

#[test]
fn test_example1() {
    let input = "2199943210
3987894921
9856789892
8767896789
9899965678";

    let numbers = read_data(input);
    assert_eq!(solve1(numbers), 15);
}

#[test]
fn test_example2() {
    let input = "2199943210
3987894921
9856789892
8767896789
9899965678";

    let numbers = read_data(input);
    assert_eq!(solve2(numbers), 1134);
}
