mod pattern;

use std::{
    cmp,
    collections::{HashMap, HashSet},
};

use combinations::Combinations;
use permutohedron::Heap;

use pattern::Pattern;

pub const WORD_LENGTH: usize = 5;
const NUMBER_FEEDBACK_VARIANTS: usize = 3_usize.pow(WORD_LENGTH as u32);

pub struct Bot<'a> {
    pub possible_solutions: HashSet<&'a str>,
    pub extra_guessing_options: HashSet<&'a str>,
    pub pattern: Pattern,
    pub max_number_guesses: usize,
    pub count: usize,
    feedback_variants: [[Feedback; WORD_LENGTH]; NUMBER_FEEDBACK_VARIANTS],
}

impl<'a> Bot<'a> {
    pub fn new(
        possible_solutions: HashSet<&'a str>,
        extra_guessing_options: HashSet<&'a str>,
        max_number_guesses: usize,
    ) -> Self {
        Bot {
            possible_solutions,
            extra_guessing_options,
            pattern: Pattern::default(),
            max_number_guesses,
            count: 1,
            feedback_variants: Feedback::all_variants(),
        }
    }

    pub fn guess(&mut self, word: &str, feedback: [Feedback; WORD_LENGTH]) {
        assert_eq!(word.len(), WORD_LENGTH);
        self.pattern.insert_guess(word, feedback);
        self.count += 1;
    }

    pub fn all_matching_solutions(&self) -> Vec<&'a str> {
        self.possible_solutions
            .iter()
            .filter_map(|&sol| {
                if self.pattern.matches_word(sol) {
                    Some(sol)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn calculate_entropy(&self, word: &str) -> f64 {
        let mut entropy: f64 = 0.0;
        let matching_solutions = self.all_matching_solutions();
        for fb_variant in &self.feedback_variants {
            let mut new_pattern = self.pattern.clone();
            new_pattern.insert_guess(word, *fb_variant);

            let probability = new_pattern.matching_probability(&matching_solutions);

            let bits = if probability > 0.0 {
                -probability.log2()
            } else {
                0.0
            };

            entropy += probability * bits;
        }
        entropy
    }

    pub fn recommend_guesses(&self, progress: bool) -> Vec<(&'a str, Option<f64>)> {
        if self.count >= self.max_number_guesses {
            return self
                .all_matching_solutions()
                .into_iter()
                .map(|word| (word, None))
                .collect();
        }

        let mut entropy_map = HashMap::with_capacity(
            self.possible_solutions.len() + self.extra_guessing_options.len(),
        );

        if progress {
            println!();
        }

        for (i, word) in self
            .possible_solutions
            .iter()
            .chain(self.extra_guessing_options.iter())
            .enumerate()
        {
            if progress {
                let percent = ((i as f32
                    / (self.possible_solutions.len() + self.extra_guessing_options.len()) as f32)
                    * 100.0) as usize;

                print!("\r{} ({}%)", word, percent);
            }

            let entropy = self.calculate_entropy(word);
            entropy_map.insert(*word, entropy);
        }

        if progress {
            print!("\r");
        }

        let mut recommendations: Vec<_> = entropy_map.into_iter().collect();

        recommendations.sort_by(|a, b| {
            a.1.partial_cmp(&b.1)
                .unwrap_or(cmp::Ordering::Equal)
                .reverse()
        });

        recommendations
            .into_iter()
            .map(|(word, entropy)| (word, Some(entropy)))
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Feedback {
    Gray,
    Yellow,
    Green,
}

impl Feedback {
    pub fn all_variants() -> [[Feedback; WORD_LENGTH]; NUMBER_FEEDBACK_VARIANTS] {
        let mut variants = Vec::with_capacity(NUMBER_FEEDBACK_VARIANTS);

        let combs = Combinations::new(
            [
                [Feedback::Gray; WORD_LENGTH],
                [Feedback::Yellow; WORD_LENGTH],
                [Feedback::Green; WORD_LENGTH],
            ]
            .concat()
            .to_vec(),
            WORD_LENGTH,
        );

        for mut comb in combs {
            let permut_heap = Heap::new(&mut comb);
            for permut in permut_heap {
                let arr: [Feedback; WORD_LENGTH] = permut.try_into().unwrap();
                if !variants.contains(&arr) {
                    variants.push(arr);
                }
            }
        }

        variants.try_into().unwrap()
    }
}
