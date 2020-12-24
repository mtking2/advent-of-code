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
    let d2p1 = day2::count_valid_passwords(util::input(2), day2::is_valid_password_part1)?;
    println!("{}", d2p1);
    let d2p2 = day2::count_valid_passwords(util::input(2), day2::is_valid_password_part2)?;
    println!("{}", d2p2);

    Ok(())
}
