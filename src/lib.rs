mod feedback;
mod pattern;

use std::cmp;
use std::collections::{HashMap, HashSet};

pub use feedback::Feedback;
use pattern::Pattern;

pub const WORD_LENGTH: usize = 5;

pub struct Bot<'a> {
    pub possible_solutions: HashSet<&'a str>,
    pub extra_guessing_options: HashSet<&'a str>,
    pub pattern: Pattern,
    pub hard_mode: bool,
    pub count: usize,
}

impl<'a> Bot<'a> {
    pub fn new(
        possible_solutions: HashSet<&'a str>,
        extra_guessing_options: HashSet<&'a str>,
        hard_mode: bool,
    ) -> Self {
        Bot {
            possible_solutions,
            extra_guessing_options,
            pattern: Pattern::default(),
            hard_mode,
            count: 1,
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

        let mut distribution = HashMap::new();
        for solution in &matching_solutions {
            let fb = Feedback::from_guess(word, solution);
            *distribution.entry(fb).or_insert(0) += 1;
        }

        for (_, v) in distribution {
            let probability: f64 = v as f64 / matching_solutions.len() as f64;
            let information = -probability.log2();
            entropy += probability * information;
        }

        entropy
    }

    pub fn recommend_guesses(&self, progress: bool) -> Vec<(&'a str, f64)> {
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
            .filter(|w| self.pattern.matches_word(w) || !self.hard_mode)
            .enumerate()
        {
            if progress {
                let percent = ((i as f32
                    / (self.possible_solutions.len() + self.extra_guessing_options.len()) as f32)
                    * 100.0) as usize;

                print!("\r{word} ({percent}%)");
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
    }
}
