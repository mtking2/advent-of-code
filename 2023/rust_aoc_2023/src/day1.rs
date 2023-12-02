use aoc::Result;
use regex::{Captures, Regex};

pub fn part1<T: Iterator<Item = String>>(calibration_file: T) -> Result<i32> {
	let mut total = 0;
	for line in calibration_file {
		println!("line: {}", line);

		let mut numbers = "".to_string();

		// get the first number
		for char in line.chars() {
			if char.is_numeric() {
				numbers.push(char);
				break;
			}
		}

		// get the last number
		for char in line.chars().rev() {
			if char.is_numeric() {
				numbers.push(char);
				break;
			}
		}

		let number = numbers.parse::<i32>().unwrap();
		println!("number: {}", number);
		total += number
	}
	return Ok(total);
}

fn get_number(token: &str) -> Option<&str> {
	match token {
		"one" => Some("1"),
		"two" => Some("2"),
		"three" => Some("3"),
		"four" => Some("4"),
		"five" => Some("5"),
		"six" => Some("6"),
		"seven" => Some("7"),
		"eight" => Some("8"),
		"nine" => Some("9"),
		// overlapping numbers
		"twone" => Some("21"),
		"oneight" => Some("18"),
		"threeight" => Some("38"),
		"fiveight" => Some("58"),
		"sevenine" => Some("79"),
		"eightwo" => Some("82"),
		"eighthree" => Some("83"),
		"nineight" => Some("98"),
		_ => None,
	}
}

pub fn part2<T: Iterator<Item = String>>(calibration_file: T) -> Result<i32> {
	let mut total = 0;
	for line in calibration_file {
		println!("line: {}", line);

		let mut numbers = "".to_string();

		let re = Regex::new(r"(twone|oneight|threeight|fiveight|sevenine|eightwo|eighthree|nineight|one|two|three|four|five|six|seven|eight|nine)").unwrap();
		let parsed_string = re.replace_all(&line, |caps: &Captures| {
			get_number(caps.get(0).unwrap().as_str())
				.unwrap()
				.to_string()
		});
		println!("parsed_str: {:?}", parsed_string);

		// get the first number
		for char in parsed_string.chars() {
			if char.is_numeric() {
				numbers.push(char);
				break;
			}
		}

		// get the last number
		for char in parsed_string.chars().rev() {
			if char.is_numeric() {
				numbers.push(char);
				break;
			}
		}

		let number = numbers.parse::<i32>().unwrap();
		println!("number: {}", number);
		total += number
	}
	Ok(total)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::YEAR;

	#[test]
	fn test_example_1() -> Result<()> {
		assert_eq!(
			part1(aoc::example(YEAR, 1, 1).flat_map(|line| line.parse()))?,
			142
		);
		Ok(())
	}

	#[test]
	fn test_example_2() -> Result<()> {
		assert_eq!(
			part2(aoc::example(YEAR, 1, 2).flat_map(|line| line.parse()))?,
			281
		);
		Ok(())
	}

	#[test]
	fn part1_test() -> Result<()> {
		assert_eq!(
			Some(part1(aoc::input(YEAR, 1).flat_map(|line| line.parse()))?),
			aoc::answer(YEAR, 1, 1)
		);
		Ok(())
	}

	#[test]
	fn part2_test() -> Result<()> {
		assert_eq!(
			Some(part2(aoc::input(YEAR, 1).flat_map(|line| line.parse()))?),
			aoc::answer(YEAR, 1, 2)
		);
		Ok(())
	}
}
