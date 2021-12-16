use crate::Result;

pub fn part1<T: Iterator<Item = String>>(data: T) -> Result<i32> {
    let report: Vec<String> = data.collect();
    let mut gamma_counts = vec![0; report[0].chars().count()];

    for bin_str in report {
        bin_str.chars().enumerate().for_each(|(i, c)| {
            if c == '1' {
                gamma_counts[i] += 1;
            } else {
                gamma_counts[i] -= 1;
            }
        });
    }

    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();
    for count in gamma_counts {
        if count > 0 {
            gamma_rate.push('1');
            epsilon_rate.push('0');
        } else {
            gamma_rate.push('0');
            epsilon_rate.push('1');
        }
    }

    let gamma_val = i32::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon_val = i32::from_str_radix(&epsilon_rate, 2).unwrap();

    // println!("{} {}", gamma_val, epsilon_val);

    Ok(gamma_val * epsilon_val)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test_example_1() -> Result<()> {
        assert_eq!(
            part1(util::example(3, 1).flat_map(|line| line.parse()))?,
            198
        );
        Ok(())
    }

    #[test]
    fn part1_test() -> Result<()> {
        assert_eq!(
            Some(part1(util::input(3).flat_map(|line| line.parse()))?),
            util::answer(3, 1)
        );
        Ok(())
    }

    // #[test]
    // fn test_example_2() -> Result<()> {
    //     assert_eq!(
    //         part2(util::example(2, 1).flat_map(|line| line.parse()))?,
    //         900
    //     );
    //     Ok(())
    // }

    // #[test]
    // fn part2_test() -> Result<()> {
    //     assert_eq!(
    //         Some(part2(util::input(2).flat_map(|line| line.parse()))?),
    //         util::answer(2, 2)
    //     );
    //     Ok(())
    // }
}