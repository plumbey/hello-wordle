use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use rand::random_range;

pub fn get_random_wordle_word(path : &str) -> Result<String, Error> {
    let line_count = file_line_count(path)?;

    let file = File::open(path)?;

    let reader = BufReader::new(file);

    let rand_line_num = random_range(0..line_count);

    match reader.lines().nth(rand_line_num) {
        Some(word) => return word,
        None => return Err(
            Error::new(
                ErrorKind::Other,
                "Error, could not find word in file {path}!"
                )),
    }
}

fn file_line_count(path : &str) -> Result<usize, Error> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    Ok(reader.lines().count())
}
