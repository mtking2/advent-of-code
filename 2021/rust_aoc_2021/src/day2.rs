use aoc::Result;

pub fn part1<T: Iterator<Item = String>>(movements: T) -> Result<i32> {
    let mut horizontal_position = 0;
    let mut depth = 0;
    
    for movement in movements {
        // println!("{}", movement);
        let mut movement_parts = movement.split(" ");
        let (dir, val): (&str, i32) = (movement_parts.next().unwrap(), movement_parts.next().unwrap().parse()?); 
        if dir == "forward" {
            horizontal_position += val;
        } else if dir == "up" {
            depth -= val;
        } else if dir == "down" {
            depth += val;
        }
    }

    Ok(horizontal_position * depth)
}

pub fn part2<T: Iterator<Item = String>>(movements: T) -> Result<i32> {
    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;
    
    for movement in movements {
        // println!("{}", movement);
        let mut movement_parts = movement.split(" ");
        let (dir, val): (&str, i32) = (movement_parts.next().unwrap(), movement_parts.next().unwrap().parse()?); 
        if dir == "forward" {
            horizontal_position += val;
            depth += aim * val;
        } else if dir == "up" {
            aim -= val;
        } else if dir == "down" {
            aim += val;
        }
    }

    Ok(horizontal_position * depth)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() -> Result<()> {
        assert_eq!(
            part1(aoc::example(2021, 2, 1).flat_map(|line| line.parse()))?,
            150
        );
        Ok(())
    }

    #[test]
    fn part1_test() -> Result<()> {
        assert_eq!(
            Some(part1(aoc::input(2021, 2).flat_map(|line| line.parse()))?),
            aoc::answer(2021, 2, 1)
        );
        Ok(())
    }

    #[test]
    fn test_example_2() -> Result<()> {
        assert_eq!(
            part2(aoc::example(2021, 2, 1).flat_map(|line| line.parse()))?,
            900
        );
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        assert_eq!(
            Some(part2(aoc::input(2021, 2).flat_map(|line| line.parse()))?),
            aoc::answer(2021, 2, 2)
        );
        Ok(())
    }
}
