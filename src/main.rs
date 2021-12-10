use clap::{App, Arg};
use std::time::Instant;

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

macro_rules! aoc_days{
    ($($day:ident),+ $(,)?) => {
        $(mod $day;)*
        pub fn match_day(d: &str) {
            let d = format!("{}{}", "aoc", d);
            match &*d {
                $(stringify!($day) => run_day!($day, &format!("data/{}", stringify!($day))),)*
                _ => eprintln!("Day not valid"),
            }
        }
    };
}

aoc_days!(aoc1, aoc2, aoc3, aoc4, aoc5, aoc6, aoc7, aoc8, aoc9, aoc10,);

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

    let day = matches.value_of("day").unwrap_or("0");

    match_day(day);
}
