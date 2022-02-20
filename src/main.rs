mod wordlists;

use std::collections::HashSet;
use std::io::Write;
use std::{env, io};

use wordle_bot::WORD_LENGTH;
use wordle_bot::{Bot, Feedback};

use wordlists::{EXTRA_GUESSING_OPTIONS, POSSIBLE_SOLUTIONS};

fn main() {
    let possible_solutions = POSSIBLE_SOLUTIONS.lines().collect::<HashSet<&str>>();
    let extra_guessing_options = EXTRA_GUESSING_OPTIONS.lines().collect::<HashSet<&str>>();

    let mut args = env::args().skip(1);
    let hard_mode = args.next() == Some("hard".to_string());

    let mut bot = Bot::new(possible_solutions, extra_guessing_options, hard_mode);

    loop {
        let matching = bot.all_matching_solutions();
        if matching.len() == 0 {
            println!("There is no word that matches the pattern :(");
        } else if matching.len() == 1 {
            println!(
                "There is only one word that matches the pattern:\n{}",
                matching[0]
            );
        }

        print!("\n[{}] > ", bot.count);
        let _ = io::stdout().flush();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Did not enter correct string");

        let mut instructions = input.split_whitespace();

        match instructions.next() {
            Some("recommend") => {
                let n: Option<usize> = match instructions.next() {
                    Some(arg) => match arg.parse() {
                        Ok(num) => Some(num),
                        Err(_) => {
                            if arg == "all" {
                                None
                            } else {
                                eprintln!("Number could not be parsed");
                                continue;
                            }
                        }
                    },
                    None => Some(10),
                };

                let recs = bot.recommend_guesses(true);
                let n = if let Some(n) = n { n } else { recs.len() };

                for (rec, entropy) in recs.into_iter().take(n) {
                    println!("{} ({:.3})", rec, entropy);
                }
                println!("-------------\n-> {}", n);
            }

            Some("matching") => {
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

            Some("mode") => match instructions.next() {
                Some("hard") => bot.hard_mode = true,
                Some("easy") => bot.hard_mode = false,
                Some("toggle") => bot.hard_mode = !bot.hard_mode,
                _ => {
                    if bot.hard_mode {
                        println!("Mode: hard");
                    } else {
                        println!("Mode: easy");
                    }
                }
            },

            Some("debug") => match instructions.next() {
                Some("pattern") => println!("{:?}", bot.pattern),
                Some("count") => println!("{:?}", bot.count),
                Some("possible_solutions") => println!("{:?}", bot.possible_solutions),
                Some("extra_guessing_options") => println!("{:?}", bot.extra_guessing_options),
                Some("mode") | Some("hard") | Some("hard_mode") => println!("{:?}", bot.hard_mode),
                obj => {
                    eprintln!("Object {:?} is not debuggable.", obj);
                    eprintln!("Try `pattern`, `count`, `possible_solutions` or `extra_guessing_options` instead.");
                }
            },

            Some("clear") => print!("{esc}[2J{esc}[1;1H", esc = 27 as char),

            Some("reset") => {
                bot = Bot::new(
                    bot.possible_solutions,
                    bot.extra_guessing_options,
                    bot.hard_mode,
                );
            }

            Some("quit") | Some("exit") => break,

            Some("help") => {
                println!("Instructions:\n");
                println!("recommend <XY|all>");
                println!("matching");
                println!("guess <WORD>");
                println!("entropy <WORD>");
                println!("mode [hard|easy|toggle]");
                println!("debug <OBJ>");
                println!("clear");
                println!("reset");
                println!("quit|exit");
                println!("help");
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
