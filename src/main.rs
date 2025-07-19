mod words;
mod files;

const WORD_LENGTH : usize = 5;
const MAX_GUESSES : usize = 6;
const WORDLE_WORDS : &str = 
    include_str!("assets/valid_wordle_words.txt");

fn main() {

    println!("Welcome to Wordle");
    let secret_word = match files::get_random_wordle_word(WORDLE_WORDS) {
        Ok(word) => word,
        Err(_) => return,
    };
    
    for i in 0..MAX_GUESSES {
        println!("Guess {}:", i + 1);

        let mut user_word = words::get_valid_word(WORD_LENGTH);
        println!("");

        if words::grade_word(&mut user_word, &secret_word) {
            println!("You won! The word was {secret_word}");
            return;
        }
    }
    
    println!("You lost :( The word was {secret_word}");
}
