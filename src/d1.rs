// use std::fs;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

pub fn run() {
    println!("Hello from d1!");

    let file_name = "/Users/michael/Projects/advent-of-code/src/input.txt";

    let file = File::open(&file_name).expect("file not found");
    let reader = BufReader::new(file);
    // let contents = fs::read_to_string("input.txt")
    //     .expect("Something went wrong reading the file");

    for line in reader.lines() {
        // func(&line.unwrap());
        println!("{}", &line.unwrap());
    }

    // println!("input.txt:\n{}", contents);
}
