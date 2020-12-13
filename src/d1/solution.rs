// use std::fs;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

fn d1p1(filename: impl AsRef<Path>) {
    println!("Day 1, Part 1");

    let file = File::open(filename).expect("file not found");
    let reader = BufReader::new(file);
    let lines: Vec<u32> = reader.lines()
                    .map(|l| l.unwrap().parse::<u32>().expect("could not read line"))
                    .collect();

    for num_1 in &lines {
        for num_2 in &lines {

            if num_1 + num_2 == 2020 {
                println!("{} + {} == 2020", num_1, num_2);
                return println!("{} * {} == {}", num_1, num_2, num_1 * num_2);
            }
        }
    }
}

fn d1p2(filename: impl AsRef<Path>) {
    println!("Day 1, Part 2");

    let file = File::open(filename).expect("file not found");
    let reader = BufReader::new(file);
    let lines: Vec<u32> = reader.lines()
                    .map(|l| l.unwrap().parse::<u32>().expect("could not read line"))
                    .collect();

    for num_1 in &lines {
        for num_2 in &lines {
            for num_3 in &lines {

                if num_1 + num_2 + num_3 == 2020 {
                    println!("{} + {} + {} == 2020", num_1, num_2, num_3);
                    return println!("{} * {} * {} == {}",
                        num_1, num_2, num_3,
                        num_1 * num_2 * num_3
                    );
                }
            }
        }
    }
}

pub fn run(filename: impl AsRef<Path>) {
    d1p1(&filename);
    println!();
    d1p2(&filename);
}
