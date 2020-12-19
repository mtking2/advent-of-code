use aoc::day1;
use aoc::day2;
use aoc::util;
use aoc::Result;

fn main() -> Result<()> {
    println!("Hello, Advent of Code!");

    println!("\nDay 1:");
    let d1p1 = day1::part1(util::input(1).flat_map(|line| line.parse()))?;
    println!("{}", d1p1);

    let d1p2 = day1::part2(util::input(1).flat_map(|line| line.parse()))?;
    println!("{}", d1p2);

    println!("\nDay 2:");
    day2::run(util::input(2))?;

    Ok(())
}
