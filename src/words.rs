use std::io;

enum LetterColor {
    Black,
    Yellow,
    Green,
}

pub fn get_valid_word(desired_len : usize) -> String {
    loop {
        let mut s = String::new();

        if io::stdin().read_line(&mut s).is_err() { println!("Error reading word, please try again.") }

        if s.trim().len() == desired_len {
            return s;
        }

        println!("Word length invalid. please try again.");
    };
}

fn letter_color(i : usize, c : char, secret : &String) -> LetterColor {
    match secret.find(c) {
        Some(ind) => 
            if ind == i {LetterColor::Green} else {LetterColor::Yellow},
        None => LetterColor::Black,
    }
}

fn color_escape_sequence(color : LetterColor) -> &'static str {
    match color {
        LetterColor::Black => "\\e[0m",
        LetterColor::Yellow => "\\e[43m",
        LetterColor::Green => "\\e[42m",
    }
}

fn grade_word(guess : &mut String, secret : &String) -> bool {

    for (i, c) in secret.as_str().chars().enumerate() {
        guess.insert_str(
            i, 
            color_escape_sequence(letter_color(i, c, secret))
            );
    }

    // reset colors
    guess.insert_str(
        guess.len() - 1, 
        color_escape_sequence(LetterColor::Black)
        );

    guess == secret
}
