// use super::*;

macro_rules! mask  {
    (C) => {Correctness::Correct};
    (M) => {Correctness::Misplaced};
    (W) => {Correctness::Wrong};
    ($($c:tt)+) => {[
        $(mask!($c)),+
    ]}
}
mod tests {

    // #[cfg(test)]
    // mod matches {

    // }
    // #[cfg(test)]
    mod guess_filter {
        use crate::Correctness;
        use crate::Guess;
        use std::borrow::Cow;

        macro_rules! check {
            ($prev:literal + [$($mask:tt)+] allows $next:literal) => {
                assert!(Guess {
                    // word: $prev.to_string(),
                    word: Cow::Borrowed($prev),
                    mask: mask!($($mask)+)
                }
                .matches($next))
            };
            ($prev:literal + [$($mask:tt)+] disallows $next:literal) => {
                assert!(!Guess {
                    // word: $prev.to_string(),
                    word: Cow::Borrowed($prev),
                    mask: mask!($($mask)+)
                }
                .matches($next))
            };
        }
        #[test]
        fn matches() {
            check!("abcde" + [C C C C C] allows "abcde");
            check!("abcdf" + [C C C C C] disallows "abcde");
            check!("eabcd" + [M M M M M] disallows "eacde");
        }

        #[test]
        fn partial() {
            check!("abcfg" + [C C C W W] allows "abcde");
            check!("eabcd" + [M M M M M] allows "abcde");
            check!("aaabb" + [C M W W W] disallows "accaa");
            check!("baaaa" + [W C M W W] allows "aaccc");
            check!("baaaa" + [W C M W W] disallows "caacc");
            check!("tares" + [W M M W W] disallows "brink");
        }
        #[test]

        fn all_wrong() {
            check!("abcde" + [W W W W W] allows "ghijk");
            check!("abcde" + [W W W W W] disallows "eabcd");
        }
        #[test]
        fn from_crash() {
            check!("tares" + [W M M W W] disallows "brink");
            check!("tares" + [W M M W W] disallows "rural");
            check!("tares" + [W M M W W] disallows "arrah");
            check!("tares" + [W M M W W] disallows "heapy");
            check!("tares" + [W M M W W] allows "flora");
            check!("tares" + [W M M W W] allows "araba");
            check!("praam" + [C C M W W] disallows "prial");
            // check!("urari" + [])            
        }
    }

    mod game {
        use crate::Guess;
        use crate::Wordle;
        #[test]
        fn genius() {
            let w = Wordle::new();
            assert_eq!(
                w.play("moved", |_history: &[Guess]| "moved".to_string()),
                Some(1)
            );
        }

        #[test]
        fn magnificent() {
            let w = Wordle::new();
            assert_eq!(
                w.play("moved", |_history: &[Guess]| {
                    if _history.len() == 1 {
                        return "moved".to_string();
                    }
                    return "wrong".to_string();
                }),
                Some(2)
            );
        }
        #[test]
        fn impressive() {
            let w = Wordle::new();

            assert_eq!(
                w.play("moved", |_history: &[Guess]| {
                    if _history.len() == 2 {
                        return "moved".to_string();
                    }
                    return "wrong".to_string();
                }),
                Some(3)
            );
        }
        #[test]
        fn splendid() {
            let w = Wordle::new();
            assert_eq!(
                w.play("moved", |_history: &[Guess]| {
                    if _history.len() == 3 {
                        return "moved".to_string();
                    }
                    return "wrong".to_string();
                }),
                Some(4)
            );
        }
        #[test]
        fn great() {
            let w = Wordle::new();
            assert_eq!(
                w.play("moved", |_history: &[Guess]| {
                    if _history.len() == 4 {
                        return "moved".to_string();
                    }
                    return "wrong".to_string();
                }),
                Some(5)
            );
        }
        #[test]
        fn phew() {
            let w = Wordle::new();
            assert_eq!(
                w.play("moved", |_history: &[Guess]| {
                    if _history.len() == 5 {
                        return "moved".to_string();
                    }
                    return "wrong".to_string();
                }),
                Some(6)
            );
        }
        #[test]
        fn always_wrong() {
            let w = Wordle::new();
            assert_eq!(
                w.play("moved", |_history: &[Guess]| "wrong".to_string()),
                None
            );
        }
    }

    mod compute {
        use crate::Correctness;
        #[test]
        fn all_green() {
            assert_eq!(Correctness::compute("abcde", "abcde"), mask!(C C C C C))
        }

        #[test]
        fn all_grey() {
            assert_eq!(Correctness::compute("abcde", "gfihs"), mask![W W W W W])
        }

        #[test]
        fn repeat_green() {
            assert_eq!(Correctness::compute("aacde", "aajik"), mask![C C W W W])
        }

        #[test]
        fn repeat_yellow() {
            assert_eq!(Correctness::compute("aabbb", "ccaac"), mask![W W M M W])
        }

        #[test]
        fn repeat_some() {
            assert_eq!(Correctness::compute("aabbb", "caacc"), mask![W C M W W])
        }
        #[test]
        fn extra1() {
            assert_eq!(Correctness::compute("azzaz", "aaabb"), mask![C M W W W])
        }
        #[test]
        fn extra2() {
            assert_eq!(Correctness::compute("baccc", "aaddd"), mask![W C W W W])
        }
        #[test]
        fn extra3() {
            assert_eq!(Correctness::compute("abcde", "aacde"), mask![C W C C C])
        }
        #[test]
        fn debug_compute() {
            assert_eq!(Correctness::compute("cigar", "braai"), mask![W M W C M]);
            assert_eq!(Correctness::compute("tares", "rural"), mask![W W C M W]);
            // assert_eq!(Correctness::compute_faster("tares", "rural"), mask![W W C M W]);
        }
        // #[test]
        // fn repeat_some() {
        //     assert_eq!(Correctness::compute("aabbb", "caacc"), mask![W C M W W])
        // }
    }
}
