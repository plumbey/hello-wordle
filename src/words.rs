use std::io;

pub fn get_valid_word(desired_len : usize) -> String {

    loop {
        let mut s = String::new();

        match io::stdin().read_line(&mut s) {
            Err(_) => println!("Error reading word, please try again."),
            Ok(_) => (),
        }

        if s.trim().len() == desired_len {
            return s;
        }

        println!("Word length invalid. please try again.");
    };
}
