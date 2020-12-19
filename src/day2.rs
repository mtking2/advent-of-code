use crate::Result;

pub fn run<T: Iterator<Item = String>>(data: T) -> Result<bool> {
    let result = false;

    for line in data {
        // println!("{}", line);
        let v: Vec<&str> = line.split(":").map(|s| s.trim()).collect();
        println!("{:?}", v);
    }
    Ok(result)
}
