mod words;
mod files;

const WORD_LENGTH : usize = 5;
const WORDLE_WORDS_FILE : &str = "./assets/valid_wordle_words.txt";

fn main() {
    
    let secret_word = files::get_random_wordle_word(WORDLE_WORDS_FILE);
    match secret_word {
        Ok(word) => println!("{word}"),
        Err(_) => (),
    }

    let user_word = words::get_valid_word(WORD_LENGTH);
    println!("{user_word}");
}
