use std::{str::FromStr, usize};

// use clap::ArgEnum;
// use clap::ValueEnum;
use clap::Parser;

///setting up argumen tparser to enable us to choose whcih algorith we want to run with our binary
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    algo: Algorithm,
    #[clap(short, long)]
    max: Option<usize>,
}

#[derive(Debug)]
enum Algorithm {
    Naive,
    Allocs,
}

impl FromStr for Algorithm {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "naive" => Ok(Self::Naive),
            "allocs" => Ok(Self::Allocs),
            _ => Err(format!("don't have that algo implemented '{}'", s)),
        }
    }
}

const GAMES: &str = include_str!("../../answers.txt");
use solver::{algorithms::{Naive, Allocs}, Guesser, Wordle};
fn main() {
    let args = Args::parse();
    match args.algo {
        Algorithm::Naive => play(Naive::new, args.max),
        Algorithm::Allocs => play(Allocs::new, args.max)
    }
}

fn play<G>(mut mk: impl FnMut() -> G, max: Option<usize>)
where
    G: Guesser,
{
    let w = Wordle::new();
    for answer in GAMES.split_whitespace().take(max.unwrap_or(usize::MAX)) {
        let guesser = (mk)();
        println!("ANSWER AT START: {answer}");
        if let Some(score) = w.play(answer, guesser) {
            println!("score {score}");
        } else {
            eprintln!("FAILED TO GUESS");
        }
    }
}
