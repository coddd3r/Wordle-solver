const GAMES: &str = include_str!("../../answers.txt");

use solver::{algorithms::Naive, Wordle};
fn main() {
    let w = Wordle::new();
    for answer in GAMES.split_whitespace() {
        let guesser = Naive::new();
        w.play(answer, guesser);
    }
}
// enum Correctness {
//     Correct,
//     Misplaced,
//     Wrong,
// }

// struct Guess {
//     word: String,
//     mask: [Correctness; 5],
// }
// trait Guesser {
//     fn guess(&mut self, history: &[Guess]) -> String {}
// }

// //take an answer and try guesses
// fn play<G: Guesser>(answer: &'static str, guesser: G) {}
