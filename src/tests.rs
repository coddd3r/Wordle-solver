// use super::*;

mod tests {
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
        macro_rules! mask  {
            (C) => {Correctness::Correct};
            (M) => {Correctness::Misplaced};
            (W) => {Correctness::Wrong};
            ($($c:tt)+) => {[
                $(mask!($c)),+
            ]}
        }

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
        // #[test]
        // fn repeat_some() {
        //     assert_eq!(Correctness::compute("aabbb", "caacc"), mask![W C M W W])
        // }
    }
}
