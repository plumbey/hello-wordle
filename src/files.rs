use std::io::Error;
use rand::random_range;

pub fn get_random_wordle_word(words : &str) -> Result<String, Error> {
    let line_count = words.lines().count();

    let rand_line_num = random_range(0..line_count);

    match words.lines().nth(rand_line_num) {
        Some(word) => Ok(word.to_string()),
        None => Err(
            Error::other(
                "Error, could not find word in file {path}!"
                )),
    }
}
