mod aoc1;
mod aoc2;
mod aoc3;
// mod aoc6;
// mod aoc8;
// mod aoc9;

macro_rules! run_day {
    ( $x:ident, $input:expr ) => {
        println!("\n{}:", stringify!($x));
        println!("Part 1: {}", $x::run_part1($input));
        println!("Part 2: {}", $x::run_part2($input));
    };
}

fn main() {
    run_day!(aoc1, "data/1");
    run_day!(aoc2, "data/2");
    run_day!(aoc3, "data/3");
}
