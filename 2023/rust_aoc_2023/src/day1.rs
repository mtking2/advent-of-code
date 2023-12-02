use crate::Result;
use itertools::Itertools;

pub fn part1<T: Iterator<Item = String>>(calibration_file: T) -> Result<i32> {
	// TODO: Implement part 1
	return Ok(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() -> Result<()> {
        assert_eq!(
            part1(aoc::example(2023, 1, 1).flat_map(|line| line.parse()))?,
            7
        );
        Ok(())
    }
}
