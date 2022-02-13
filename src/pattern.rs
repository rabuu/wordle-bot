use std::collections::HashSet;

use crate::Feedback;
use crate::WORD_LENGTH;

#[derive(Debug, Clone, Default)]
pub struct Pattern {
    pub characters: [Character; WORD_LENGTH],
    pub excluded: Option<HashSet<char>>,
    pub required: Option<HashSet<char>>,
}

impl Pattern {
    pub fn matches_word(&self, word: &str) -> bool {
        assert_eq!(word.len(), WORD_LENGTH);

        if let Some(required) = &self.required {
            for required_char in required {
                if !word.contains(*required_char) {
                    return false;
                }
            }
        }

        for (i, character) in self.characters.iter().enumerate() {
            match character {
                Character::Known(c) => {
                    if *c != word.chars().nth(i).unwrap() {
                        return false;
                    }
                }
                Character::Excluding(local_excluded) => {
                    let empty = HashSet::new();
                    let all_excluded = local_excluded
                        .as_ref()
                        .unwrap_or(&empty)
                        .iter()
                        .chain(self.excluded.as_ref().unwrap_or(&empty).iter());
                    for excluded_char in all_excluded {
                        if *excluded_char == word.chars().nth(i).unwrap() {
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

        for (i, c) in word.chars().enumerate() {
            match feedback[i] {
                Feedback::Gray => {
                    let excluded = self.excluded.get_or_insert(HashSet::new());
                    excluded.insert(c);
                }
                Feedback::Yellow => {
                    if let Character::Excluding(char_excluding) = &mut self.characters[i] {
                        let char_excluding = char_excluding.get_or_insert(HashSet::new());
                        char_excluding.insert(c);
                    }

                    let required = self.required.get_or_insert(HashSet::new());
                    required.insert(c);
                }
                Feedback::Green => {
                    self.characters[i] = Character::Known(c);
                }
            }
        }
    }

    pub fn matching_probability<'a, I>(&self, words: I, len: usize) -> f64
    where
        I: Iterator<Item = &'a str>,
    {
        words.filter(|w| self.matches_word(w)).count() as f64 / len as f64
    }
}

#[derive(Debug, Clone)]
pub enum Character {
    Known(char),
    Excluding(Option<HashSet<char>>),
}

impl Default for Character {
    fn default() -> Self {
        Character::Excluding(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_match() {
        let pattern = Pattern {
            characters: [
                Character::Known('a'),
                Character::default(),
                Character::default(),
                Character::Known('l'),
                Character::default(),
            ],
            excluded: None,
            required: None,
        };

        assert!(pattern.matches_word("apple"));
    }

    #[test]
    fn local_excluded() {
        let pattern = Pattern {
            characters: [
                Character::Known('a'),
                Character::Excluding(Some(HashSet::from_iter(vec!['k', 'l']))),
                Character::default(),
                Character::Known('l'),
                Character::default(),
            ],
            excluded: None,
            required: None,
        };

        assert!(pattern.matches_word("apple"));
    }
}
