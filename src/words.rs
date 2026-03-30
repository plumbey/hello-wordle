use rand::random_range;
use std::io;
use std::io::Error;

enum LetterColor {
    Default,
    Yellow,
    Green,
    Reset,
    FGBlack,
}

pub struct WordleGame {
    max_guesses: u8,
    other_words: &'static str,
    common_words: &'static str,
}

impl WordleGame {
    pub fn new(other_words: &'static str, common_words: &'static str) -> WordleGame {
        WordleGame {
            max_guesses: 6,
            other_words: other_words,
            common_words: common_words,
        }
    }

    pub fn play(&mut self) -> bool {
        println!("Welcome to Wordle");
        let secret_word = match self.get_random_wordle_word() {
            Ok(word) => word,
            Err(_) => return false,
        };

        for i in 0..self.max_guesses {
            println!("Guess {}:", i + 1);

            let user_word = self.get_valid_word();
            println!();

            if self.grade_word(&user_word, &secret_word) {
                println!("You won! The word was {secret_word}");
                return true;
            }
        }

        println!("You lost :( The word was {secret_word}");
        return false;
    }

    fn get_random_wordle_word(&mut self) -> Result<String, Error> {
        let line_count = self.common_words.lines().count();

        let rand_line_num = random_range(0..line_count);

        match self.common_words.lines().nth(rand_line_num) {
            Some(word) => Ok(word.to_string()),
            None => Err(Error::other("Error, could not find word in file {path}!")),
        }
    }

    fn grade_word(&self, guess: &str, secret: &str) -> bool {
        // Make the text dark
        print!(
            "{}",
            WordleGame::color_escape_sequence(LetterColor::FGBlack)
        );

        for (i, c) in guess.chars().enumerate() {
            print!(
                "{}",
                WordleGame::color_escape_sequence(WordleGame::letter_color(i, c, secret))
            );
            print!("{c}");
        }

        // reset colors
        print!("{}", WordleGame::color_escape_sequence(LetterColor::Reset));
        println!();

        guess == secret
    }

    fn get_valid_word(&self) -> String {
        loop {
            let mut s = String::new();

            if io::stdin().read_line(&mut s).is_err() {
                println!("Error reading word, please try again.")
            }

            if s.len() == 6
                && (self.common_words.find(&s) != None || self.other_words.find(&s) != None)
            {
                return s.trim().to_string();
            }

            println!("Invalid word. Please try again");
        }
    }

    fn letter_color(i: usize, c: char, secret: &str) -> LetterColor {
        if secret.as_bytes()[i] as char == c {
            return LetterColor::Green;
        } else if secret.find(c) != None {
            return LetterColor::Yellow;
        };

        LetterColor::Default
    }

    fn color_escape_sequence(color: LetterColor) -> &'static str {
        match color {
            LetterColor::Default => "\x1b[49m",
            LetterColor::Yellow => "\x1b[43m",
            LetterColor::Green => "\x1b[42m",
            LetterColor::FGBlack => "\x1b[30m",
            LetterColor::Reset => "\x1b[0m",
        }
    }
}
