use crate::Result;

pub fn is_valid_password(policy: &str, password: &str) -> Result<bool> {
    let result = false;
    println!("{}: {}", policy, password);
    // TODO: check pswd
    Ok(result)
}

pub fn count_valid_passwords<T: Iterator<Item = String>>(data: T) -> Result<i32> {
    let mut num_passwords = 0;
    let mut valid_passwords = 0;

    // let lines = data.map(|line| line.split(":").map(|s| s.trim()) );

    for line in data {
        // println!("{}", line);
        let v: Vec<&str> = line.split(":").map(|s| s.trim()).collect();
        let (policy, password) = (v[0], v[1]);
        // println!("{}: {}", policy, password);
        match is_valid_password(policy, password) {
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
    use crate::util;

    #[test]
    fn test_example() -> Result<()> {
        let mut examples = util::example(2, 1);

        let e1 = examples.next().unwrap();
        let e2 = examples.next().unwrap();
        let e3 = examples.next().unwrap();
        assert_eq!(None, examples.next());

        let v: Vec<&str> = e1.split(":").collect();
        assert_eq!(is_valid_password(v[0], v[1])?, true);

        let v: Vec<&str> = e2.split(":").collect();
        assert_eq!(is_valid_password(v[0], v[1])?, true);

        let v: Vec<&str> = e3.split(":").collect();
        assert_eq!(is_valid_password(v[0], v[1])?, true);

        Ok(())
    }

    #[test]
    fn test_example_count_valid_passwords() -> Result<()> {
        assert_eq!(count_valid_passwords(util::example(2, 1))?, 2);
        Ok(())
    }
}
