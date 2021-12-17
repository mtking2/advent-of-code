use crate::Result;

pub fn get_gamma_epsilon_rates(report: &Vec<String>) -> Result<(String, String)> {
    let mut gamma_counts = vec![0; *&report[0].chars().count()];

    for bin_str in report {
        bin_str.chars().enumerate().for_each(|(i, c)| {
            if c == '1' {
                gamma_counts[i] += 1;
            } else {
                gamma_counts[i] -= 1;
            }
        });
    }
    // println!("gamma_counts: {:?}", gamma_counts);

    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();
    for count in gamma_counts {
        if count >= 0 {
            gamma_rate.push('1');
            epsilon_rate.push('0');
        } else {
            gamma_rate.push('0');
            epsilon_rate.push('1');
        }
    }
    Ok((gamma_rate, epsilon_rate))
}

pub fn part1<T: Iterator<Item = String>>(data: T) -> Result<i32> {
    let report: Vec<String> = data.collect();
    let (gamma_rate, epsilon_rate) = get_gamma_epsilon_rates(&report)?;

    let gamma_val = i32::from_str_radix(&gamma_rate, 2).unwrap();
    let epsilon_val = i32::from_str_radix(&epsilon_rate, 2).unwrap();

    // println!("{} {}", gamma_val, epsilon_val);

    Ok(gamma_val * epsilon_val)
}

pub fn part2<T: Iterator<Item = String>>(data: T) -> Result<i32> {
    let mut report: Vec<String> = data.collect();
    let mut report_copy = report.clone();
    println!("Full report: {:?}", report);

    let mut gamma_rate = get_gamma_epsilon_rates(&report)?.0;
    let mut epsilon_rate = get_gamma_epsilon_rates(&report_copy)?.1;

    let mut i = 0;
    while report.len() > 1 || report_copy.len() > 1 {
        if report.len() > 1 {
            report.retain(|d| d.chars().nth(i).unwrap() == gamma_rate.chars().nth(i).unwrap());
        }
        if report_copy.len() > 1 {
            report_copy.retain(|d| d.chars().nth(i).unwrap() == epsilon_rate.chars().nth(i).unwrap());
        }
        gamma_rate = get_gamma_epsilon_rates(&report)?.0;
        epsilon_rate = get_gamma_epsilon_rates(&report_copy)?.1;
        i += 1;
        // println!("{:?}", report_copy);
    }

    let oxygen_generator_rating = i32::from_str_radix(&report[0], 2).unwrap();
    let co2_scrubber_rating = i32::from_str_radix(&report_copy[0], 2).unwrap();

    Ok(oxygen_generator_rating * co2_scrubber_rating)
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

    #[test]
    fn test_example_2() -> Result<()> {
        assert_eq!(
            part2(util::example(3, 1).flat_map(|line| line.parse()))?,
            230
        );
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        assert_eq!(
            Some(part2(util::input(3).flat_map(|line| line.parse()))?),
            util::answer(3, 2)
        );
        Ok(())
    }
}