use aoc;
use aoc::Result;
use rust_aoc_2023::day5;
use rust_aoc_2023::YEAR;

fn main() -> Result<()> {
	println!("\nDay 5:");

	let d5p2 = day5::part2(aoc::example(YEAR, 5, 1).flat_map(|line| line.parse()))?;
	// let d5p2 = day5::part2(aoc::input(2023, 5).flat_map(|line| line.parse()))?;
	println!("{}", d5p2);

	Ok(())
}
