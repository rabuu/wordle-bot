mod pattern;

use std::cmp;
use std::collections::{HashMap, HashSet};

use pattern::Pattern;

pub const WORD_LENGTH: usize = 5;

pub struct Bot<'a> {
    pub possible_solutions: HashSet<&'a str>,
    pub extra_guessing_options: HashSet<&'a str>,
    pub pattern: Pattern,
    pub max_number_guesses: usize,
    pub count: usize,
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

    fn guess_distribution(
        word: &str,
        solutions: &[&str],
    ) -> HashMap<[Feedback; WORD_LENGTH], usize> {
        let mut distribution = HashMap::new();

        for solution in solutions {
            let fb = Feedback::from_guess(word, solution);
            *distribution.entry(fb).or_insert(0) += 1;
        }

        distribution
    }

    pub fn calculate_entropy(&self, word: &str) -> f64 {
        let mut entropy: f64 = 0.0;

        let matching_solutions = self.all_matching_solutions();
        let distribution = Self::guess_distribution(word, &matching_solutions);

        for (_, v) in distribution {
            let probability: f64 = v as f64 / matching_solutions.len() as f64;
            let information = -probability.log2();
            entropy += probability * information;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Feedback {
    Gray,
    Yellow,
    Green,
}

impl Feedback {
    pub fn from_guess(word: &str, solution: &str) -> [Self; WORD_LENGTH] {
        assert_eq!(word.len(), WORD_LENGTH);
        assert_eq!(solution.len(), WORD_LENGTH);
        let n = word.len();

        use Feedback::*;

        let mut feedback = [Gray; WORD_LENGTH];
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
