use crate::Result;
use itertools::Itertools;

pub fn part1<T: Iterator<Item = i32>>(mut measurements: T) -> Result<i32> {
    let mut count = 0;
    let mut prev_measurement = measurements.nth(0).unwrap();

    for measurement in measurements {
        // println!("{}, {}, {}", measurement, prev_measurement, count);
        if measurement > prev_measurement {
            count += 1;
        }
        prev_measurement = measurement;
    }
    Ok(count)
}

pub fn part2<T: Iterator<Item = i32>>(measurements: T) -> Result<i32> {
    let mut count = 0;
    let mut windows = measurements.tuple_windows::<(_,_,_)>();
    let first_window = windows.nth(0).unwrap();
    let mut prev_sum = first_window.0 + first_window.1 + first_window.2;

    for (a,b,c) in windows {
        let sum = a + b + c;
        // println!("{}, {}, {}", sum, prev_sum, count);
        if sum > prev_sum {
            count += 1;
        }
        prev_sum = sum;
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test_example_1() -> Result<()> {
        assert_eq!(
            part1(util::example(1, 1).flat_map(|line| line.parse()))?,
            7
        );
        Ok(())
    }

    #[test]
    fn test_example_2() -> Result<()> {
        assert_eq!(
            part2(util::example(1, 1).flat_map(|line| line.parse()))?,
            5
        );
        Ok(())
    }

    #[test]
    fn part1_test() -> Result<()> {
        assert_eq!(
            Some(part1(util::input(1).flat_map(|line| line.parse()))?),
            util::answer(1, 1)
        );
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        assert_eq!(
            Some(part2(util::input(1).flat_map(|line| line.parse()))?),
            util::answer(1, 2)
        );
        Ok(())
    }
}
