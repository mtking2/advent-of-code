use aoc::Result as AocResult;
use itertools::Itertools;
use regex::Regex;
use std::cmp;

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

fn map_seed_range(
	start: u64,
	end: u64,
	seeds: &mut Vec<(u64, u64)>,
	new_seeds: &mut Vec<(u64, u64)>,
	almanac_map: &Vec<Vec<u64>>,
) {
	println!("map: {:?}", almanac_map);

	let map: Vec<(u64, u64, u64)> = almanac_map
		.iter()
		.map(|map| (map[0], map[1], map[2]))
		.collect();

	let mut break_flag = false;

	for (des_range_start, src_range_start, range_length) in map {
		// println!(
		// 	"des_range_start: {}, src_range_start: {}, range_length: {}",
		// 	des_range_start, src_range_start, range_length
		// );

		let overlap_start = cmp::max(start, src_range_start);
		let overlap_end = cmp::min(end, src_range_start + range_length);

		if overlap_start < overlap_end {
			new_seeds.push((
				(des_range_start as i64 + (overlap_start as i64 - src_range_start as i64)) as u64,
				(des_range_start as i64 + (overlap_end as i64 - src_range_start as i64)) as u64,
			));

			if start < overlap_start {
				seeds.push((start, overlap_start));
			}
			if overlap_end < end {
				seeds.push((overlap_end, end));
			}
			break_flag = true;
			break;
		}
	}

	if !break_flag {
		new_seeds.push((start, end));
	}
}

pub fn part2<T: Iterator<Item = String>>(mut almanac: T) -> AocResult<u64> {
	let seeds_re = Regex::new(r"^seeds: ").unwrap();

	let file_str = almanac.join("\n");
	let file: Vec<&str> = file_str.split("\n\n").collect();
	println!("File: {:?}\n", file);

	let mut seeds: Vec<(u64, u64)> = seeds_re
		.replace(&file[0], "")
		.split(" ")
		.map(|s| s.parse::<u64>().unwrap())
		.tuples::<(u64, u64)>()
		.map(|(s, l)| (s, s + l))
		.collect();

	println!("Seeds: {:?}\n", seeds);

	let maps: Vec<_> = file
		.iter()
		.skip(1)
		.map(|m| {
			m.split("\n")
				.skip(1)
				.map(|l| {
					l.split(" ")
						.map(|n| n.parse::<u64>().unwrap())
						.collect::<Vec<u64>>()
				})
				.collect::<Vec<_>>()
		})
		.collect();

	println!("Maps: {:?}\n", maps);

	for map in maps {
		let mut new_seeds: Vec<(u64, u64)> = Vec::new();

		while seeds.len() > 0 {
			let (start_seed, end_seed) = seeds.pop().unwrap();
			map_seed_range(start_seed, end_seed, &mut seeds, &mut new_seeds, &map);
		}
		seeds = new_seeds;
		println!("\nSeeds: {:?}\n", seeds);
	}

	let min_location = seeds.iter().min().unwrap();

	Ok(min_location.0)
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
	fn test_example_2() -> AocResult<()> {
		assert_eq!(
			part2(aoc::example(YEAR, 5, 1).flat_map(|line| line.parse()))?,
			46
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

	#[test]
	fn part2_test() -> AocResult<()> {
		assert_eq!(
			Some(part2(aoc::input(YEAR, 5).flat_map(|line| line.parse()))?),
			aoc::answer(YEAR, 5, 2)
		);
		Ok(())
	}
}
