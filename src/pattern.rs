use std::collections::HashSet;

use crate::WORD_LENGTH;

#[derive(Debug, Default)]
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
}

#[derive(Debug)]
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
