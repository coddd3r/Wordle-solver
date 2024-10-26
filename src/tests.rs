// use super::*;

mod tests {

    mod game {
        use crate::Guess;
        use crate::Wordle;
        #[test]
        fn one_go() {
            let w = Wordle::new();
            // assert_eq!(w.play("moved",|_history: &[Guess]| "moved".to_string()), Some(1));
            let guesser = guesser!(|_history| { "moved".to_string() });
            assert_eq!(w.play("moved", guesser), Some(1));
        }

        #[test]
        fn two_go() {
            let w = Wordle::new();
            // assert_eq!(w.play("moved",|_history: &[Guess]| "moved".to_string()), Some(1));
            let guesser = guesser!(|_history| { "moved".to_string() });
            assert_eq!(w.play("moved", guesser), Some(1));
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
