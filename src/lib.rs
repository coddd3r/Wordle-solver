use std::collections::HashSet;

const DICTIONARY: &str = include_str!("../../dictionary.txt");

pub mod algorithms;

pub struct Wordle {
    dictionary: HashSet<&'static str>,
}

impl Wordle {
    pub fn new() -> Self {
        Self {
            dictionary: HashSet::from_iter(DICTIONARY.lines().map(|line| {
                line.split_once(' ')
                    .expect("every line shoul be word and count separated by space")
                    .0
            })),
        }
    }
    //take an answer and try guesses
    pub fn play<G: Guesser>(&self, answer: &'static str, mut guesser: G) -> Option<usize> {
        let mut history: Vec<Guess> = Vec::new();
        for i in 1..32 {
            let guess = guesser.guess(&history[..]);
            assert!(self.dictionary.contains(&*guess));
            if guess == answer {
                return Some(i);
            }

            let correctness = todo!();
            history.push(Guess {
                word: guess,
                mask: correctness,
            })
        }
        None
        // panic!("TOO MANY ATTEMPTS")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Correctness {
    Correct,
    Misplaced,
    Wrong,
}

impl Correctness {
    fn compute(answer: &str, guess: &str) -> [Self; 5] {
        println!("in compute answer:{}, quess:{}", answer, guess);
        assert_eq!(guess.len(), 5);
        let mut c = [Correctness::Wrong; 5];
        let mut used = [false; 5];

        //mark all correctly placed letters
        for (i, (a, g)) in answer.chars().zip(guess.chars()).enumerate() {
            if a == g {
                c[i] = Correctness::Correct;
                used[i] = true;
            }
        }

        //check for the yellow letters by checking if this letter is in the answer but not in this position
        for (i, g) in guess.chars().enumerate() {
            if used[i] {
                continue;
            }
            if answer.chars().enumerate().any(|(j, a)| {
                if a == g && !used[j] {
                    used[j] = true;
                    println!("checking guess char {g} at pos {i} FOUND MISPLACED at {j} char: {a}");
                    return true;
                };
                false
            }) {
                println!("IN HERE");
                c[i] = Correctness::Misplaced;
                println!("c after misplaced:{:?}", c);
            } else {
                c[i] = Correctness::Wrong;
            }
        }
        println!("returning C:{:?}", c);
        c
    }
}
pub struct Guess {
    word: String,
    mask: [Correctness; 5],
}
pub trait Guesser {
    fn guess(&mut self, history: &[Guess]) -> String;
}

// impl<F: Fn(&[Guess]) -> String> Guesser for F {
//     fn guess(&mut self, history: &[Guess]) -> String {
//         (*self)(history)
//     }
// }

#[cfg(test)]
macro_rules! guesser {
    (|$history:ident| $impl:block) => {{
        struct G;
        impl $crate::Guesser for G {
            fn guess(&mut self, $history: &[Guess]) -> String {
                $impl
            }
        }
        G
    }};
}

#[cfg(test)]
mod tests;
