use aoc::Result as AocResult;

fn get_card_score(card: String) -> (u32, u32) {
	let mut winning_count: u32 = 0;
	let delimiters = &[':', '|'];

	// let mut parts = scratch_card.split(':');
	let mut parts = card.split(move |c| delimiters.contains(&c));

	// Destructuring directly into two variables
	let (_card_title_str, winning_numbers_str, scratch_numbers_str) = if let (
		Some(first),
		Some(second),
		Some(third),
	) =
		(parts.next(), parts.next(), parts.next())
	{
		(first, second, third)
	} else {
		// Handle the case where the input does not match the expected format
		panic!("Input does not contain three parts separated by a colon and a pipe");
	};

	let winning_numbers: Vec<u32> = winning_numbers_str
		.split_whitespace()
		.map(|s| s.parse::<u32>().unwrap())
		.collect();

	let scratch_numbers: Vec<u32> = scratch_numbers_str
		.split_whitespace()
		.map(|s| s.parse::<u32>().unwrap())
		.collect();

	// print!("winning_numbers: ");
	for number in scratch_numbers {
		if winning_numbers.contains(&number) {
			winning_count += 1;
			// print!("{} ", number);
		}
	}
	// println!();
	if winning_count > 0 {
		(winning_count, 2u32.pow(winning_count - 1))
	} else {
		(0, 0)
	}
}

pub fn part1<T: Iterator<Item = String>>(scratch_cards: T) -> AocResult<u32> {
	let mut total: u32 = 0;

	for scratch_card in scratch_cards {
		let (_winning_count, card_score) = get_card_score(scratch_card);

		if card_score > 0 {
			total += card_score;
		}
	}

	Ok(total)
}

fn get_card_copies(
	original_cards: &Vec<(u32, u32)>,
	cards_to_copy: &Vec<(u32, u32)>,
) -> Vec<(u32, u32)> {
	let mut copies = <Vec<(u32, u32)>>::new();

	for (_i, card) in cards_to_copy.iter().enumerate() {
		let start_idx = card.0 as usize + 1;
		let end_idx = start_idx + card.1 as usize;

		let card_copies = &original_cards[start_idx..end_idx].to_vec();

		copies.extend(card_copies);
	}

	copies.sort_by(|a, b| a.0.cmp(&b.0));
	copies
}

pub fn part2<T: Iterator<Item = String>>(scratch_cards: T) -> AocResult<u32> {
	let mut cards = <Vec<(u32, u32)>>::new();

	for (i, scratch_card) in (0u32..).zip(scratch_cards) {
		let (winning_count, _card_score) = get_card_score(scratch_card);
		cards.push((i, winning_count));
	}
	println!("cards: {:?}\n\n", cards);

	let mut all_cards = cards.clone();

	let mut copies = get_card_copies(&cards, &cards);

	while copies.len() > 0 {
		all_cards.extend(&copies);
		copies = get_card_copies(&cards, &copies);
	}

	Ok(all_cards.len() as u32)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::YEAR;

	#[test]
	fn test_example_1() -> AocResult<()> {
		assert_eq!(
			part1(aoc::example(YEAR, 4, 1).flat_map(|line| line.parse()))?,
			13
		);
		Ok(())
	}

	#[test]
	fn test_example_2() -> AocResult<()> {
		assert_eq!(
			part2(aoc::example(YEAR, 4, 2).flat_map(|line| line.parse()))?,
			30
		);
		Ok(())
	}

	#[test]
	fn part1_test() -> AocResult<()> {
		assert_eq!(
			Some(part1(aoc::input(YEAR, 4).flat_map(|line| line.parse()))?),
			aoc::answer(YEAR, 4, 1)
		);
		Ok(())
	}

	#[test]
	fn part2_test() -> AocResult<()> {
		assert_eq!(
			Some(part2(aoc::input(YEAR, 4).flat_map(|line| line.parse()))?),
			aoc::answer(YEAR, 4, 2)
		);
		Ok(())
	}
}
