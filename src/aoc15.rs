#![allow(dead_code, unreachable_code)]
use std::{cmp::Ordering, str::FromStr};

use itertools::Itertools;

#[derive(Debug)]
struct Matrix {
    data: Vec<Vec<i8>>,
    dim: (usize, usize),
}

impl Matrix {
    fn get_neighbors(&self, y: i32, x: i32) -> Vec<(usize, usize)> {
        let mut neigh = Vec::new();

        // All neighbors, no diagonals
        for i in (y - 1)..=(y + 1) {
            for j in (x - 1)..=(x + 1) {
                if i >= 0
                    && i < self.dim.0 as i32
                    && j >= 0
                    && j < self.dim.1 as i32
                    && (i == y || j == x)       // remove this to include diagonals
                    && (i, j) != (y, x)
                {
                    neigh.push((i as usize, j as usize));
                }
            }
        }

        neigh
    }

    fn get_cost(&self, pos: (usize, usize)) -> i8 {
        self.data[pos.0][pos.1]
    }
}

impl FromStr for Matrix {
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
    let input = read_data(&std::fs::read_to_string(path).unwrap());

    solve2(input)
}

pub fn run_part1(path: &str) -> i64 {
    let input = read_data(&std::fs::read_to_string(path).unwrap());

    solve1(input)
}

fn read_data(input: &str) -> Matrix {
    Matrix::from_str(input).unwrap()
}

#[derive(Eq, Clone, Debug)]
struct DijkstaCost {
    cost: i32,
    visited: bool,
    pos: (usize, usize),
}

impl Ord for DijkstaCost {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for DijkstaCost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for DijkstaCost {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

fn dijkstra(input: Matrix) -> i64 {
    let end: (usize, usize) = (input.dim.0 - 1, input.dim.1 - 1);

    let mut costs: Vec<Vec<DijkstaCost>> = vec![
        vec![
            DijkstaCost {
                cost: i32::MAX,
                visited: false,
                pos: (0, 0)
            };
            input.dim.1
        ];
        input.dim.0
    ];

    for i in 0..input.data.len() {
        for j in 0..input.data[0].len() {
            costs[i][j].pos = (i, j);
        }
    }
    costs[0][0].cost = 0;

    // Dijksta
    for i in 0..input.dim.0 {
        for j in 0..input.dim.1 {
            let n = costs.iter().flatten().filter(|a| !a.visited).min().unwrap();
            let pos = n.pos;
            costs[pos.0][pos.1].visited = true;

            for neigh in input.get_neighbors(pos.0 as i32, pos.1 as i32) {
                if !costs[neigh.0][neigh.1].visited
                    && (input.get_cost(neigh) as i32 + costs[pos.0][pos.1].cost
                        < costs[neigh.0][neigh.1].cost)
                {
                    costs[neigh.0][neigh.1].cost =
                        input.get_cost(neigh) as i32 + costs[pos.0][pos.1].cost;
                }
            }
        }
    }
    costs[end.0][end.1].cost as i64
}

fn solve1(input: Matrix) -> i64 {
    dijkstra(input)
}

fn solve2(mut input: Matrix) -> i64 {
    let mut new_matrix: Vec<Vec<i8>> = input.data.clone();
    for x in 1..5 {
        for i in 0..input.dim.0 {
            let new_line = new_matrix[i]
                .iter()
                .take(input.dim.0)
                .map(|a| {
                    let n = (*a + x) % 9;
                    if n == 0 {
                        9
                    } else {
                        n
                    }
                })
                .collect_vec();

            new_matrix.push(new_line);
        }
    }

    for x in 1..5 {
        for i in 0..new_matrix.len() {
            let mut new_line = new_matrix[i]
                .iter()
                .take(input.dim.1)
                .map(|a| {
                    let n = (*a + x) % 9;
                    if n == 0 {
                        9
                    } else {
                        n
                    }
                })
                .collect_vec();
            new_matrix[i].append(&mut new_line);
        }
    }

    input.data = new_matrix;
    input.dim.0 *= 5;
    input.dim.1 *= 5;

    dijkstra(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn test_example1() {
        let input = read_data(INPUT);
        assert_eq!(solve1(input), 40);
    }
    #[test]
    fn test_example2() {
        let input = read_data(INPUT);
        assert_eq!(solve2(input), 315);
    }
}
