use crate::Result;
use itertools::Itertools;

struct Board {
    data: [[i32; 5]; 5],
}

pub fn part1<T: Iterator<Item = String>>(data: T) -> Result<i32> {
    let mut game_data = data.filter(|d| !d.is_empty());

    let number_sequence: Vec<i32> = game_data
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    println!("{:?}", number_sequence);

    let mut boards: Vec<Board> = Vec::new();

    for rows in &game_data.chunks(5) {
        let mut board = [[0; 5]; 5];

        for (i, row) in rows.enumerate() {
            row.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .enumerate()
                .for_each(|(j, n)| {
                    board[i][j] = n;
                });
        }
        // println!("{:?}", board);
        boards.push(Board { data: board });
    }

    for board in boards {
        println!("{:?}", board.data);
    }

    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test_example_1() -> Result<()> {
        assert_eq!(
            part1(util::example(4, 1).flat_map(|line| line.parse()))?,
            4512
        );
        Ok(())
    }

    // #[test]
    // fn part1_test() -> Result<()> {
    //     assert_eq!(
    //         Some(part1(util::input(4).flat_map(|line| line.parse()))?),
    //         util::answer(4, 1)
    //     );
    //     Ok(())
    // }
}
