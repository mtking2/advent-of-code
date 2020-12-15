use aoc::day1;
use aoc::util;

fn main() {
    println!("Hello, Advent of Code!\n");

    let d1p1 = day1::part1(util::input(1).flat_map(|line| line.parse())).unwrap();
    println!("{}", d1p1);

    let d1p2 = day1::part2(util::input(1).flat_map(|line| line.parse())).unwrap();
    println!("{}", d1p2);
}
