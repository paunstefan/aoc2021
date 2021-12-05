use clap::{App, Arg};

mod aoc1;
mod aoc2;
mod aoc3;
mod aoc4;
mod aoc5;

macro_rules! run_day {
    ( $x:ident, $input:expr ) => {{
        println!("\n{}:", stringify!($x));
        println!("Part 1: {}", $x::run_part1($input));
        println!("Part 2: {}", $x::run_part2($input));
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
        _ => eprintln!("Day not valid"),
    }
}
