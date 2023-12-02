use crate::Result;
use itertools::Itertools;

pub fn part1<T: Iterator<Item = i32>>(expenses: T) -> Result<i32> {
    let result = expenses
        .combinations(2)
        .find(|n| n[0] + n[1] == 2020)
        .map(|n| n[0] * n[1])
        .unwrap();
    Ok(result)
}

pub fn part2<T: Iterator<Item = i32>>(expenses: T) -> Result<i32> {
    let result = expenses
        .combinations(3)
        .find(|n| n[0] + n[1] + n[2] == 2020)
        .map(|n| n[0] * n[1] * n[2])
        .unwrap();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() -> Result<()> {
        assert_eq!(
            part1(aoc::example(2020, 1, 1).flat_map(|line| line.parse()))?,
            514579
        );
        Ok(())
    }

    #[test]
    fn test_example_2() -> Result<()> {
        assert_eq!(
            part2(aoc::example(2020, 1, 1).flat_map(|line| line.parse()))?,
            241861950
        );
        Ok(())
    }

    #[test]
    fn part1_test() -> Result<()> {
        assert_eq!(
            Some(part1(aoc::input(2020, 1).flat_map(|line| line.parse()))?),
            aoc::answer(2020, 1, 1)
        );
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        assert_eq!(
            Some(part2(aoc::input(2020, 1).flat_map(|line| line.parse()))?),
            aoc::answer(2020, 1, 2)
        );
        Ok(())
    }
}
