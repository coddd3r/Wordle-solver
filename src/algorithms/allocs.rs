use std::{borrow::Cow, collections::HashMap};

use crate::{Correctness, Guess, Guesser, DICTIONARY};

pub struct Allocs {
    remaining: HashMap<&'static str, usize>,
}

impl Allocs {
    pub fn new() -> Self {
        Self {
            remaining: HashMap::from_iter(DICTIONARY.lines().map(|line| {
                let (q, b) = line.split_once(' ').unwrap();
                (q, b.parse().expect("every count is a number"))
            })),
        }
    }
}
#[derive(Debug, Copy, Clone)]
struct Candidate {
    word: &'static str,
    goodness: f64,
}

impl Guesser for Allocs {
    fn guess(&mut self, history: &[Guess]) -> String {
        println!("in Allocs calculating best guess...");

        if history.is_empty() {
            return "tares".to_string();
        }

        if let Some(last) = history.last() {
            //filter out possibilities in self.remaining based on history of guesses
            self.remaining.retain(|&word, _| last.matches(word));
        }
        //total count of all remaining words' frequencies
        let remaining_count: usize = self.remaining.iter().map(|(_, &c)| c).sum();

        //if we were to guess a word in the remaining candidates, what is the probability of getting each pattern
        //sum together probability to give a measure of the amout of information we would get from using the cadidate ass the next guess

        let mut best_candidate: Option<Candidate> = None;
        for (&word, _) in &self.remaining {
            let mut sum_of_probabilities = 0.0;
            //given all possible permutations of correctness
            for pattern in Correctness::patterns() {
                let mut in_pattern_total = 0;
                //if we guessed a word and got pattern, compute words that are left
                for (&candidate, count) in &self.remaining {
                    let g = Guess {
                        word: Cow::Borrowed(word),
                        mask: pattern,
                    };

                    if g.matches(candidate) {
                        in_pattern_total += count;
                    }
                }
                //count the total frequencies of words that would match this pattern divided by all remaining candidates
                if in_pattern_total == 0 {
                    continue;
                }
                let pattern_prob = in_pattern_total as f64 / remaining_count as f64;
                sum_of_probabilities += pattern_prob * pattern_prob.log2();
            }

            let goodness = -sum_of_probabilities;

            if let Some(c) = best_candidate {
                if goodness > c.goodness {
                    best_candidate = Some(Candidate { word, goodness })
                }
            } else {
                best_candidate = Some(Candidate { word, goodness });
            }
        }
        best_candidate.unwrap().word.to_string()
    }
}
