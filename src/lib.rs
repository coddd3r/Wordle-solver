use std::{borrow::Cow, collections::HashSet};

const DICTIONARY: &str = include_str!("../source_txt/dictionary.txt");

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
        let mut history: Vec<Guess> = Vec::new();
        for i in 1..32 {
            let guess = guesser.guess(&history[..]);
            assert!(self.dictionary.contains(&*guess));
            if guess == answer {
                println!("FOUND ANSWER {guess} {answer}");
                return Some(i);
            }

            let correctness = Correctness::compute(answer, &guess);
            println!("IN PLAY answer:{answer}, guess:{guess}, correctness: {correctness:?}");
            history.push(Guess {
                word: Cow::Owned(guess),
                mask: correctness,
            })
        }
        None
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
        assert_eq!(answer.len(), 5);
        let mut c = [Correctness::Wrong; 5];
        let mut used = [false; 5];

        //mark all correctly placed letters
        for (i, (a, g)) in answer.bytes().zip(guess.bytes()).enumerate() {
            if a == g {
                c[i] = Correctness::Correct;
                used[i] = true;
            }
        }
        //check for the yellow letters by checking if this letter is in the answer but not in this position
        for (i, g) in guess.bytes().enumerate() {
            //already marked correct
            if c[i] == Correctness::Correct {
                continue;
            }
            if answer.bytes().enumerate().any(|(j, a)| {
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
    //is faster than old matches with no double yellow check
    // slower than matches with double yellow check
    pub fn matches(&self, word: &str) -> bool {
        Correctness::compute(word, &self.word) == self.mask
    }

    pub fn matches_faster(&self, word: &str) -> bool {
        assert_eq!(self.word.len(), 5);
        assert_eq!(word.len(), 5);

        let mut used = [false; 5];
        //MARK ALL CORECCTLY PLACED CHARS FIRST
        for (i, ((g, &m), w)) in self
            .word
            .bytes()
            .zip(&self.mask)
            .zip(word.bytes())
            .enumerate()
        {
            // if the prev guess was correct at this position but the letters differ then this candidate does not match
            match m {
                Correctness::Correct => {
                    if g != w {
                        return false;
                    }
                    used[i] = true;
                }
                // if wrong or misplaced in previous guess and is the same letter at the same position in candidate, then no match
                _ => {
                    if g == w {
                        return false;
                    }
                }
            }
        }

        for (i, (w, &w_m)) in word.bytes().zip(&self.mask).enumerate() {
            if w_m == Correctness::Correct {
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
                .any(|(j, (g, &m))| {
                    //if char of guess does not match the char from word being checked
                    // println!("in word index:{i} letter:{w} self index:{j} letter:{g}");
                    if g != w || m == Correctness::Correct || used[j] {
                        return false;
                    };

                    //we're looking for a char 'w' in 'word, and have found a char 'w' in previous guess;
                    //it's colour in the previous geuss shoudl tell us if the current char might be okay
                    match m {
                        Correctness::Correct => unreachable!("all correct guesses are used"),
                        Correctness::Misplaced => {
                            // println!("matching misplaced, self.word letter:{} j:{j} word letter:{} i:{i}", g as char, w as char);
                            //if w was misplaced int he same position in the prev guess, this curr possible 'word' cannot be the solution word
                            if j == i {
                                unreachable!();
                            }
                            //if misplaced but at different positions, could be true
                            //println!("setting used true in misplaced at self index:{j}, word index:{i} self letter: {}, word letter :{}", g as char, w as char);
                            used[j] = true;
                            return true;
                        }
                        Correctness::Wrong => {
                            if j == i {
                                unreachable!()
                            }
                            //letter cannot be present anywhere
                            plausible = false;
                            return false;
                        }
                    }
                })
                && plausible
            {
                assert!(plausible);
            } else if !plausible {
                //println!("returning false");
                return false;
            }
        }

        //TODO: Always uncomment
        //NOTE: this part if a fix to make sure there are no unused yellows in any potential candidate
        //Faster than using correctness compute and way faster than the matches without it
        for (j, &m) in self.mask.iter().enumerate() {
            if m == Correctness::Misplaced && !used[j] {
                //println!("returning false in EXTRA LOOP");
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
