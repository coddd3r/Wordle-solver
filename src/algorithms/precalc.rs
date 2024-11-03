use once_cell::sync::OnceCell;
use std::{borrow::Cow, collections::HashMap};

use crate::{Correctness, Guess, Guesser, DICTIONARY};

static INITIAL: OnceCell<Vec<(&'static str, usize)>> = OnceCell::new();
static MATCH_HISTORY: OnceCell<HashMap<(&'static str, &'static str, [Correctness; 5]), bool>> =
    OnceCell::new();

//remining does not need to be a map since we dont look into it
//initialize remaining exactly once using oncecell
//only set to owned when we start pruning
pub struct PreCalc {
    // remaining: HashMap<&'static str, usize>,
    // remaining: Vec<(&'static str, usize)>,
    remaining: Cow<'static, Vec<(&'static str, usize)>>,
}

impl PreCalc {
    pub fn new() -> Self {
        Self {
            remaining: Cow::Borrowed(INITIAL.get_or_init(|| {
                Vec::from_iter(DICTIONARY.lines().map(|line| {
                    let (q, b) = line.split_once(' ').unwrap();
                    (q, b.parse().expect("every count is a number"))
                }))
            })),
        }
    }
}
#[derive(Debug, Copy, Clone)]
struct Candidate {
    word: &'static str,
    goodness: f64,
}

impl Guesser for PreCalc {
    fn guess(&mut self, history: &[Guess]) -> String {
        if history.is_empty() {
            return "tares".to_string();
        }

        if let Some(last) = history.last() {
            //filter out possibilities in self.remaining based on history of guesses
            //if the pointer is laready owned then just take a mut
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
        for &(word, _) in &*self.remaining {
            let mut sum_of_probabilities = 0.0;
            //given all possible permutations of correctness
            for pattern in Correctness::patterns() {
                let mut in_pattern_total = 0;
                //if we guessed a word and got pattern, compute words that are left
                for &(candidate, count) in &*self.remaining {
                    let matches = MATCH_HISTORY.get_or_init(|| {
                        let words = INITIAL.get().unwrap();
                        let patterns = Correctness::patterns();
                        let mut out: HashMap<(&str, &str, [Correctness; 5]), bool> =
                            HashMap::with_capacity(
                                (words.len() * words.len() * patterns.count()) / 2,
                            );
                        for &(word1, _) in INITIAL.get().unwrap() {
                            for &(word2, _) in INITIAL.get().unwrap() {
                                for pattern in Correctness::patterns() {
                                    if word2 < word1 {
                                        break;
                                    }
                                    let g = Guess {
                                        word: Cow::Borrowed(word),
                                        mask: pattern,
                                    };
                                    out.insert((word1, word2, pattern), g.matches(candidate));
                                }
                            }
                        }
                        out
                    });
                    let key = if word < candidate {
                        (word, candidate, pattern)
                    } else {
                        (candidate, word, pattern)
                    };
                    if *matches.get(&key).unwrap() {
                        in_pattern_total += count;
                    };
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
