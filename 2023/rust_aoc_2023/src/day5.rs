use aoc::Result as AocResult;
use regex::Regex;

fn get_seed_location(seed: u64, almanac_map: &Vec<String>) -> u64 {
	// println!("seed: {}", seed);
	let map_re = Regex::new(r" map:$").unwrap();
	// let mut map_mode: String;

	let mut map_cursor = seed as i64;
	let mut skip_to_next_map = false;

	for line in almanac_map {
		if map_re.is_match(&line) {
			// map_mode = map_re.replace(&line, "").to_string();
			// println!("map_mode: {}", map_mode);
			skip_to_next_map = false;
		} else if line == "" {
			continue;
		} else if skip_to_next_map {
			continue;
		} else {
			// println!("line: {:?}", line);

			let mut map_nums = line.split(" ");

			let (des_range_start, src_range_start, range_length) = (
				map_nums.next().unwrap().parse::<i64>().unwrap(),
				map_nums.next().unwrap().parse::<i64>().unwrap(),
				map_nums.next().unwrap().parse::<i64>().unwrap(),
			);

			// let des_range_end = des_range_start + range_length;
			let src_range_end = src_range_start + range_length;

			if map_cursor >= src_range_start && map_cursor <= src_range_end {
				// cursor is in the source range, we can map it
				// println!(
				// 	"map_cursor: {}, src_range_start: {}, src_range_end: {}, des_range_start: {}, des_range_end: {}",
				// 	map_cursor, src_range_start, src_range_end, des_range_start, des_range_end
				// );

				let delta = des_range_start - src_range_start;
				map_cursor += delta;

				skip_to_next_map = true;
			}
		}
	}
	map_cursor as u64
}

pub fn part1<T: Iterator<Item = String>>(mut almanac: T) -> AocResult<u64> {
	let seeds_re = Regex::new(r"^seeds: ").unwrap();

	let seeds_line = almanac.next().unwrap();
	assert!(seeds_re.is_match(&seeds_line));

	let seeds: Vec<u64> = seeds_re
		.replace(&seeds_line, "")
		.split(" ")
		.map(|s| s.parse().unwrap())
		.collect();
	// println!("seeds: {:?}", seeds);

	let mut seed_locations: Vec<u64> = Vec::new();
	let map: Vec<String> = almanac.collect();

	for seed in seeds {
		seed_locations.push(get_seed_location(seed, &map));
	}
	// println!("seed_locations: {:?}", seed_locations);

	let min_location = seed_locations.iter().min().unwrap();

	Ok(*min_location)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::YEAR;

	#[test]
	fn test_example_1() -> AocResult<()> {
		assert_eq!(
			part1(aoc::example(YEAR, 5, 1).flat_map(|line| line.parse()))?,
			35
		);
		Ok(())
	}

	#[test]
	fn part1_test() -> AocResult<()> {
		assert_eq!(
			Some(part1(aoc::input(YEAR, 5).flat_map(|line| line.parse()))?),
			aoc::answer(YEAR, 5, 1)
		);
		Ok(())
	}
}
