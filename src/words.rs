use std::io;

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
