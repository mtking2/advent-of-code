pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
