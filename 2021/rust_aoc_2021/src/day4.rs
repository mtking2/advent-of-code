use crate::Result;
use itertools::Itertools;
use itertools::any;
use itertools::all;

pub struct Board {
    rows: [[i32; 5]; 5],
    columns: [[i32; 5]; 5],
}

pub fn get_sequence_and_boards<T: Iterator<Item = String>>(data: T) -> Result<(Vec<i32>, Vec<Board>)> {
    let mut game_data = data.filter(|d| !d.is_empty());

    let number_sequence: Vec<i32> = game_data
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    // println!("number_sequence {:?}\n", number_sequence);

    let mut boards: Vec<Board> = Vec::new();

    for input_rows in &game_data.chunks(5) {
        let mut rows = [[0; 5]; 5];
        let mut columns = [[0; 5]; 5];

        for (i, row) in input_rows.enumerate() {
            row.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .enumerate()
                .for_each(|(j, n)| {
                    rows[i][j] = n;
                    columns[j][i] = n;
                });
        }
        // println!("{:?}", board);
        boards.push(Board { rows: rows, columns });
    }
    Ok((number_sequence, boards))
}

pub fn part1<T: Iterator<Item = String>>(data: T) -> Result<i32> {
    let (mut number_sequence, boards) = get_sequence_and_boards(data)?;

    let mut temp_sequence = number_sequence.split_off(5);
    let mut winning_board: Option<Board> = None;

    // println!("number_sequence: {:?}", number_sequence);
    // println!("temp_sequence: {:?}\n", temp_sequence);

    while (winning_board.is_none()) && temp_sequence.len() > 0 {
        for board in &boards {
            let has_matching_row: bool = any(&board.rows, |row| all(row, |e| number_sequence.contains(e)));
            let has_matching_col: bool = any(&board.columns, |col| all(col, |e| number_sequence.contains(e)));
            if has_matching_row || has_matching_col {
                winning_board = Some(Board { rows: board.rows, columns: board.columns });
                break;
            } else {
                winning_board = None;
            }
        }
        if winning_board.is_none() {
            number_sequence.push(temp_sequence.remove(0));
        }
    }

    let winner = winning_board.unwrap();
    println!("winning board: {:?}", winner.rows);
    println!("winning number: {:?}", number_sequence.last());
    let mut sum: i32 = 0;

    for row in winner.rows {
        for col in row {
            if !number_sequence.contains(&col) {
                sum += col;
            }
        }
    }

    Ok(sum * number_sequence.last().unwrap())
}

//pub fn part2<T: Iterator<Item = String>>(data: T) -> Result<i32> {
//    // part1(data, true)
//    let (mut number_sequence, boards) = get_sequence_and_boards(data)?;
//
//
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() -> Result<()> {
        assert_eq!(
            part1(aoc::example(2021, 4, 1).flat_map(|line| line.parse()))?,
            4512
        );
        Ok(())
    }

//    #[test]
//    fn test_example_2() -> Result<()> {
//        assert_eq!(
//            part2(aoc::example(4, 1).flat_map(|line| line.parse()))?,
//            1924
//        );
//        Ok(())
//    }

    #[test]
    fn part1_test() -> Result<()> {
        assert_eq!(
            Some(part1(aoc::input(2021, 4).flat_map(|line| line.parse()))?),
            aoc::answer(2021, 4, 1)
        );
        Ok(())
    }
}
