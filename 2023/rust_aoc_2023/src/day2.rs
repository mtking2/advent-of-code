use aoc::Result;
use std::collections::HashMap;

fn get_cube_max_count(color: &str) -> Option<i16> {
	match color {
		"red" => Some(12),
		"green" => Some(13),
		"blue" => Some(14),
		_ => None,
	}
}

fn is_game_possible(sets: &str) -> Result<bool> {
	let mut is_possible = true;

	for set in sets.split(";").into_iter() {
		for block_group in set.split(",").into_iter() {
			let mut group_parts = block_group.split_whitespace();

			let (count, color): (i16, &str) = (
				group_parts.next().unwrap().parse()?,
				group_parts.next().unwrap(),
			);

			if count > get_cube_max_count(color).unwrap() {
				is_possible = false;
				break;
			}
		}
		if !is_possible {
			break;
		}
	}
	Ok(is_possible)
}

pub fn part1<T: Iterator<Item = String>>(game_records: T) -> Result<i32> {
	let mut total = 0;
	for game in game_records {
		println!("{}", game);

		let mut parts = game.split(':');

		// Destructuring directly into two variables
		let (game_title, sets) = if let (Some(first), Some(second)) = (parts.next(), parts.next()) {
			(first, second)
		} else {
			// Handle the case where the input does not match the expected format
			panic!("Input does not contain two parts separated by a colon");
		};

		let game_id: i32 = game_title
			.split_whitespace()
			.last()
			.unwrap()
			.parse::<i32>()
			.unwrap();

		println!("game_id: {}", game_id);
		println!("sets: {}", sets);

		if is_game_possible(sets).unwrap() {
			total += game_id;
		}
	}
	Ok(total)
}

fn get_game_power(sets: &str) -> Result<i32> {
	let mut max_hash = HashMap::<&str, i32>::from([("red", 1), ("green", 1), ("blue", 1)]);
	println!("max_hash: {:?}", max_hash);

	for set in sets.split(";").into_iter() {
		for block_group in set.split(",").into_iter() {
			let mut group_parts = block_group.split_whitespace();

			let (count, color): (i32, &str) = (
				group_parts.next().unwrap().parse()?,
				group_parts.next().unwrap(),
			);

			if &count > max_hash.get(color).unwrap() {
				max_hash.insert(color, count);
			}
		}
	}

	// get the power of the game, meaning red_max * green_max * blue_max
	let power = max_hash.values().product::<i32>();
	Ok(power)
}

pub fn part2<T: Iterator<Item = String>>(game_records: T) -> Result<i32> {
	let mut total = 0;
	// get max count for each color for each game
	for game in game_records {
		let sets = game.split(':').last().unwrap().trim();
		println!("sets: {:?}", sets);

		let power = get_game_power(sets)?;
		println!("power: {}", power);

		// sum the powers of all games
		total += power;
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
			part1(aoc::example(YEAR, 2, 1).flat_map(|line| line.parse()))?,
			8
		);
		Ok(())
	}

	#[test]
	fn test_example_2() -> Result<()> {
		assert_eq!(
			part2(aoc::example(YEAR, 2, 2).flat_map(|line| line.parse()))?,
			2286
		);
		Ok(())
	}

	#[test]
	fn part1_test() -> Result<()> {
		assert_eq!(
			Some(part1(aoc::input(YEAR, 2).flat_map(|line| line.parse()))?),
			aoc::answer(YEAR, 2, 1)
		);
		Ok(())
	}

	#[test]
	fn part2_test() -> Result<()> {
		assert_eq!(
			Some(part2(aoc::input(YEAR, 2).flat_map(|line| line.parse()))?),
			aoc::answer(YEAR, 2, 2)
		);
		Ok(())
	}
}
