mod words;
mod files;

const WORD_LENGTH : usize = 5;
const WORDLE_WORDS : &str = 
    include_str!("assets/valid_wordle_words.txt");

fn main() {
    let secret_word = files::get_random_wordle_word(WORDLE_WORDS);
    if let Ok(word) = secret_word { println!("{word}") }

    let user_word = words::get_valid_word(WORD_LENGTH);
    println!("{user_word}");
}
