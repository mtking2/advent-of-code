pub mod day1;
pub mod day2;
pub mod util;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;