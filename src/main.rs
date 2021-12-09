use clap::{App, Arg};
use std::time::Instant;

mod aoc1;
mod aoc2;
mod aoc3;
mod aoc4;
mod aoc5;
mod aoc6;
mod aoc7;
mod aoc8;
mod aoc9;

macro_rules! run_day {
    ( $x:ident, $input:expr ) => {{
        println!("\n{}:", stringify!($x));
        let start = Instant::now();
        let p1 = $x::run_part1($input);
        let duration = start.elapsed();
        println!("Part 1: {} ({:?})", p1, duration);

        let start = Instant::now();
        let p1 = $x::run_part2($input);
        let duration = start.elapsed();
        println!("Part 2: {} ({:?})", p1, duration);
    }};
}

fn main() {
    let matches = App::new("Advent of Code 2021")
        .version("0.3")
        .author("Stefan Paun")
        .arg(
            Arg::with_name("day")
                .short("d")
                .long("day")
                .help("Select the day")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let day = matches
        .value_of("day")
        .unwrap_or("0")
        .parse::<i32>()
        .unwrap();

    match day {
        1 => run_day!(aoc1, "data/1"),
        2 => run_day!(aoc2, "data/2"),
        3 => run_day!(aoc3, "data/3"),
        4 => run_day!(aoc4, "data/4"),
        5 => run_day!(aoc5, "data/5"),
        6 => run_day!(aoc6, "data/6"),
        7 => run_day!(aoc7, "data/7"),
        8 => run_day!(aoc8, "data/8"),
        9 => run_day!(aoc9, "data/9"),
        _ => eprintln!("Day not valid"),
    }
}
