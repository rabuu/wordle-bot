use std::collections::HashSet;
use std::io::{self, Write};

use wordle_bot::WORD_LENGTH;
use wordle_bot::{Bot, Feedback};

const POSSIBLE_SOLUTIONS: &'static str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/wordlists/en/powerlanguage/possible_solutions"
));

const EXTRA_GUESSING_OPTIONS: &'static str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/wordlists/en/powerlanguage/extra_guessing_options"
));

fn main() {
    // let mut possible_solutions = HashSet::new();
    // possible_solutions.insert("apfel");
    // possible_solutions.insert("halle");
    // possible_solutions.insert("malle");

    // let mut extra_guessing_options = HashSet::new();
    // extra_guessing_options.insert("aeiou");

    let possible_solutions = POSSIBLE_SOLUTIONS.lines().collect();
    let extra_guessing_options = EXTRA_GUESSING_OPTIONS.lines().collect();

    let mut bot = Bot::new(possible_solutions, extra_guessing_options);

    loop {
        print!("\nInput: ");
        let _ = io::stdout().flush();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Did not enter correct string");

        let mut instructions = input.split_whitespace();

        match instructions.next() {
            Some("recommend") => {
                let recommendations = bot.recommend_guesses();
                println!("Recommendations:\n");
                for recommendation in &recommendations {
                    println!("{}", recommendation);
                }
                println!("\nNumber: {}", recommendations.len());
            }

            Some("insert_guess") => {
                if let Some(word) = instructions.next() {
                    if word.len() != WORD_LENGTH {
                        eprintln!("Length is not {}.", WORD_LENGTH);
                        continue;
                    }

                    let mut feedback = [Feedback::Gray; WORD_LENGTH];
                    for (i, fb) in feedback.iter_mut().enumerate().take(WORD_LENGTH) {
                        print!("The {}. character is (GRAY/yellow/green): ", i + 1);
                        let _ = io::stdout().flush();
                        let mut input = String::new();
                        io::stdin()
                            .read_line(&mut input)
                            .expect("Did not enter correct string");

                        match input.trim_end() {
                            "yellow" | "y" => *fb = Feedback::Yellow,
                            "green" | "g" => *fb = Feedback::Green,
                            _ => (),
                        }
                    }

                    bot.insert_guess(word, feedback);
                }
            }
            Some("debug") => match instructions.next() {
                Some("pattern") => println!("{:?}", bot.pattern),
                Some("count") => println!("{:?}", bot.count),
                Some("possible_solutions") => println!("{:?}", bot.possible_solutions),
                Some("extra_guessing_options") => println!("{:?}", bot.extra_guessing_options),
                obj => {
                    eprintln!("Object {:?} is not debugable.", obj);
                    eprintln!("Try `pattern`, `count`, `possible_solutions` or `extra_guessing_options` instead.");
                }
            },

            Some("help") => {
                println!("Possible instructions:\n\nrecommend\ninsert_guess\ndebug\nexit/quit")
            }

            Some("quit") | Some("exit") => break,
            _ => eprintln!("No instruction. Enter `exit` or `quit` to exit or call `help`."),
        }
    }
}
