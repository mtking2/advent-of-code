use aoc::Result;

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
			println!("{:?}", group_parts);

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
	fn part1_test() -> Result<()> {
		assert_eq!(
			Some(part1(aoc::input(YEAR, 2).flat_map(|line| line.parse()))?),
			aoc::answer(YEAR, 2, 1)
		);
		Ok(())
	}
}
