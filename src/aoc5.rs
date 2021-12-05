#![allow(dead_code)]
use std::str::FromStr;

use itertools::Itertools;

type Point = (i32, i32);

#[derive(Debug, Clone)]
struct Line {
    points: Vec<Point>,
}

impl Line {
    fn is_straight(&self) -> bool {
        self.points[0].0 == self.points[1].0 || self.points[0].1 == self.points[1].1
    }

    fn all_points(&self) -> Vec<Point> {
        let mut points = Vec::new();
        let (mut p1, mut p2) = (self.points[0], self.points[1]);
        if self.is_straight() {
            // Just simple iteration
            // Vertical
            if p1.0 == p2.0 {
                if p1.1 > p2.1 {
                    std::mem::swap(&mut p1, &mut p2);
                }
                for i in (p1.1)..(p2.1 + 1) {
                    points.push((p1.0, i))
                }
            }
            // Horizontal
            else {
                if p1.0 > p2.0 {
                    std::mem::swap(&mut p1, &mut p2);
                }
                for i in (p1.0)..(p2.0 + 1) {
                    points.push((i, p1.1))
                }
            }
        } else {
            // Line equation
            // m = (y1 - y2) / (x1-x2);
            // c = y1 - x1 * m;
            // Then, for any given x:
            // y = mx + c;
            let m = (p1.1 - p2.1) / (p1.0 - p2.0);
            let c = p1.1 - p1.0 * m;

            if p1.0 > p2.0 {
                std::mem::swap(&mut p1, &mut p2);
            }

            for i in (p1.0)..(p2.0 + 1) {
                points.push((i, m * i + c))
            }
        }
        points
    }
}

impl FromStr for Line {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points = s
            .split(" -> ")
            .map(|p| {
                let a = p
                    .split(',')
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect_vec();
                (a[0], a[1])
            })
            .collect_vec();

        Ok(Self { points })
    }
}

pub fn run_part2(path: &str) -> i32 {
    let data = read_data(path);
    general_solve(&data)
}

pub fn run_part1(path: &str) -> i32 {
    let data = read_data(path);
    let data = data
        .iter()
        .filter(|l| l.is_straight())
        .cloned()
        .collect_vec();

    general_solve(&data)
}

fn read_data(path: &str) -> Vec<Line> {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|s| s.parse().unwrap())
        .collect_vec()
}

fn general_solve(input: &[Line]) -> i32 {
    let mut floor = vec![vec![0u8; 1000]; 1000];
    for line in input {
        for point in line.all_points() {
            floor[point.0 as usize][point.1 as usize] += 1;
        }
    }
    let mut c = 0;
    for line in floor {
        for place in line {
            if place > 1 {
                c += 1;
            }
        }
    }
    c
}
