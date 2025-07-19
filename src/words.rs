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
        LetterColor::Black => "\x1b[0m",
        LetterColor::Yellow => "\x1b[43m",
        LetterColor::Green => "\x1b[42m",
    }
}

pub fn grade_word(guess : &mut String, secret : &String) -> bool {

    for (i, c) in guess.as_str().chars().enumerate() {
        print!("{}",color_escape_sequence(letter_color(i, c, secret)));
        print!("{}", c);
    }

    // reset colors
    print!("{}",color_escape_sequence(LetterColor::Black));

    guess == secret
}
