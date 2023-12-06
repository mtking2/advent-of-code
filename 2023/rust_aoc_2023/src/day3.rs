use std::cmp;

use aoc::Result as AocResult;
use itertools::Itertools;

pub fn part1<T: Iterator<Item = String>>(engine_schematic: T) -> AocResult<i32> {
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

				let start_idx = if j == row.len() - 1 && col.is_numeric() {
					(j - number_length) + 1
				} else {
					cmp::max(j as isize - number_length as isize, 0) as usize
				}; // first index of the number

				let test_start_idx = cmp::max(start_idx as isize - 1, 0) as usize; // first index of the number - 1
				let end_idx = if j == row.len() - 1 { j } else { j - 1 }; // last index of the number
				let test_end_idx = j; // last index of the number + 1 (or last if at end of row)

				// check if there is a symbol before the number
				if start_idx != test_start_idx
					&& !row[test_start_idx].is_numeric()
					&& row[test_start_idx] != '.'
				{
					is_part_number = true;
				}

				// check if there is a symbol after the number
				if !is_part_number
					&& end_idx != test_end_idx
					&& !row[test_end_idx].is_numeric()
					&& row[test_end_idx] != '.'
				{
					is_part_number = true;
				}

				// check if there is a symbol above the number
				if !is_part_number && i > 0 {
					// loop through the row above the number for the length of the number, including diagonal positions
					for k in (test_start_idx)..(test_end_idx + 1) {
						let test_char = schematic_vec[i - 1][k];
						if !test_char.is_numeric() && test_char != '.' {
							is_part_number = true;
							// println!("symbol above, char:{} ({},{})", test_char, i, k + 1);
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
							// println!("symbol below, char:{} ({},{})", test_char, i + 2, k + 1);
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

					// println!(
					// 	"row:{}, number:{}, len:{}, ({},{}), test:({},{})",
					// 	i, number, number_length, start_idx, end_idx, test_start_idx, test_end_idx
					// );

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

fn search_backward(vector: &Vec<Vec<char>>, i: usize, mut j: isize, number: &mut String) {
	while j as isize >= 0 && vector[i][j as usize].is_numeric() {
		number.push(vector[i][j as usize]);
		j -= 1;
	}
	*number = number.chars().rev().collect::<String>();
}

fn search_forward(vector: &Vec<Vec<char>>, i: usize, mut j: isize, number: &mut String) {
	while j < vector[i].len() as isize && vector[i][j as usize].is_numeric() {
		number.push(vector[i][j as usize]);
		j += 1;
	}
}

fn store_number(number: &mut String, numbers: &mut Vec<String>) {
	numbers.push(number.clone());
	number.clear();
}

pub fn part2<T: Iterator<Item = String>>(engine_schematic: T) -> AocResult<i32> {
	let schematic_vec = engine_schematic
		.map(|line| line.chars().collect_vec())
		.collect_vec();

	let mut total = 0;

	for (i, row) in schematic_vec.iter().enumerate() {
		for (j, col) in row.iter().enumerate() {
			if col == &'*' {
				// we have a *

				// search all positions around the * for numbers in order
				// 4 3 5
				// 1 _ 2
				// 7 6 8

				let not_first_col = j > 0;
				let not_last_col = j < row.len() - 1;
				let not_first_row = i > 0;
				let not_last_row = i < schematic_vec.len() - 1;

				// 4 1 7 if not_first_col
				// 5 2 8 if not_last_col
				// 4 3 5 if not_first_row
				// 7 6 8 if not_last_row

				let mut numbers = Vec::<String>::new();
				let mut number = String::new();

				// if 1 is number search backwards, inserting at begining of string, until we hit a non numeric and add to list
				if not_first_col && schematic_vec[i][j - 1].is_numeric() {
					search_backward(&schematic_vec, i, j as isize - 1, &mut number);
					store_number(&mut number, &mut numbers);
				}

				// if 2 is number search forwards, appending to string, until we hit a non numeric and add to list
				if not_last_col && schematic_vec[i][j + 1].is_numeric() {
					search_forward(&schematic_vec, i, j as isize + 1, &mut number);
					store_number(&mut number, &mut numbers);
				}

				// if 3 is number search backwards and forwards, inserting and appending to string
				if not_first_row && schematic_vec[i - 1][j].is_numeric() {
					search_backward(&schematic_vec, i - 1, j as isize, &mut number);
					search_forward(&schematic_vec, i - 1, j as isize + 1, &mut number);
					store_number(&mut number, &mut numbers);
				} else {
					if not_first_row && not_first_col && schematic_vec[i - 1][j - 1].is_numeric() {
						// if 4 is number search backwards
						search_backward(&schematic_vec, i - 1, j as isize - 1, &mut number);
						store_number(&mut number, &mut numbers);
					}
					if not_first_row && not_last_col && schematic_vec[i - 1][j + 1].is_numeric() {
						// if 5 is number search forwards
						search_forward(&schematic_vec, i - 1, j as isize + 1, &mut number);
						store_number(&mut number, &mut numbers);
					}
				}

				// if 6 is number search backwards and forwards
				if not_last_row && schematic_vec[i + 1][j].is_numeric() {
					search_backward(&schematic_vec, i + 1, j as isize, &mut number);
					search_forward(&schematic_vec, i + 1, j as isize + 1, &mut number);
					store_number(&mut number, &mut numbers);
				} else {
					if not_last_row && not_first_col && schematic_vec[i + 1][j - 1].is_numeric() {
						// if 7 is number search backwards
						search_backward(&schematic_vec, i + 1, j as isize - 1, &mut number);
						store_number(&mut number, &mut numbers);
					}
					if not_last_row && not_last_col && schematic_vec[i + 1][j + 1].is_numeric() {
						// if 8 is number search forwards
						search_forward(&schematic_vec, i + 1, j as isize + 1, &mut number);
						store_number(&mut number, &mut numbers);
					}
				}

				println!("numbers:{:?}", numbers);

				// if number count == 2, then add gear ratio to total
				if numbers.len() == 2 {
					let parsed_ints: Result<Vec<i32>, _> =
						numbers.into_iter().map(|s| s.parse::<i32>()).collect();
					match parsed_ints {
						Ok(ints) => {
							let gear_ratio: i32 = ints.iter().product();
							total += gear_ratio;
							println!("Gear ratio: {}", gear_ratio);
						}
						Err(e) => println!("Error parsing: {}", e),
					}
				}
			}
		}
	}
	Ok(total)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::YEAR;

	#[test]
	fn test_example_1() -> AocResult<()> {
		assert_eq!(
			part1(aoc::example(YEAR, 3, 1).flat_map(|line| line.parse()))?,
			4361
		);
		Ok(())
	}

	#[test]
	fn test_example_2() -> AocResult<()> {
		assert_eq!(
			part2(aoc::example(YEAR, 3, 2).flat_map(|line| line.parse()))?,
			467835
		);
		Ok(())
	}

	#[test]
	fn part1_test() -> AocResult<()> {
		assert_eq!(
			Some(part1(aoc::input(YEAR, 3).flat_map(|line| line.parse()))?),
			aoc::answer(YEAR, 3, 1)
		);
		Ok(())
	}

	#[test]
	fn part2_test() -> AocResult<()> {
		assert_eq!(
			Some(part2(aoc::input(YEAR, 3).flat_map(|line| line.parse()))?),
			aoc::answer(YEAR, 3, 2)
		);
		Ok(())
	}
}
