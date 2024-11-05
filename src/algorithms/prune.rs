/*do not reconsider patterns we've already eliminated */

use once_cell::sync::OnceCell;
use std::borrow::Cow;

use crate::{Correctness, Guess, Guesser, DICTIONARY};

static INITIAL: OnceCell<Vec<(&'static str, usize)>> = OnceCell::new();
static PATTERNS: OnceCell<Vec<[Correctness; 5]>> = OnceCell::new();
pub struct Prune {
    remaining: Cow<'static, Vec<(&'static str, usize)>>,
    patterns: Cow<'static, Vec<[Correctness; 5]>>,
}

impl Prune {
    pub fn new() -> Self {
        Self {
            remaining: Cow::Borrowed(INITIAL.get_or_init(|| {
                Vec::from_iter(DICTIONARY.lines().map(|line| {
                    let (q, b) = line.split_once(' ').unwrap();
                    (q, b.parse().expect("every count is a number"))
                }))
            })),
            patterns: Cow::Borrowed(PATTERNS.get_or_init(|| Correctness::patterns().collect())),
        }
    }
}
#[derive(Debug, Copy, Clone)]
struct Candidate {
    word: &'static str,
    goodness: f64,
}

impl Guesser for Prune {
    fn guess(&mut self, history: &[Guess]) -> String {
        //println!("in Prune calculating best guess...");

        if history.is_empty() {
            self.patterns = Cow::Borrowed(PATTERNS.get().unwrap());
            return "tares".to_string();
        } else {
            assert!(!self.patterns.is_empty())
        }

        if let Some(last) = history.last() {
            //filter out possibilities in self.remaining based on history of guesses
            //if the pointer is already owned then just take a mut
            if matches!(self.remaining, Cow::Owned(_)) {
                self.remaining
                    .to_mut()
                    .retain(|&(word, _)| last.matches(word));
            } else {
                self.remaining = Cow::Owned(
                    self.remaining
                        .iter()
                        .filter(|(word, _)| last.matches(word))
                        .copied()
                        .collect(),
                );
            }
        }
        //total count of all remaining words' frequencies
        let remaining_count: usize = self.remaining.iter().map(|&(_, c)| c).sum();

        //if we were to guess a word in the remaining candidates, what is the probability of getting each pattern
        //sum together probability to give a measure of the amout of information we would get from using the cadidate ass the next guess

        let mut best_candidate: Option<Candidate> = None;
        for &(word, count) in &*self.remaining {
            let mut sum_of_probabilities = 0.0;
            //given all possible permutations of correctness
            // for pattern in Correctness::patterns() {
            let check_pattern = |pattern: &[Correctness; 5]| {
                let mut in_pattern_total = 0;
                //if we guessed a word and got pattern, compute words that are left
                for &(candidate, count) in &*self.remaining {
                    let g = Guess {
                        word: Cow::Borrowed(word),
                        mask: *pattern,
                    };

                    if g.matches(candidate) {
                        in_pattern_total += count;
                    }
                }
                //count the total frequencies of words that would match this pattern divided by all remaining candidates
                if in_pattern_total == 0 {
                    return false;
                }
                let pattern_prob = in_pattern_total as f64 / remaining_count as f64;
                sum_of_probabilities += pattern_prob * pattern_prob.log2();
                return true;
            };

            if matches!(self.patterns, Cow::Owned(_)) {
                self.patterns.to_mut().retain(check_pattern)
            } else {
                self.patterns = Cow::Owned(
                    self.patterns
                        .iter()
                        .copied()
                        .filter(check_pattern)
                        .collect(),
                );
            }
            let word_probability = count as f64 / remaining_count as f64;
            let goodness = word_probability * -sum_of_probabilities;

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
