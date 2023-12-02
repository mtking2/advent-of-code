mod aoc;
pub use self::aoc::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut result = example(2020, 1, 1);

        assert_eq!(result.next().unwrap(), "1721");
        assert_eq!(result.next().unwrap(), "979");
        assert_eq!(result.next().unwrap(), "366");
        assert_eq!(result.next().unwrap(), "299");
        assert_eq!(result.next().unwrap(), "675");
        assert_eq!(result.next().unwrap(), "1456");
    }
}
