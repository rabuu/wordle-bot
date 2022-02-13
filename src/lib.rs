mod pattern;

use std::collections::HashSet;

use pattern::{Character, Pattern};

pub const WORD_LENGTH: usize = 5;

pub struct Bot<'a> {
    pub possible_solutions: HashSet<&'a str>,
    pub extra_guessing_options: HashSet<&'a str>,
    pub pattern: Pattern,
    pub count: usize,
}

impl<'a> Bot<'a> {
    pub fn new(
        possible_solutions: HashSet<&'a str>,
        extra_guessing_options: HashSet<&'a str>,
    ) -> Self {
        Bot {
            possible_solutions,
            extra_guessing_options,
            pattern: Pattern::default(),
            count: 1,
        }
    }

    pub fn insert_guess(&mut self, word: &str, feedback: [Feedback; WORD_LENGTH]) {
        assert_eq!(word.len(), WORD_LENGTH);

        for (i, c) in word.chars().enumerate() {
            match feedback[i] {
                Feedback::Gray => {
                    let excluded = self.pattern.excluded.get_or_insert(HashSet::new());
                    excluded.insert(c);
                }
                Feedback::Yellow => {
                    if let Character::Excluding(char_excluding) = &mut self.pattern.characters[i] {
                        let char_excluding = char_excluding.get_or_insert(HashSet::new());
                        char_excluding.insert(c);
                    }

                    let required = self.pattern.required.get_or_insert(HashSet::new());
                    required.insert(c);
                }
                Feedback::Green => {
                    self.pattern.characters[i] = Character::Known(c);
                }
            }
        }

        self.count += 1;
    }

    pub fn recommend_guesses(&self) -> HashSet<&'a str> {
        let mut recommendations = HashSet::new();
        for &possible_sol in &self.possible_solutions {
            if self.pattern.matches_word(possible_sol) {
                recommendations.insert(possible_sol);
            }
        }
        recommendations
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Feedback {
    Gray,
    Yellow,
    Green,
}
