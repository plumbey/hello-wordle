use std::io;

pub fn get_valid_word(desired_len : usize) -> String {
    let mut s = String::new();

    loop {
        match io::stdin().read_line(&mut s) {
            Err(_) => println!("Error reading word, please try again."),
            Ok(_) => (),
        }

        s = s.trim().to_string();

        if s.len() == desired_len {
            break;
        }

        println!("Word length invalid. please try again.");

    };

    s
}
