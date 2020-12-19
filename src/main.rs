use aoc::day1;
use aoc::util;
use aoc::Result;

fn main() -> Result<()> {
    println!("Hello, Advent of Code!\n");

    let d1p1 = day1::part1(util::input(1).flat_map(|line| line.parse()))?;
    println!("{}", d1p1);

    let d1p2 = day1::part2(util::input(1).flat_map(|line| line.parse()))?;
    println!("{}", d1p2);

    Ok(())
}
