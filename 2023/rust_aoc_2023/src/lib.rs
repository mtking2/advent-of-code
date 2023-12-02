pub mod day1;

const YEAR: u16 = 2023;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
