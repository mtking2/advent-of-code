// Shamelessly stolen from BenoitZugmeyer because it's genius.
// Helpers to fetch input data directly from adventofcode.com, caching results and answers.
//
// https://github.com/BenoitZugmeyer/RustyAdventOfCode/blob/master/2020/src/util.rs

use native_tls::TlsConnector;

use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader, Seek, Write},
    net::TcpStream,
    path::Path,
};

fn fetch_aoc(path: &str) -> impl Iterator<Item = String> {
    let session =
        std::env::var("AOC_SESSION").expect("AOC_SESSION environment variable is missing");
    let connector = TlsConnector::new().expect("Failed to create a TLS connector");
    let stream =
        TcpStream::connect("adventofcode.com:443").expect("Failed to connect to the server");
    let mut stream = connector
        .connect("adventofcode.com", stream)
        .expect("Failed to connect to the server with TLS");

    let request = &format!(
        "\
         GET {} HTTP/1.0\r\n\
         Cookie: session={}\r\n\
         \r\n\
         ",
        path, session
    )
    .into_bytes();

    stream.write_all(request).unwrap();

    let reader = BufReader::new(stream);
    let mut lines = reader.lines().filter_map(Result::ok);
    let status = lines.next().expect("Empty response from server");

    if status.ends_with("404 Not Found") {
        panic!("Got a 404 for {}", path);
    }

    lines
        // Skip header lines
        .skip_while(|line| !line.is_empty())
        // Skip blank line
        .skip(1)
}

fn from_cache<T: FnOnce() -> Option<Vec<String>>>(
    name: &str,
    factory: T,
) -> Option<impl Iterator<Item = String>> {
    let cache_dir = Path::new("cache");
    fs::create_dir_all(cache_dir).expect("Failed to create cache dir");

    let file_cache_path = cache_dir.join(name);
    let file = match File::open(&file_cache_path) {
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            if let Some(result) = factory() {
                // Populate cache
                let mut file = fs::OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(&file_cache_path)
                    .expect("Failed to open cache file for writing");
                file.write_all(result.join("\n").as_bytes())
                    .expect("Failed to write cache");
                file.seek(io::SeekFrom::Start(0))
                    .expect("Failed to seek file cache");
                file
            } else {
                return None;
            }
        }
        Err(error) => panic!("Failed to open cache file: {}", error),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    Some(reader.lines().filter_map(Result::ok))
}

pub fn answer<T: std::str::FromStr>(year: u16, day: u8, puzzle: usize) -> Option<T> {
    from_cache(&format!("answer_{:02}_{}", day, puzzle), || {
        fetch_aoc(&format!("/{}/day/{}", year, day))
            .filter_map(|line| {
                line.split(|c| c == '<' || c == '>')
                    .skip_while(|s| s != &"Your puzzle answer was ")
                    .nth(2)
                    .map(std::string::ToString::to_string)
            })
            .nth(puzzle - 1)
            .map(|answer| vec![answer])
    })
    // Get the first line
    .and_then(|mut cache| cache.next())
    // Parse the number
    .and_then(|answer| answer.parse().ok())
}

pub fn input(year: u16, day: u8) -> impl Iterator<Item = String> {
    from_cache(&format!("input_{:02}", day), move || {
        Some(fetch_aoc(&format!("/{}/day/{}/input", year, day)).collect())
    })
    .unwrap()
}

pub fn example(year: u16, day: u8, example: usize) -> impl Iterator<Item = String> {
    from_cache(&format!("example_{:02}_{}", day, example), move || {
        fetch_aoc(&format!("/{}/day/{}", year, day))
            .scan((false, Vec::new()), |state, line| {
                let is_start = line.starts_with("<pre><code>");
                let is_end = line.ends_with("</code></pre>");

                if !state.0 && is_start {
                    state.0 = true;
                }

                if state.0 {
                    let start = if is_start { 11 } else { 0 };
                    let end = if is_end { line.len() - 13 } else { line.len() };
                    state.1.push(line[start..end].to_string());
                    if is_end {
                        state.0 = false;
                        return Some(Some(std::mem::replace(&mut state.1, Vec::new())));
                    }
                }

                return Some(None);
            })
            .flatten()
            .nth(example - 1)
    })
    .unwrap()
}
