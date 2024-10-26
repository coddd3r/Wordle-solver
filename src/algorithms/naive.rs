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
    count: usize,
    goodness: f64,
}

impl Guesser for Naive {
    fn guess(&mut self, history: &[Guess]) -> String {
        let mut best: Option<Candidate> = None;
        if let Some(last) = history.last() {
            //filter out possibilities in self.remaining based on history of guesses
            self.remaining.retain(|word, _| last.matches(word));
        }
        
        
        for (&word, &count) in &self.remaining {
            let goodness = 0.0;
            if let Some(c) = best {
                if goodness > c.goodness {
                    best = Some(Candidate {
                        count,
                        word,
                        goodness,
                    })
                }
            } else {
                best = Some(Candidate {
                    word,
                    count,
                    goodness,
                });
            }
        }
        best.unwrap().word.to_string()
    }
}
