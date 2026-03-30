mod words;

const VALID_WORDS: &'static str = include_str!("assets/other-words.txt");
const COMMON_WORDS: &'static str = include_str!("assets/common-words.txt");

fn main() {
    let mut wordle = words::WordleGame::new(VALID_WORDS, COMMON_WORDS);
    wordle.play();
}
