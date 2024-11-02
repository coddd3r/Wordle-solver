use std::{borrow::Cow, collections::HashSet};

const DICTIONARY: &str = include_str!("../dictionary.txt");

pub mod algorithms;

pub struct Wordle {
    dictionary: HashSet<&'static str>,
}

impl Wordle {
    pub fn new() -> Self {
        Self {
            dictionary: HashSet::from_iter(DICTIONARY.lines().map(|line| {
                line.split_once(' ')
                    .expect("every line should be word and count separated by space")
                    .0
            })),
        }
    }
    //take an answer and try guesses
    pub fn play<G: Guesser>(&self, answer: &'static str, mut guesser: G) -> Option<usize> {
        println!("IN WORDLE playing with answer:{answer}");
        // assert!(self.dictionary.contains(answer));
        // if self.dictionary.contains(answer) {
        //     // println!("ANSWER PRESENT");
        // }
        let mut history: Vec<Guess> = Vec::new();
        for i in 1..32 {
            let guess = guesser.guess(&history[..]);
            assert!(self.dictionary.contains(&*guess));
            if guess == answer {
                println!("FOUND ANSWER {guess} {answer}");
                return Some(i);
            }

            let correctness = Correctness::compute(answer, &guess);
            println!("answer:{answer}, guess:{guess}, correctness: {correctness:?}");
            history.push(Guess {
                word: Cow::Owned(guess),
                mask: correctness,
            })
        }
        None
        // panic!("TOO MANY ATTEMPTS")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Correctness {
    Correct,
    Misplaced,
    Wrong,
}

impl Correctness {
    fn compute(answer: &str, guess: &str) -> [Self; 5] {
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
            //already marked correct
            if c[i] == Correctness::Correct {
                continue;
            }
            if answer.chars().enumerate().any(|(j, a)| {
                if a == g && !used[j] {
                    used[j] = true;
                    return true;
                };
                false
            }) {
                c[i] = Correctness::Misplaced;
            } else {
                c[i] = Correctness::Wrong;
            }
        }
        c
    }

    //generate all possible permutations of length 5
    pub fn patterns() -> impl Iterator<Item = [Correctness; 5]> {
        itertools::iproduct!(
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong]
        )
        .map(|(a, b, c, d, e)| [a, b, c, d, e])
    }
}

#[derive(Debug)]
pub struct Guess<'a> {
    // word: String,
    word: Cow<'a, str>,
    mask: [Correctness; 5],
}

impl Guess<'_> {
    pub fn matches(&self, word: &str) -> bool {
        //check greens
        // println!("'prev guess{}  curr guess{}", self.word, word);
        // if self.word == word {
        //     return true;
        // }
        assert_eq!(self.word.len(), 5);
        assert_eq!(word.len(), 5);
        let mut used = [false; 5];
        // let yellows = self.mask;
        //MARK ALL CORECCTLY PLACED CHARS FIRST
        for (i, ((g, &m), w)) in self
            .word
            .bytes()
            .zip(&self.mask)
            .zip(word.bytes())
            .enumerate()
        {
            if m == Correctness::Correct {
                if g != w {
                    return false;
                }
                used[i] = true;
            }
        }

        for (i, (w, m)) in word.bytes().zip(&self.mask).enumerate() {
            if *m == Correctness::Correct {
                //already evaluated in loop above
                continue;
            }
            let mut plausible = true;
            //if this letter occurs in the previous guess and was marked as misplaced
            if self
                .word
                .bytes()
                .zip(&self.mask)
                .enumerate()
                .any(|(j, (g, m))| {
                    //if char of guess does not match the char from word being checked
                    if g != w || used[j] {
                        return false;
                    };
                    //we're looking for a char 'w' in 'word, and have found a char 'w' in previous guess;
                    //it's colour in the previous geuss shoudl tell us if the current char might be okay
                    match m {
                        Correctness::Correct => unreachable!("all correct guesses are used"),
                        Correctness::Misplaced => {
                            // println!("matching misplaced");
                            //if w was misplaced int he same position in the prev guess, this curr possible 'word' cannot be the solution word
                            if j == i {
                                plausible = false;
                                used[j] = true;
                                return false;
                            }
                            used[j] = true;
                            return true;
                        }
                        Correctness::Wrong => {
                            plausible = false;
                            return false;
                        }
                    }
                })
                && plausible
            {
                assert!(plausible);
            } else if !plausible {
                return false;
            }
        }
        for (i, &m) in self.mask.iter().enumerate() {
            if m == Correctness::Misplaced && !used[i] {
                return false;
            }
        }
        true
    }
}

pub trait Guesser {
    fn guess(&mut self, history: &[Guess]) -> String;
}

impl<F: Fn(&[Guess]) -> String> Guesser for F {
    fn guess(&mut self, history: &[Guess]) -> String {
        (*self)(history)
    }
}

// #[cfg(test)]
// macro_rules! guesser {
//     (|$history:ident| $impl:block) => {{
//         struct G;
//         impl $crate::Guesser for G {
//             fn guess(&mut self, $history: &[Guess]) -> String {
//                 $impl
//             }
//         }
//         G
//     }};
// }
#[cfg(test)]
mod tests;
