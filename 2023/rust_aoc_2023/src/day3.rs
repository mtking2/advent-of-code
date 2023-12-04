use std::cmp;

use aoc::Result;
use itertools::Itertools;

pub fn part1<T: Iterator<Item = String>>(engine_schematic: T) -> Result<i32> {
	let schematic_vec = engine_schematic
		.map(|line| line.chars().collect_vec())
		.collect_vec();

	let mut total = 0;

	for (i, row) in schematic_vec.iter().enumerate() {
		let mut number_length = 0;

		for (j, col) in row.iter().enumerate() {
			// print!("{}", col); // print the column

			if col.is_numeric() {
				number_length += 1;
			}

			if (!col.is_numeric() || j == row.len() - 1) && number_length > 0 {
				// we have reached the end of a number, the end of the row, or both
				// j is currently at the index after the last digit, or is the index of the last digit if at end of row
				let mut is_part_number = false;

				// let start_idx = cmp::max((j as isize - number_length), 0) as usize; // first index of the number
				let start_idx = if j == row.len() - 1 && col.is_numeric() {
					(j - number_length) + 1
				} else {
					cmp::max(j as isize - number_length as isize, 0) as usize
				}; // first index of the number
				let test_start_idx =
					cmp::max((j as isize - number_length as isize) - 1, 0) as usize; // first index of the number - 1

				let end_idx = if j == row.len() - 1 { j } else { j - 1 }; // last index of the number
				let test_end_idx = j; // last index of the number + 1 (or last if at end of row)

				// println!(
				// 	"row:{}, ({},{}), test:({},{})",
				// 	i, start_idx, end_idx, test_start_idx, test_end_idx
				// );
				// println!(
				// 	"number:{}",
				// 	row.iter()
				// 		.skip(start_idx)
				// 		.take(number_length as usize)
				// 		.collect::<String>()
				// );
				// break;

				// check if there is a symbol before the number
				if start_idx != test_start_idx
					&& !row[test_start_idx].is_numeric()
					&& row[test_start_idx] != '.'
				{
					is_part_number = true;
					println!("symbol left of number");
				}

				// check if there is a symbol after the number
				if !is_part_number
					&& end_idx != test_end_idx
					&& !row[test_end_idx].is_numeric()
					&& row[test_end_idx] != '.'
				{
					is_part_number = true;
					println!("symbol right of number");
				}

				// check if there is a symbol above the number
				if !is_part_number && i > 0 {
					// loop through the row above the number for the length of the number, including diagonal positions
					for k in (test_start_idx)..(test_end_idx + 1) {
						let test_char = schematic_vec[i - 1][k];
						if !test_char.is_numeric() && test_char != '.' {
							is_part_number = true;
							println!("symbol above number");
							break;
						}
					}
				}

				// check if there is a symbol anywhere below the number
				if !is_part_number && i < schematic_vec.len() - 1 {
					// loop through the row below the number for the length of the number, including diagonal positions
					for k in (test_start_idx)..(test_end_idx + 1) {
						let test_char = schematic_vec[i + 1][k];
						if !test_char.is_numeric() && test_char != '.' {
							is_part_number = true;
							println!("symbol below number");
							break;
						}
					}
				}

				// if there IS a symbol, then add the number to the total
				// get an i32 of the number before adding it to the total
				if is_part_number {
					let number = row
						.iter()
						.skip(start_idx)
						.take(number_length as usize)
						.collect::<String>()
						.parse::<i32>()?;

					println!(
						"row:{}, number:{}, ({},{}), test:({},{})",
						i, number, start_idx, end_idx, test_start_idx, test_end_idx
					);
					total += number;
				}

				number_length = 0;
			}
		}
		// println!(); // print a new line
	}

	// if symbol idx is within range of number idx, then add to count

	Ok(total)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::YEAR;

	#[test]
	fn test_example_1() -> Result<()> {
		assert_eq!(
			part1(aoc::example(YEAR, 3, 1).flat_map(|line| line.parse()))?,
			4361
		);
		Ok(())
	}

	#[test]
	fn part1_test() -> Result<()> {
		assert_eq!(
			Some(part1(aoc::input(YEAR, 3).flat_map(|line| line.parse()))?),
			aoc::answer(YEAR, 3, 1)
		);
		Ok(())
	}
}
