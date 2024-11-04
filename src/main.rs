use clap::Parser;
use std::{str::FromStr, usize};

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
    VecRem,
    InitOnce,
    PreCalc,
}

impl FromStr for Algorithm {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "naive" => Ok(Self::Naive),
            "allocs" => Ok(Self::Allocs),
            "vecrem" => Ok(Self::VecRem),
            "initonce" => Ok(Self::InitOnce),
            "precalc" => Ok(Self::PreCalc),
            _ => Err(format!("don't have that algo implemented '{}'", s)),
        }
    }
}

const GAMES: &str = include_str!("../source_txt/answers.txt");
use solver::{
    algorithms::{Allocs, InitOnce, Naive, PreCalc, VecRem},
    Guesser, Wordle,
};
fn main() {
    let args = Args::parse();
    match args.algo {
        Algorithm::Naive => play(Naive::new, args.max),
        Algorithm::Allocs => play(Allocs::new, args.max),
        Algorithm::VecRem => play(VecRem::new, args.max),
        Algorithm::InitOnce => play(InitOnce::new, args.max),
        Algorithm::PreCalc => play(PreCalc::new, args.max),
    }
}

fn play<G>(mut maker_function: impl FnMut() -> G, max: Option<usize>)
where
    G: Guesser,
{
    let w = Wordle::new();
    for answer in GAMES.split_whitespace().take(max.unwrap_or(usize::MAX)) {
        let guesser = (maker_function)();
        println!("ANSWER AT START: {answer}");
        if let Some(score) = w.play(answer, guesser) {
            println!("score {score}");
        } else {
            eprintln!("FAILED TO GUESS");
        }
    }
}
