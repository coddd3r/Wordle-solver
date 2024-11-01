use std::collections::HashMap;

use crate::{Correctness, Guess, Guesser, DICTIONARY};

pub struct Naive {
    remaining: HashMap<&'static str, usize>,
}

impl Naive {
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

impl Guesser for Naive {
    fn guess(&mut self, history: &[Guess]) -> String {
        println!("in naive calculating guess");
        let mut best: Option<Candidate> = None;
        if let Some(last) = history.last() {
            //filter out possibilities in self.remaining based on history of guesses
            self.remaining.retain(|&word, _| last.matches(word));
        }
        assert!(!self.remaining.is_empty());

        if history.is_empty() {
            return "tares".to_string();
        }
        let remaining_count: usize = self.remaining.iter().map(|(_, &c)| c).sum();

        //if we were to guess a word in the remaining candidates, what is the probability of getting each pattern
        //sum together probability to give a measure of the amout of information we would get from using the cadidate ass the next guess

        for (&word, _) in &self.remaining {
            let mut sum_of_probabilities = 0.0;
            //given all possible permutations of correctness
            for pattern in Correctness::patterns() {
                let mut in_pattern_total = 0;
                //if we guessed a word and got pattern, compute words that are left
                for (&candidate, count) in &self.remaining {
                    let g = Guess {
                        word: word.to_string(),
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

            let goodness = 0.0 - sum_of_probabilities;

            if let Some(c) = best {
                if goodness > c.goodness {
                    best = Some(Candidate { word, goodness })
                }
            } else {
                best = Some(Candidate { word, goodness });
            }
        }
        best.unwrap().word.to_string()
    }
}
