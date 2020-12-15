use std::error::Error;
use itertools::Itertools;

pub fn part1<T: Iterator<Item = i32>>(expenses: T) -> Result<i32, Box<dyn Error + Send + Sync>> {
    let result = expenses
        .combinations(2)
        .find(|n| n[0] + n[1] == 2020)
        .map(|n| n[0] * n[1])
        .unwrap();
    Ok(result)
}

pub fn part2<T: Iterator<Item = i32>>(expenses: T) -> Result<i32, Box<dyn Error + Send + Sync>> {
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
    use crate::util;

    #[test]
    fn test() {
        assert_eq!(
            part1(util::example(1, 1).flat_map(|line| line.parse())).unwrap(),
            514579
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            part2(util::example(1, 1).flat_map(|line| line.parse())).unwrap(),
            241861950
        );
    }

    #[test]
    fn part1_test() {
        assert_eq!(
            Some(part1(util::input(1).flat_map(|line| line.parse())).unwrap()),
            util::answer(1, 1)
        );
    }

    #[test]
    fn part2_test() {
        assert_eq!(
            Some(part2(util::input(1).flat_map(|line| line.parse())).unwrap()),
            util::answer(1, 2)
        );
    }
}
