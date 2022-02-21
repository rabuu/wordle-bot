use crate::WORD_LENGTH;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Feedback {
    Purple,
    Yellow,
    Green,
}

impl Feedback {
    pub fn from_guess(word: &str, solution: &str) -> [Self; WORD_LENGTH] {
        assert_eq!(word.len(), WORD_LENGTH);
        assert_eq!(solution.len(), WORD_LENGTH);
        let n = word.len();

        use Feedback::*;

        let mut feedback = [Purple; WORD_LENGTH];
        let mut used = [false; WORD_LENGTH];
        for i in 0..n {
            if word.chars().nth(i).unwrap() == solution.chars().nth(i).unwrap() {
                feedback[i] = Green;
                used[i] = true;
            }
        }
        for i in 0..n {
            for j in 0..n {
                if word.chars().nth(i).unwrap() == solution.chars().nth(j).unwrap() && !used[j] {
                    feedback[i] = Yellow;
                    used[j] = true;
                }
            }
        }

        feedback
    }
}
