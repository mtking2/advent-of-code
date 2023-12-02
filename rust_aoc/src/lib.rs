mod aoc;
pub use self::aoc::*;

// pub use self::aoc::example;
// pub use crate::aoc::input;
// pub use crate::aoc::answer;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
