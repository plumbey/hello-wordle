use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use rand::random_range;

pub fn get_random_wordle_word(path : &str) -> Result<String, Error> {
    let line_count = file_line_count(path)?;

    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let rand_line_num = random_range(0..line_count);

    let mut line_count = 0;

    for line in reader.lines() {
        if line_count == rand_line_num {
            match line {
                Ok(line) => return Ok(line),
                Err(e) => return Err(e),
            }
        }
        line_count += 1;
    }

    Err(Error::new(ErrorKind::Other, "Could not find word in file {path}!"))
}

fn file_line_count(path : &str) -> Result<usize, Error> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let mut line_count = 0;

    for _ in reader.lines() {
        line_count += 1;
    }

    Ok(line_count)
}
