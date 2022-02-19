use std::collections::HashSet;
use std::io;
use std::io::Write;

use wordle_bot::WORD_LENGTH;
use wordle_bot::{Bot, Feedback};

const POSSIBLE_SOLUTIONS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    // "/wordlists/wordle-spielen.de/possible_solutions"
    "/wordlists/nytimes.com/possible_solutions"
));

const EXTRA_GUESSING_OPTIONS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    // "/wordlists/wordle-spielen.de/extra_guessing_options"
    "/wordlists/nytimes.com/extra_guessing_options"
));

fn main() {
    let possible_solutions = POSSIBLE_SOLUTIONS.lines().collect::<HashSet<&str>>();
    let extra_guessing_options = EXTRA_GUESSING_OPTIONS.lines().collect::<HashSet<&str>>();

    let mut bot = Bot::new(possible_solutions, extra_guessing_options);

    loop {
        print!("\n[{}] Input: ", bot.count);
        let _ = io::stdout().flush();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Did not enter correct string");

        let mut instructions = input.split_whitespace();

        match instructions.next() {
            Some("recommend") => {
                if let Some(n) = instructions.next() {
                    if let Ok(n) = n.parse::<usize>() {
                        for (rec, entropy) in bot.recommend_guesses(true).iter().take(n) {
                            println!("{} ({:.3})", rec, entropy);
                        }
                    } else if n == "all" {
                        let recs = bot.recommend_guesses(true);
                        for (rec, entropy) in recs.iter() {
                            println!("{} ({:.3})", rec, entropy);
                        }
                        println!("-------------\n-> {}", recs.len());
                    } else {
                        eprintln!("Not a valid number. Pass a number or `all`.");
                    }
                } else {
                    eprintln!("Please pass a number: `recommend XY`");
                }
            }

            Some("matching") => {
                let matching = bot.all_matching_solutions();
                for solution in &matching {
                    println!("{}", solution);
                }
                println!("-------------\n-> {}", matching.len());
            }

            Some("guess") => {
                if let Some(word) = instructions.next() {
                    if word.len() != WORD_LENGTH {
                        eprintln!("Length is not {}.", WORD_LENGTH);
                        continue;
                    }

                    let mut feedback = [Feedback::Gray; WORD_LENGTH];
                    for (i, fb) in feedback.iter_mut().enumerate().take(WORD_LENGTH) {
                        print!("The {}. character is (GRAY/[y]ellow/[g]reen): ", i + 1);
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

                    bot.guess(word, feedback);
                } else {
                    eprintln!("Please pass the word you want to insert: `guess WORD`");
                }
            }

            Some("entropy") => {
                if let Some(word) = instructions.next() {
                    if word.len() != WORD_LENGTH {
                        eprintln!("Length is not {}.", WORD_LENGTH);
                        continue;
                    }

                    println!("Entropy: {}", bot.calculate_entropy(word));
                }
            }

            Some("debug") => match instructions.next() {
                Some("pattern") => println!("{:?}", bot.pattern),
                Some("count") => println!("{:?}", bot.count),
                Some("possible_solutions") => println!("{:?}", bot.possible_solutions),
                Some("extra_guessing_options") => println!("{:?}", bot.extra_guessing_options),
                obj => {
                    eprintln!("Object {:?} is not debuggable.", obj);
                    eprintln!("Try `pattern`, `count`, `possible_solutions` or `extra_guessing_options` instead.");
                }
            },

            Some("clear") => print!("{esc}[2J{esc}[1;1H", esc = 27 as char),

            Some("reset") => {
                bot = Bot::new(bot.possible_solutions, bot.extra_guessing_options);
            }

            Some("quit") | Some("exit") => break,

            Some("help") => {
                println!(
                    "Possible instructions:\n\nrecommend\nguess\ndebug\nreset\nexit/quit\nhelp"
                )
            }

            Some(unknown) => {
                eprintln!(
                    "Unknown instruction: {}\nEnter `exit` or `quit` to exit or call `help`.",
                    unknown
                )
            }

            _ => eprintln!("No instruction. Enter `exit` or `quit` to exit or call `help`."),
        }
    }
}
