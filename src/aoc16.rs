#![allow(dead_code, unreachable_code)]
use itertools::Itertools;

#[derive(Debug)]
struct PReader {
    bits: Vec<u8>,
    pos: usize,
}

impl PReader {
    fn read_u8(&mut self, len: usize) -> u8 {
        let mut res = 0u8;
        for _ in 0..len {
            res = (res << 1) | self.bits[self.pos];
            self.pos += 1;
        }
        res
    }

    fn read_u16(&mut self, len: usize) -> u16 {
        let mut res = 0u16;
        for _ in 0..len {
            res = (res << 1) | self.bits[self.pos] as u16;
            self.pos += 1;
        }
        res
    }

    fn read_bit(&mut self) -> u8 {
        let ret = self.bits[self.pos];
        self.pos += 1;
        ret
    }
}

#[derive(Debug)]
struct PHeader {
    version: u8,
    typ: u8,
}

impl PHeader {
    fn parse(input: &mut PReader) -> Self {
        let version = input.read_u8(3);
        let typ = input.read_u8(3);

        Self { version, typ }
    }
}

fn parse_packets(packets: &mut PReader) -> (u64, u64) {
    let header = PHeader::parse(packets);
    if header.typ == 4 {
        let mut val: u64 = 0;
        loop {
            let stop_bit = packets.read_bit();
            val = (val << 4) | packets.read_u8(4) as u64;
            if stop_bit == 0 {
                break;
            }
        }
        return (header.version as u64, val);
    }
    let mut ret = (header.version as u64, 0);
    let len_type = packets.read_bit();

    let mut vals = Vec::new();

    if len_type == 1 {
        let p_len = packets.read_u16(11);
        for _ in 0..p_len {
            let res = parse_packets(packets);
            ret.0 += res.0;
            vals.push(res.1);
        }
    } else {
        let b_len = packets.read_u16(15);
        let current_pos = packets.pos;

        while packets.pos - current_pos != b_len as usize {
            let res = parse_packets(packets);
            ret.0 += res.0;
            vals.push(res.1);
        }
    }
    ret.1 = operate(&vals, header.typ);

    ret
}

fn operate(vals: &Vec<u64>, typ: u8) -> u64 {
    match typ {
        0 => vals.iter().sum(),
        1 => vals.iter().product(),
        2 => *vals.iter().min().unwrap(),
        3 => *vals.iter().max().unwrap(),
        5 => (vals[0] > vals[1]) as u64,
        6 => (vals[0] < vals[1]) as u64,
        7 => (vals[0] == vals[1]) as u64,
        _ => panic!("Operation not supported"),
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

fn read_data(input: &str) -> PReader {
    let bits = input
        .chars()
        .map(|c| c.to_digit(16).unwrap() as u8)
        .tuples::<(_, _)>()
        .map(|a| a.0 << 4 | a.1)
        .map(|a| {
            let mut v = Vec::new();
            for i in (0..8).rev() {
                v.push((a >> i) & 1);
            }
            v
        })
        .flatten()
        .collect_vec();

    PReader { bits, pos: 0 }
}

fn solve1(mut input: PReader) -> i64 {
    let v_sum = parse_packets(&mut input);

    v_sum.0 as i64
}

fn solve2(mut input: PReader) -> i64 {
    let ret = parse_packets(&mut input);
    ret.1 as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "9C0141080250320F1802104A08";

    #[test]
    fn test_example1() {
        let input = read_data(INPUT);
        assert_eq!(solve1(input), 20);
    }
    #[test]
    fn test_example2() {
        let input = read_data(INPUT);
        assert_eq!(solve2(input), 1);
    }
}
