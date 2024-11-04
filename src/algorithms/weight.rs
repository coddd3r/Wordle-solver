/*
    calculate the prob of word among remaining candidates and add to goodness measure
*/

use once_cell::sync::OnceCell;
use std::{borrow::Cow, collections::BTreeMap};

use crate::{Correctness, Guess, Guesser, DICTIONARY};

static INITIAL: OnceCell<Vec<(&'static str, usize)>> = OnceCell::new();
//store match history in a Hashmap
static MATCH_HISTORY: OnceCell<BTreeMap<(&'static str, &'static str, [Correctness; 5]), bool>> =
    OnceCell::new();

pub struct Weight {
    remaining: Cow<'static, Vec<(&'static str, usize)>>,
}

impl Weight {
    pub fn new() -> Self {
        Self {
            remaining: Cow::Borrowed(INITIAL.get_or_init(|| {
                let mut words = Vec::from_iter(DICTIONARY.lines().map(|line| {
                    let (q, b) = line.split_once(' ').unwrap();
                    (q, b.parse().expect("every count is a number"))
                }));
                words.sort_unstable_by_key(|&(_, count)| std::cmp::Reverse(count));
                words
            })),
        }
    }
}
#[derive(Debug, Copy, Clone)]
struct Candidate {
    word: &'static str,
    goodness: f64,
}

impl Guesser for Weight {
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
        for &(word, count) in &*self.remaining {
            let mut sum_of_probabilities = 0.0;
            //given all possible permutations of correctness
            for pattern in Correctness::patterns() {
                let mut in_pattern_total = 0;
                //if we guessed a word and got pattern, compute words that are left
                for &(candidate, count) in &*self.remaining {
                    let matches_hist = MATCH_HISTORY.get_or_init(|| {
                        const MAX_SIZE: usize = 256;
                        let words = &INITIAL.get().unwrap()[..MAX_SIZE];
                        // let patterns = Correctness   ::patterns();
                        let mut out = BTreeMap::new();
                        for &(word1, _) in words {
                            for &(word2, _) in words {
                                if word2 < word1 {
                                    break;
                                }
                                for pattern in Correctness::patterns() {
                                    let g = Guess {
                                        word: Cow::Borrowed(word1),
                                        mask: pattern,
                                    };
                                    out.insert((word1, word2, pattern), g.matches(word2));
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
                    if matches_hist.get(&key).copied().unwrap_or_else(|| {
                        let g = Guess {
                            word: Cow::Borrowed(word),
                            mask: pattern,
                        };
                        g.matches(candidate)
                    }) {
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

            let word_probability = count as f64/ remaining_count as f64;
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
