#![allow(dead_code, unreachable_code)]
use regex::Regex;

type Target = ((i32, i32), (i32, i32));

struct Probe {
    x_pos: i32,
    y_pos: i32,
    x_vel: i32,
    y_vel: i32,
}

impl Probe {
    fn new(x_vel: i32, y_vel: i32) -> Self {
        Self {
            x_pos: 0,
            y_pos: 0,
            x_vel,
            y_vel,
        }
    }

    fn step(&mut self) {
        self.x_pos += self.x_vel;
        self.y_pos += self.y_vel;

        self.x_vel -= self.x_vel.signum();
        self.y_vel -= 1;
    }

    fn is_in_target(&self, tgt: &Target) -> bool {
        self.x_pos >= (tgt.0).0
            && self.x_pos <= (tgt.0).1
            && self.y_pos >= (tgt.1).0
            && self.y_pos <= (tgt.1).1
    }
}

pub fn run_part2(path: &str) -> i64 {
    let input = read_data(&std::fs::read_to_string(path).unwrap());

    solve2(input)
}

pub fn run_part1(path: &str) -> i64 {
    let input = read_data(&std::fs::read_to_string(path).unwrap());
    println!("{:?}", input);
    solve1(input)
}

fn read_data(input: &str) -> Target {
    let re = Regex::new(r".*=(-?\d+)\.\.(-?\d+).*=(-?\d+)\.\.(-?\d+)").unwrap();
    let caps = re.captures(input).unwrap();

    (
        (caps[1].parse().unwrap(), caps[2].parse().unwrap()),
        (caps[3].parse().unwrap(), caps[4].parse().unwrap()),
    )
}

fn solve1(input: Target) -> i64 {
    let mut max_y = i32::MIN;

    for x in 0..1000 {
        for y in 0..1000 {
            let mut probe = Probe::new(x, y);
            let mut temp_max_y = i32::MIN;
            for _ in 0..200 {
                probe.step();
                if probe.y_vel >= 0 {
                    temp_max_y = probe.y_pos;
                }
                if probe.is_in_target(&input) && temp_max_y > max_y {
                    max_y = temp_max_y;
                    break;
                }
            }
        }
    }

    max_y as i64
}

fn solve2(input: Target) -> i64 {
    let mut c = 0;

    for x in 0..1000 {
        for y in -500..1000 {
            let mut probe = Probe::new(x, y);
            for _ in 0..200 {
                probe.step();
                if probe.is_in_target(&input) {
                    c += 1;
                    break;
                }
            }
        }
    }

    c as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn test_example1() {
        let input = read_data(INPUT);
        assert_eq!(solve1(input), 45);
    }
    #[test]
    fn test_example2() {
        let input = read_data(INPUT);
        assert_eq!(solve2(input), 112);
    }
}
