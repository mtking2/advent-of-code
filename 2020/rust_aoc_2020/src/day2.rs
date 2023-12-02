use crate::Result;

pub fn is_valid_password_part1(policy: &str, password: &str) -> Result<bool> {
    // println!("{}: {}", policy, password);
    let mut policy_parts = policy.split(" ");
    let (bounds, key): (&str, char) = (policy_parts.next().unwrap(), policy_parts.next().unwrap().parse().expect("not a single char"));
    let mut bounds = bounds.split("-");
    let (min, max): (usize, usize) = (
        bounds.next().unwrap().parse().expect("NaN"),
        bounds.next().unwrap().parse().expect("NaN"),
    );
    // println!("({},{}) => {}", min, max, key);

    let count = password.chars().filter(|c| c == &key).count();
    let result = count >= min && count <= max;
    Ok(result)
}

pub fn is_valid_password_part2(policy: &str, password: &str) -> Result<bool> {
    let mut policy_parts = policy.split(" ");
    let (bounds, key): (&str, char) = (policy_parts.next().unwrap(), policy_parts.next().unwrap().parse().expect("not a single char"));
    let mut bounds = bounds.split("-");
    let (i, j): (usize, usize) = (
        bounds.next().unwrap().parse::<usize>().expect("NaN") - 1,
        bounds.next().unwrap().parse::<usize>().expect("NaN") - 1,
    );

    // println!("(i,j) = ({},{})", i, j);
    let chars: Vec<char> = password.chars().collect();
    let c1 = chars[i];
    let c2 = chars[j];

    let result = (c1 == key) ^ (c2 == key); // exclusive OR

    // println!("{} => {} => {:?},{:?} === {}", password, key, c1, c2, result);
    Ok(result)
}

pub fn count_valid_passwords<T: Iterator<Item = String>, F: Fn(&str, &str) -> Result<bool>>(data: T, check_fn: F) -> Result<i32> {
    let mut num_passwords = 0;
    let mut valid_passwords = 0;

    for line in data {
        let v: Vec<&str> = line.split(":").map(|s| s.trim()).collect();
        let (policy, password) = (v[0], v[1]);

        match check_fn(policy, password) {
            Ok(is_valid) => {
                if is_valid {
                    valid_passwords += 1;
                }
                num_passwords += 1;
            }
            Err(_) => (),
        }
    }

    println!(
        "Counted {}/{} valid passwords",
        valid_passwords, num_passwords
    );
    Ok(valid_passwords)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() -> Result<()> {
        let mut examples = aoc::example(2020, 2, 1);

        let e1 = examples.next().unwrap();
        let e2 = examples.next().unwrap();
        let e3 = examples.next().unwrap();
        assert_eq!(None, examples.next());

        let v: Vec<&str> = e1.split(": ").collect();
        assert_eq!(is_valid_password_part1(v[0], v[1])?, true);

        let v: Vec<&str> = e2.split(": ").collect();
        assert_eq!(is_valid_password_part1(v[0], v[1])?, false);

        let v: Vec<&str> = e3.split(": ").collect();
        assert_eq!(is_valid_password_part1(v[0], v[1])?, true);

        Ok(())
    }

    #[test]
    fn test_example_part2() -> Result<()> {
        let mut examples = aoc::example(2020, 2, 1);

        let e1 = examples.next().unwrap();
        let e2 = examples.next().unwrap();
        let e3 = examples.next().unwrap();
        assert_eq!(None, examples.next());

        let v: Vec<&str> = e1.split(": ").collect();
        assert_eq!(is_valid_password_part2(v[0], v[1])?, true);

        let v: Vec<&str> = e2.split(": ").collect();
        assert_eq!(is_valid_password_part2(v[0], v[1])?, false);

        let v: Vec<&str> = e3.split(": ").collect();
        assert_eq!(is_valid_password_part2(v[0], v[1])?, false);

        Ok(())
    }

    #[test]
    fn test_example_count_valid_passwords_part1() -> Result<()> {
        assert_eq!(count_valid_passwords(aoc::example(2020, 2, 1), is_valid_password_part1)?, 2);
        Ok(())
    }

    #[test]
    fn test_example_count_valid_passwords_part2() -> Result<()> {
        assert_eq!(count_valid_passwords(aoc::example(2020, 2, 1), is_valid_password_part2)?, 1);
        Ok(())
    }

    #[test]
    fn part1_test() -> Result<()> {
        assert_eq!(
            Some(count_valid_passwords(aoc::input(2020, 2), is_valid_password_part1)?),
            aoc::answer(2020, 2, 1)
        );
        Ok(())
    }

    #[test]
    fn part2_test() -> Result<()> {
        assert_eq!(
            Some(count_valid_passwords(aoc::input(2020, 2), is_valid_password_part2)?),
            aoc::answer(2020, 2, 2)
        );
        Ok(())
    }
}
