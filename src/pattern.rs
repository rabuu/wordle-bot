use std::collections::HashSet;

use crate::Feedback;
use crate::WORD_LENGTH;

#[derive(Debug, Clone, Default)]
pub struct Pattern {
    pub characters: [Character; WORD_LENGTH],
    pub required: HashSet<char>,
}

impl Pattern {
    pub fn matches_word(&self, word: &str) -> bool {
        assert_eq!(word.len(), WORD_LENGTH);

        for required_char in &self.required {
            if !word.contains(*required_char) {
                return false;
            }
        }

        for (i, character) in self.characters.iter().enumerate() {
            let c = word.chars().nth(i).unwrap();
            match character {
                Character::Known(known_char) => {
                    if c != *known_char {
                        return false;
                    }
                }
                Character::Unknown(excluding) => {
                    for excluding_char in excluding.iter() {
                        if c == *excluding_char {
                            return false;
                        }
                    }
                }
            }
        }

        true
    }

    pub fn insert_guess(&mut self, word: &str, feedback: [Feedback; WORD_LENGTH]) {
        assert_eq!(word.len(), WORD_LENGTH);

        for (i, fb) in feedback.into_iter().enumerate() {
            let c = word.chars().nth(i).unwrap();
            match fb {
                Feedback::Gray => {
                    if !self.required.contains(&c) {
                        for character in &mut self.characters {
                            if let Character::Unknown(excluding) = character {
                                excluding.insert(c);
                            }
                        }
                    } else if let Character::Unknown(excluding) = &mut self.characters[i] {
                        excluding.insert(c);
                    }
                }
                Feedback::Yellow => {
                    self.required.insert(c);
                    if let Character::Unknown(excluding) = &mut self.characters[i] {
                        excluding.insert(c);
                    }
                }
                Feedback::Green => {
                    self.characters[i] = Character::Known(c);
                }
            }
        }
    }

    pub fn matching_probability(&self, words: &[&str]) -> f64 {
        words.iter().filter(|w| self.matches_word(w)).count() as f64 / words.len() as f64
    }
}

#[derive(Debug, Clone)]
pub enum Character {
    Known(char),
    Unknown(HashSet<char>),
}

impl Default for Character {
    fn default() -> Self {
        Character::Unknown(HashSet::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pattern_01() {
        let mut pattern = Pattern::default();

        use Feedback::*;
        pattern.insert_guess("apple", [Green, Yellow, Gray, Gray, Gray]);

        assert!(pattern.matches_word("asipm"));
        assert!(pattern.matches_word("asrip"));
        assert!(!pattern.matches_word("bspim"));
        assert!(!pattern.matches_word("asrim"));
    }
}
